
extern crate clap;
extern crate discord;

mod command;
mod config_file;

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
    config = config_file::read(config);

    println!("{:?}", config);
}
