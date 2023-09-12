use std::collections::HashMap;
use std::sync::Mutex;

use redis::Commands;

fn setup_and_get_client(
    redis_endpoint: &str,
    redis_port: &str,
    redis_user: &str,
    redis_password: &str,
) -> redis::Client {
    let client;
    if !redis_user.is_empty() && !redis_password.is_empty() {
        client = redis::Client::open(
            "redis://".to_owned()
                + redis_user
                + ":"
                + redis_password
                + "@"
                + redis_endpoint
                + ":"
                + redis_port
                + "/",
        )
            .unwrap();
    } else {
        client =
            redis::Client::open("redis://".to_owned() + redis_endpoint + ":" + redis_port + "/")
                .unwrap();
    }
    return client;
}

pub fn set_value(key: &str, value: &str, config: &Mutex<HashMap<String, String>>) {
    let mut client = get_client(config);
    let _: () = client.set(key, value).unwrap();
}

pub fn get_value(key: &str, config: &Mutex<HashMap<String, String>>) -> String {
    let mut client = get_client(config);

    if client.exists(key).expect("Redis error!") {
        return client.get(key).unwrap();
    }

    return "".to_owned();
}

pub fn clear_all(config: &Mutex<HashMap<String, String>>) {
    let client = get_client(config);
    let mut con: redis::Connection = client.get_connection().unwrap();
    redis::cmd("flushdb")
        .execute(&mut con);
}

fn get_client(config: &Mutex<HashMap<String, String>>) -> redis::Client {
    let binding = config.lock().unwrap();

    let redis_endpoint: &str = binding.get("endpoint_redis").unwrap();
    let redis_port: &str = binding.get("port_redis").unwrap();
    let redis_user: &str = binding.get("user_redis").unwrap();
    let redis_password: &str = binding.get("password_redis").unwrap();

    let client = setup_and_get_client(
        redis_endpoint,
        redis_port,
        redis_user,
        redis_password,
    );

    return client;
}
