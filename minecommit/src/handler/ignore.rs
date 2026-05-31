use anyhow::Result;

use super::Handler;
use crate::odb::{OdbReader, OdbWriter};

pub(crate) struct IgnoreHandler {
    pub(crate) ignore_patterns: Vec<String>,
}

impl Handler for IgnoreHandler {
    fn workspace(&self) -> &'static str {
        "ignore"
    }

    fn flatten(self, save: &impl OdbReader, _storage: &mut impl OdbWriter) -> Result<Vec<String>> {
        let mut processed = Vec::new();
        for pattern in &self.ignore_patterns {
            for key in save.glob(pattern)? {
                log::info!("Ignore file {key}");
                processed.push(key);
            }
        }
        Ok(processed)
    }

    fn unflatten(
        self,
        _save: &mut impl OdbWriter,
        storage: &impl OdbReader,
    ) -> Result<Vec<String>> {
        let mut processed = Vec::new();
        for pattern in &self.ignore_patterns {
            for key in storage.glob(pattern)? {
                log::info!("Ignore file {key}");
                processed.push(key);
            }
        }
        Ok(processed)
    }
}
