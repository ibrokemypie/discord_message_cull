
extern crate clap;

use crate::Config;
use clap::{crate_version, App, Arg};
pub fn get_opts() -> Config {
    let matches = App::new("discord message cull")
                          .version(crate_version!())
                          .author("ibrokemypie")
                          .about("Searches for and deletes all messages containing a given string on a given discord server")
                          .arg(Arg::with_name("config")
                            .short("c").value_name("FILE")
                            .help("Sets a custom config file")
                            .takes_value(true))
                        .arg(Arg::with_name("token")
                            .short("t").value_name("TOKEN")
                            .help("Sets the user token")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("server")
                            .short("s").value_name("SERVERID")
                            .help("Sets the server ID")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::from_usage("<STRING> 'String to search for'")
                        .required(true))
                          .get_matches();


    let mut config = Config {
        config_file: matches
            .value_of("config")
            .unwrap_or("config.yml")
            .to_owned(),
        user_token: match matches.value_of("token") {
            Some(x) => Some(x.to_owned()),
            _ => None,
        },
        search_string: match matches.value_of("STRING") {
            Some(x) => Some(x.to_owned()),
            _ => None,
        },
        server_id: match matches.value_of("server") {
            Some(x) => match x.trim().parse::<u64>() {
                Ok(x) => Some(x),
                _ => None,
            },
            _ => None,
        },
    };

    return config;
}