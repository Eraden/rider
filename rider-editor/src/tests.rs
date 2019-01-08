#[cfg(test)]
pub mod support {
    use rider_config::Config;
    use std::sync::*;

    pub fn build_config() -> Arc<RwLock<Config>> {
        Arc::new(RwLock::new(Config::new()))
    }
}
