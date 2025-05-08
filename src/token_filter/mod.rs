mod ngram;
mod pg_dict;
mod skip_non_alphanumeric;
mod stemmer;
mod stopwords;
mod synonym;

use std::sync::Arc;

use ngram::{Ngram, NgramConfig};
use pg_dict::PgDictTokenFilter;
use serde::{Deserialize, Serialize};
use skip_non_alphanumeric::SkipNonAlphanumeric;
use stemmer::{StemmerKind, StemmerTokenFilter};

pub trait TokenFilter {
    fn apply(&self, token: String) -> Vec<String>;

    fn apply_batch(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .into_iter()
            .flat_map(|token| self.apply(token))
            .collect()
    }
}
pub type TokenFilterPtr = Arc<dyn TokenFilter + Sync + Send>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum TokenFilterConfig {
    SkipNonAlphanumeric,
    Stemmer(StemmerKind),
    Stopwords(String),
    PgDict(String),
    Synonym(String),
    #[serde(rename = "ngram")]
    NGram(NgramConfig),
}

pub fn get_token_filter(config: TokenFilterConfig) -> TokenFilterPtr {
    match config {
        TokenFilterConfig::SkipNonAlphanumeric => Arc::new(SkipNonAlphanumeric),
        TokenFilterConfig::Stemmer(kind) => Arc::new(StemmerTokenFilter::new(kind)),
        TokenFilterConfig::Stopwords(name) => stopwords::get_stopwords_token_filter(&name),
        TokenFilterConfig::PgDict(name) => Arc::new(PgDictTokenFilter::new(&name)),
        TokenFilterConfig::Synonym(name) => synonym::get_synonym_token_filter(&name),
        TokenFilterConfig::NGram(config) => Arc::new(Ngram::new(config)),
    }
}
