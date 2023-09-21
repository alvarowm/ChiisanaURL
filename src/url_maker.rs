use std::collections::HashMap;
use std::sync::Mutex;

pub fn get_custom_url(config: &Mutex<HashMap<String, String>>, custom_path: &str) -> String {
    let binding = config.lock().unwrap();
    let mut base_url = binding.get("base_url").unwrap().clone();

    if !base_url.ends_with("/") {
        base_url.push_str("/");
    }

    base_url.push_str(custom_path);
    return base_url.to_owned();
}

pub fn get_base_url_plus_path(config: &Mutex<HashMap<String, String>>, path: &str) -> String {
    let binding = config.lock().unwrap();
    let mut base_url = binding.get("base_url").unwrap().clone();

    if !base_url.ends_with("/") {
        base_url.push_str("/");
    }

    base_url.push_str(path);
    return base_url.to_owned();
}

pub fn get_generated_url(config: &Mutex<HashMap<String, String>>) -> String {
    let binding = config.lock().unwrap();
    let chars: &[u8] = binding.get("chars").unwrap().as_bytes();
    let url_length: usize = binding.get("path_length").unwrap().parse().unwrap();
    let sufix: String = get_random_chars_with_len(url_length, chars);

    let mut base_url = binding.get("base_url").unwrap().clone();

    if !base_url.ends_with("/") {
        base_url.push_str("/");
    }

    base_url.push_str(&sufix);
    return base_url.to_owned();
}

pub fn get_random_chars(config: &Mutex<HashMap<String, String>>) -> String {
    let binding = config.lock().unwrap();
    let chars: &[u8] = binding.get("chars").unwrap().as_bytes();
    let size: i16 = binding.get("password_size").unwrap().parse().unwrap();

    let random_chars: String = get_random_chars_with_len(size as usize, chars);

    return random_chars;
}


fn get_random_chars_with_len (length: usize, chars: &[u8]) -> String{
    let mut rng = rand::thread_rng();
    use rand::Rng;
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx] as char
        })
        .collect()
}
