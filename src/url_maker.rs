use std::collections::HashMap;
use std::sync::Mutex;

pub fn get_custom_url(config: &Mutex<HashMap<String, String>>, custom_path: &str) -> String {
    let binding = config.lock().unwrap();
    let base_url = binding.get("base_url").unwrap();

    if !base_url.ends_with("/"){
        base_url.to_owned().push_str("/");
    }

    return base_url.to_owned() + custom_path;
}

pub fn get_base_url_plus_path(config: &Mutex<HashMap<String, String>>, path: &str) -> String {
    let binding = config.lock().unwrap();
    let base_url = binding.get("base_url").unwrap();

    if !base_url.ends_with("/"){
        base_url.to_owned().push_str("/");
    }

    return base_url.to_owned() + path;
}

pub fn get_generated_url(config: &Mutex<HashMap<String, String>>) -> String {
    use rand::Rng;
    let binding = config.lock().unwrap();
    let chars: &[u8] = binding.get("chars").unwrap().as_bytes();
    let url_length: usize = binding.get("path_length").unwrap().parse().unwrap();
    let mut rng = rand::thread_rng();

    let sufix: String = (0..url_length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx] as char
        })
        .collect();
    let base_url = binding.get("base_url").unwrap();

    if !base_url.ends_with("/"){
        base_url.to_owned().push_str("/");
    }

    return base_url.to_owned() + &sufix;
}

pub fn get_random_chars(config: &Mutex<HashMap<String, String>>) -> String {
    use rand::Rng;
    let binding = config.lock().unwrap();
    let chars: &[u8] = binding.get("chars").unwrap().as_bytes();
    let mut rng = rand::thread_rng();
    let size: i16 = config.lock().unwrap().get("password_size").unwrap().parse().unwrap();

    let random_chars: String = (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx] as char
        })
        .collect();
    return random_chars;
}
