use moka::future::Cache;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Database {
    storage: Cache<String, String>,
}

impl Database {
    pub fn default() -> Self {
        Self {
            storage: Cache::builder()
                .weigher(|_, v: &String| v.len() as u32)
                .max_capacity(100 * 1024 * 1024)
                .build(),
        }
    }
}
