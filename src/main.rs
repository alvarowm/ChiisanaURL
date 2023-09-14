use std::env;

use chiisana_url::{param_reader, properties_reader, server};
use chiisana_url::properties_reader::STATIC_CONFIG;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = param_reader::read_params(args);
    properties_reader::initialize_config(config_file);
    println!("");
    println!("               小                          さ                       な  ");
    println!("               _____                       _____                    _____  ");
    println!("             / \\    \\                   / \\     \\                  /\\    \\");
    println!("            /:: \\____\\                 /:: \\     \\                /::\\____\\");
    println!("           /:::/    /                 /:::: \\     \\              /:::/    /");
    println!("          /:::/    /                 /:::::: \\     \\            /:::/    /");
    println!("         /:::/    /                 /:::/ \\:::\\     \\          /:::/    /");
    println!("        /:::/    /                 /:::/__ \\:::\\     \\        /:::/    /");
    println!("       /:::/    /                 /::::\\    \\:::\\     \\      /:::/    /");
    println!("      /:::/    /        _____    /::::::\\    \\:::\\     \\    /:::/    /");
    println!("     /:::/____/       / \\    \\  /:::/\\:::\\    \\:::\\_____\\  /:::/    /");
    println!("     |:::|   /       /:: \\____\\/:::/  \\:::\\    \\:::|    | /:::/____/");
    println!("     |:::|____\\     /:::/    / \\::/   |::::\\  /::: |____| \\:::\\    \\");
    println!("      \\:::\\    \\   /:::/    /   \\/____|:::::\\/::::/    /   \\:::\\    \\");
    println!("       \\:::\\    \\ /:::/    /          |::::::::::/    /     \\:::\\    \\");
    println!("        \\:::\\    /:::/    /           |::| \\::::/    /       \\:::\\    \\");
    println!("         \\:::\\__/:::/    /            |::|  \\::/____/         \\:::\\    \\");
    println!("          \\::::::::/    /             |::|   ~|                \\:::\\    \\");
    println!("           \\::::::/    /              |::|    |                 \\:::\\    \\");
    println!("            \\::::/    /               \\::|    |                  \\:::\\____\\");
    println!("             \\::/____/                 \\:|    |                   \\::/    /");
    println!("              ~~                        \\|____|                    \\/____/");
    println!("");
    println!("v{}", env!("CARGO_PKG_VERSION"));
    println!("{}", "Running on port: ".to_string() + STATIC_CONFIG.lock().unwrap().get("port").unwrap());

    server::set_and_start_server().await;
}
