use log::info;
use log4rs;

use std::env;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("log4rs logger initialized!");

    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir();
    info!("args: {:?}", args);
    info!("current dir: {:?} ", current_dir.unwrap().display());

    let key = "key";
    let value = "value";
    info!("pre-set -> key: {} value: {}", key, value);
    env::set_var(key, value);
    info!("post-set -> key: {} value: {:?}", key, env::var(key));

    for (key, value) in env::vars_os() {
        info!("{:?}: {:?}", key, value);
    }
}