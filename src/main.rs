#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
use std::env;
use std::collections::HashMap;


mod post_actions;
mod properties_reader;
mod redis_handler;
mod requests;
mod response;
mod url_maker;
mod get_actions;
mod bodies;
mod socket_factory;
mod server;

lazy_static! {
    pub static ref STATIC_CONFIG: Mutex<HashMap<String, String>> = HashMap::new().into();
}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config_file: String = String::new();

    if !args.is_empty() && args.len() >= 2{
        for i in 1..args.len() {
            if args[i-1].contains("-properties") {
                config_file = args[i].to_string()
            }
        }
    }

    *crate::STATIC_CONFIG.lock().unwrap() = properties_reader::initialize_config(config_file).into();

    let port =  crate::STATIC_CONFIG
        .lock()
        .unwrap()
        .get("port")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    server::set_and_start_server(port).await;
}
