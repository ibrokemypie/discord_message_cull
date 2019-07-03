extern crate yaml_rust;

use crate::Config;
use std::fs;
use yaml_rust::YamlLoader;


pub fn read(mut config: Config) -> Config {

    match fs::read_to_string(config.config_file.to_owned()) {
        Ok(f) => config = missing_opts(config, f),
        _ => {}
    }

    if config.search_string == None || config.server_id == None || config.user_token == None {
        panic!("missing search string, server id or user token");
    }

    return config;
}

fn missing_opts(mut config: Config, config_file: String) -> Config {
    let config_file_yaml = YamlLoader::load_from_str(&config_file).unwrap();
    if config_file_yaml.len() > 0 {
        let config_file_options = &config_file_yaml[0];

        if config.server_id == None {
            config.server_id = Some(config_file_options["server_id"].as_i64().unwrap() as u64);
        }

        if config.user_token == None {
            config.user_token = Some(config_file_options["user_token"].as_str().unwrap().to_owned());
        }
    }

    return config;
}