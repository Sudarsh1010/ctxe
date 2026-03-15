This is a comprehensive implementation plan for your **Structural Code RAG MCP Server**. This plan focuses on architecture, data flow, and logic separation to ensure the system is robust, maintainable, and performant.

---

### **Phase 0: Architecture & Data Model**

Before writing code, define the physical boundaries of the application.

#### **1. The Three Pillars**

* **The MCP Interface Layer:** Handles JSON-RPC, protocol compliance, and tool definitions. It acts as the "API Gateway."
* **The Indexing Engine:** A background worker responsible for file watching, parsing, and embedding. It owns the "Write" path to the database.
* **The Retrieval Engine:** Handles "Read" requests from the MCP layer, executing SQL and vector searches.

#### **2. Database Schema (SQLite)**

You will need a normalized schema to support both semantic search and graph traversal.

* **Table: `files`**
  * `id` (INTEGER PK)
  * `path` (TEXT UNIQUE)
  * `merkle_hash` (TEXT) - Hash of file content for change detection.
  * `last_indexed_at` (TIMESTAMP)
* **Table: `symbols`** (The nodes of your graph)
  * `id` (INTEGER PK)
  * `file_id` (FK to files)
  * `name` (TEXT) - e.g., `login_user`
  * `type` (TEXT) - e.g., `function`, `class`, `struct`
  * `start_line` / `end_line` (INTEGER)
  * `code_content` (TEXT) - The actual raw string of the function/class.
  * `embedding` (BLOB) - Vector stored as raw bytes (SQLite-Vec format).
* **Table: `relations`** (The edges of your graph)
  * `source_symbol_id` (FK to symbols)
  * `target_symbol_id` (FK to symbols)
  * `relation_type` (TEXT) - e.g., `CALLS`, `IMPORTS`, `EXTENDS`.

---

### **Phase 1: The Initial Scan (The "Cold Start")**

This process runs when the server starts or detects a new project root.

#### **Step 1: Workspace Discovery & Ignore Rules**

* **Action:** Scan the provided root directory.
* **Logic:** Implement logic to parse `.gitignore` and `.indexignore`. Do not traverse `node_modules`, `target`, `.git`, or `dist` folders.
* **Outcome:** A list of candidate file paths eligible for indexing.

#### **Step 2: Merkle Tree Calculation**

* **Action:** For every candidate file, read contents and compute a hash (e.g., SHA-256).
* **Optimization:** Check the `files` table in SQLite. If the stored `merkle_hash` matches the computed hash, **skip** this file. It is unchanged.
* **Outcome:** A filtered list of "dirty" (new or modified) files.

#### **Step 3: Parallel Parsing (Tree-Sitter)**

* **Action:** Spin up a thread pool (Rayon in Rust).
* **Logic:**
    1. Detect language by file extension.
    2. Load the corresponding Tree-sitter grammar.
    3. Parse the file into an Abstract Syntax Tree (AST).
    4. Execute "Queries" on the AST to extract nodes (Functions, Classes, Imports).
* **Extraction:** Extract `name`, `type`, `start_line`, `end_line`, and the raw `code_content` snippet.
* **Outcome:** A stream of `Symbol` structs in memory.

#### **Step 4: Graph Construction (Relations)**

* **Action:** Analyze the extracted symbols.
* **Logic:**
  * Scan for import statements. Link current file symbols to imported module names.
  * Scan for function calls. Create `CALLS` relations.
  * *Note:* Cross-file relations (e.g., `Function A` calls `Function B` in another file) require a two-pass approach: first index all definitions, then resolve references.

#### **Step 5: Vector Embedding (Candle)**

* **Action:** Batch the `code_content` of the extracted symbols.
* **Logic:** Pass batches through the local Candle model.
* **Optimization:** Don't embed every single line. Only embed semantic units (functions, classes, structs). Store the resulting vector (Array<f32>) temporarily.
* **Outcome:** `Symbol` structs populated with embeddings.

#### **Step 6: Database Persistence**

* **Action:** Write to SQLite within a single transaction.
* **Logic:**
    1. Delete old symbols for the modified files (Cascade delete relations).
    2. Insert new `files` entries (update `merkle_hash`).
    3. Insert new `symbols` entries.
    4. Insert new `relations` entries.
    5. Commit transaction.
* **Outcome:** The index is now synchronized with the filesystem.

---

### **Phase 2: The Runtime Watcher (Hot Reload)**

This runs in a background thread concurrently with the MCP server.

#### **Step 1: Event Listening**

* **Action:** Use a file system watcher (e.g., `notify` crate in Rust).
* **Logic:** Listen for `Create`, `Write`, and `Remove` events.
* **Debounce:** Implement a "debounce" window (e.g., 500ms). If a user saves a file 5 times in 1 second, only trigger *one* indexing job after the silence.

#### **Step 2: Change Differencing**

* **Action:** Compare the event file path against the DB.
* **Logic:**
  * If `Remove`: Delete file and associated symbols from DB.
  * If `Write`: Re-compute Merkle hash. If different, mark for re-indexing.
  * If `Create`: Mark for indexing.

#### **Step 3: Targeted Re-indexing**

* **Action:** Run the pipeline (Phase 1, Steps 3-6) *only* for the specific dirty files.
* **Logic:** This ensures the system remains responsive. You never re-scan the whole project for a single file change.

---

### **Phase 3: MCP Interface (Exposed Methods)**

These are the "Tools" the AI agent will see.

#### **Tool 1: `search_code_semantic`**

* **Input:** `query` (string), `limit` (int).
* **Process:**
    1. Embed the user's query using Candle.
    2. Execute SQL: `SELECT name, type, code_content, file_path FROM symbols ORDER BY vector_distance_cos(embedding, ?) LIMIT ?`.
    3. Format results into a clean JSON structure.
* **Output:** List of relevant code snippets with metadata.

#### **Tool 2: `find_references`**

* **Input:** `symbol_name` (string).
* **Process:**
    1. SQL Query to find the `id` of the symbol where `name = ?`.
    2. SQL Query: `SELECT * FROM relations WHERE target_symbol_id = ? AND relation_type = 'CALLS'`.
    3. Join with `symbols` table to get the source code of the caller.
* **Output:** "Here are the 5 places where `get_user()` is called."

#### **Tool 3: `get_file_structure`**

* **Input:** `file_path` (string).
* **Process:**
    1. Query `symbols` table for the specific `file_id`.
    2. Return only `name`, `type`, and `line_numbers` (omit code content).
* **Output:** An "Outline" view of the file (e.g., JSON tree of classes and methods). This allows the agent to "skim" a file without reading it.

#### **Tool 4: `get_index_status`**

* **Input:** None.
* **Output:** Returns stats like "Files Indexed: 500", "Pending Backlog: 0", "Last Updated: 10:05 AM".

---

### **Phase 4: Concurrency & Resource Management**

Since this runs locally on a user's machine, resource management is critical.

#### **1. Threading Model**

* **Main Thread (Async/Tokio):** Handles the MCP server connection. It must never block. It answers queries by reading from SQLite (which is fast).
* **Background Thread (Blocking):** Handles the file watcher and the heavy Tree-sitter/Candle processing.
* **Channel Communication:** Use a `mpsc` channel. The Watcher sends "IndexJob" messages to the Indexer. This creates a queue so the system doesn't choke if the user modifies 100 files at once.

#### **2. Memory Management**

* **Embedding Cache:** If using a small model (MiniLM), you can keep the model loaded in memory.
* **Batching:** When indexing, accumulate chunks in memory until you hit a threshold (e.g., 50 chunks) before running the embedding inference. This is much faster than running inference 50 times.

#### **3. Database Locking**

* SQLite has file locks.
  * **Readers (MCP):** Use `SQLITE_OPEN_READ_ONLY` connections or standard connections with `BEGIN IMMEDIATE` for writes to prevent "Database is locked" errors.
  * **Writers (Indexer):** Keep transactions short. Do not open a transaction, parse a file, and then sleep. Parse first, transaction last.

---

### **Phase 5: Distribution & Configuration**

#### **1. Configuration File (`.codemind.toml`)**

* Allow users to define:
  * `embedding_model`: Path to local .safetensors file.
  * `ignore_patterns`: Extra patterns beyond gitignore.
  * `max_file_size`: Skip files larger than X (to prevent memory overflow).

#### **2. Binary Structure**

* The final artifact should be a single binary (e.g., `codegraph-mcp`).
* It should act as a standard MCP "stdio" server. It reads JSON-RPC from `stdin` and writes to `stdout`.
* Error logging should go to `stderr` (or a specific log file) so it doesn't pollute the MCP protocol stream.

### **Summary of Implementation Flow**

1. **Setup Project:** Initialize Rust project, add dependencies (rmcp, tree-sitter, candle, rusqlite, notify).
2. **Build Database:** Write the schema migration logic.
3. **Build Indexer:** Implement the "Phase 1" loop (Scan -> Parse -> Embed -> Store).
4. **Build MCP Server:** Implement the protocol layer and expose the tools defined in "Phase 3".
5. **Build Watcher:** Implement the "Phase 2" file watching loop.
6. **Integration:** Connect Watcher -> Indexer -> Database -> MCP Server.
7. **Optimize:** Add batching, caching, and debounce logic.
