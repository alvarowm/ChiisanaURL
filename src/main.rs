use std::env;

use chiisana_url::{param_reader, properties_reader, server};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let config_file = param_reader::read_params(args);

    properties_reader::initialize_config(config_file);

    server::set_and_start_server().await;
}
