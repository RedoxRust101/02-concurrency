// metircs data structure
// 基本功能： inc/snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt, fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl CmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
