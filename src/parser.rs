pub mod parser{
    use serde_derive::Deserialize;
    use std::str::FromStr;
    use device_query::{Keycode};

    #[derive(Deserialize)]
    pub struct Config {
        pub copy_mode: Option<bool>,
        pub config: Vec<Site>
    }

    #[derive(Deserialize)]
    pub struct Site {
        pub front: String,
        pub keys: String,
        pub back: Option<String>,
        pub copy_mode: Option<bool>
    }

    pub fn parse_keys(key_str: &String) -> Result<Vec<Keycode>, String>  {
        let mut key_events = Vec::new();
        for key in key_str.split("+") {
            match Keycode::from_str(key){
                Ok(k) => key_events.push(k),
                Err(e) => return Err(e),
            };
        }
        Ok(key_events)
    }

    pub fn parse_config_file(filename: &String) -> Config {
        return toml::from_str(&filename.to_string()).unwrap();
    }
}