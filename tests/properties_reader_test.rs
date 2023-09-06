#[cfg(test)]
mod properties_reader_test {
    use chiisana_url::properties_reader::*;

    #[test]
    fn red_config_from_file_test(){
        let config =  red_config_from_file("".to_string());
        assert_eq!(config.get("path_length").unwrap(), "4");
        assert_eq!(config.get("port").unwrap(), "8080");
        assert_eq!(config.get("port_redis").unwrap(), "6379");
        assert_eq!(config.get("user_redis").unwrap(), "");

        let config =  red_config_from_file(".\\application.properties".to_string());
        assert_eq!(config.get("password_redis").unwrap(), "");
    }

    #[test]
    fn initialize_config_test(){
        initialize_config("".to_string());
        assert_eq!(STATIC_CONFIG.lock().unwrap().get("path_length").unwrap(), "4");
        assert_eq!(STATIC_CONFIG.lock().unwrap().get("port").unwrap(), "8080");
        assert_eq!(STATIC_CONFIG.lock().unwrap().get("port_redis").unwrap(), "6379");
        assert_eq!(STATIC_CONFIG.lock().unwrap().get("user_redis").unwrap(), "");
    }
}