#[cfg(test)]
pub mod support {
    use rider_config::Config;
    use std::sync::*;

    pub fn build_config() -> Arc<RwLock<Config>> {
        let mut config = Config::new();
        config.set_theme(config.editor_config().current_theme().clone());
        Arc::new(RwLock::new(config))
    }
}
