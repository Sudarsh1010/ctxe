use std::sync::LazyLock;

use tiktoken_rs::{CoreBPE, get_bpe_from_tokenizer, tokenizer::Tokenizer};

static TOKENIZER: LazyLock<CoreBPE> = LazyLock::new(|| {
    get_bpe_from_tokenizer(Tokenizer::Cl100kBase)
        .expect("Tokenizer initialization failed")
});

pub fn get_shared_tokenizer() -> &'static CoreBPE {
    &TOKENIZER
}
