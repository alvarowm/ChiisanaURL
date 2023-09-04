use std::cell::OnceCell;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use std::env;

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

static STATIC_CONFIG: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let config = properties_reader::initialize_config(PROPERTIES_FILE.lock().unwrap());
    Mutex::new(config)
});

static PROPERTIES_FILE: Lazy<Mutex<OnceCell<String>>> = Lazy::new(||Mutex::from(OnceCell::new()));

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if !args.is_empty() && args.len() >= 2{
        for i in 1..args.len() {
            if args[i-1].contains("-properties") {
                PROPERTIES_FILE.lock().unwrap().set(args[i].to_string()).expect("Set the propery file with -properties FILE_PATH");
            }
        }
    } else {
        let _ = PROPERTIES_FILE.lock().unwrap().set("".to_string()).expect("Set the propery file with -properties FILE_PATH");
    }

    server::set_and_start_server().await;
}
