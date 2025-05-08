use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::TokenFilter;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "NgramConfig::validate_grams"))]
pub struct NgramConfig {
    #[serde(default = "NgramConfig::default_max_gram")]
    #[validate(range(min = 1, max = 255))]
    pub max_gram: usize,
    #[serde(default = "NgramConfig::default_min_gram")]
    #[validate(range(min = 1, max = 255))]
    pub min_gram: usize,
    #[serde(default = "NgramConfig::default_preserve_original")]
    pub preserve_original: bool,
}

impl NgramConfig {
    fn default_max_gram() -> usize {
        2
    }
    fn default_min_gram() -> usize {
        1
    }
    fn default_preserve_original() -> bool {
        false
    }
    fn validate_grams(&self) -> Result<(), ValidationError> {
        if self.min_gram > self.max_gram {
            return Err(ValidationError::new(
                "min_gram must be less than or equal to max_gram",
            ));
        }
        Ok(())
    }
}

pub struct Ngram {
    config: NgramConfig,
}

impl TokenFilter for Ngram {
    fn apply(&self, token: String) -> Vec<String> {
        let mut results = Vec::new();
        let len = token.len();
        for i in 0..=(len - self.config.min_gram) {
            for j in (i + self.config.min_gram)..=(i + self.config.max_gram).min(len) {
                results.push(token[i..j].to_string());
            }
        }
        if self.config.preserve_original
            && !(self.config.min_gram..=self.config.max_gram).contains(&len)
        {
            results.push(token);
        }
        results
    }
}

impl Ngram {
    pub fn new(config: NgramConfig) -> Self {
        if let Err(e) = config.validate() {
            panic!("Invalid NgramConfig: {}", e);
        }

        Ngram { config }
    }
}
