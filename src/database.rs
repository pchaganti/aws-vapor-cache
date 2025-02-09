use moka::future::Cache;

pub type KeyType = Vec<u8>;
pub type ValueType = Vec<u8>;
#[allow(dead_code)]
#[derive(Clone)]
pub struct Database {
    pub storage: Cache<KeyType, ValueType>,
}

impl Database {
    pub fn default() -> Self {
        Self {
            storage: Cache::builder()
                .weigher(|_, v: &ValueType| v.len() as u32)
                .max_capacity(100 * 1024 * 1024)
                .build(),
        }
    }
}
