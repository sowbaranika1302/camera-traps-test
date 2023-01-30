use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub plugins: Plugins,
    pub publish_port: u16,
    pub subscribe_port: u16,
    pub images_dir: String,
    pub image_file_prefix: String,

}


impl Config {
    #[allow(dead_code)]
    fn new() -> Self {
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: Default::default(),
            plugins: Default::default(),
            publish_port: 5559,
            subscribe_port: 5560,
            images_dir: "~/camera-traps/images".to_string(),
            image_file_prefix: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Plugins {
    pub internal: Vec<String>,
    pub internal_actions: Vec<String>,
    pub external: Vec<ExtPluginConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ExtPluginConfig {
    pub plugin_name: String,
    pub id: String,
    pub external_port: u16,
    pub subscriptions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::config::config::Config;

    #[test]
    fn here_i_am() {
        println!("file test: main.rs");
    }

    #[test]
    fn print_config() {
        println!("{:?}", Config::new());
    }
}

