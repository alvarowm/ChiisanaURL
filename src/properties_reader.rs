use java_properties::read;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref STATIC_CONFIG: Mutex<HashMap<String, String>> = HashMap::new().into();
}

pub fn red_config_from_file(file: String) -> HashMap<String, String> {
    let config_file =  if !file.is_empty() { file } else { ".\\application.properties".to_string() };

    let file = match File::open(config_file.clone()) {
        Err(why) => panic!("There was a error opening {}: {}", config_file, why),
        Ok(file) => file,
    };

    let config_map = match read(BufReader::new(file)) {
        Err(why) => panic!(
            "There was a error opening the config file {}: {}",
            config_file, why
        ),
        Ok(file) => file,
    };

    return config_map;
}
pub fn initialize_config(file: String){
    *STATIC_CONFIG.lock().unwrap() = red_config_from_file(file).into();


}

