
extern crate clap;
extern crate discord;

mod command;

#[derive(Debug)]
pub struct Config {
    config_file: String,
    user_token: Option<String>,
    search_string: Option<String>,
    server_id: Option<u64>,
}


// use main::Config;

fn main() {
    let mut config: Config = command::get_opts();

    println!("{:?}", config);
}
