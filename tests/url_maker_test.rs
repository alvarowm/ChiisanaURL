#[cfg(test)]
mod url_maker_test {
    use chiisana_url::url_maker::*;
    use chiisana_url::properties_reader::*;

    #[test]
    fn get_custom_url_test(){
        initialize_config("".to_string());
        let custom = get_custom_url(&*STATIC_CONFIG, "custom");
        assert_eq!(custom, "localhost:8080/custom".to_string());
        let custom = get_custom_url(&*STATIC_CONFIG, "custom2");
        assert_eq!(custom, "localhost:8080/custom2".to_string());
        STATIC_CONFIG.lock().unwrap().insert("base_url".to_string(), "localhost:8080/".to_string());
        let custom = get_custom_url(&*STATIC_CONFIG, "custom");
        assert_eq!(custom, "localhost:8080/custom".to_string());

    }

    #[test]
    fn get_base_url_plus_path_test(){
        initialize_config("".to_string());
        let custom = get_base_url_plus_path(&*STATIC_CONFIG, "custom");
        assert_eq!(custom, "localhost:8080/custom".to_string());
        let custom = get_base_url_plus_path(&*STATIC_CONFIG, "custom2");
        assert_eq!(custom, "localhost:8080/custom2".to_string());
        STATIC_CONFIG.lock().unwrap().insert("base_url".to_string(), "localhost:8080/".to_string());
        let custom = get_base_url_plus_path(&*STATIC_CONFIG, "custom");
        assert_eq!(custom, "localhost:8080/custom".to_string());

    }

    #[test]
    fn get_generated_url_test(){
        initialize_config("".to_string());
        let generated = get_generated_url(&*STATIC_CONFIG);
        assert_eq!(generated.len(), 19);
        assert_eq!(generated.contains("localhost"), true);
    }

    #[test]
    fn get_random_chars_test(){
        initialize_config("".to_string());
        let random_chars_1 = get_random_chars(&*STATIC_CONFIG);
        let random_chars_2 = get_random_chars(&*STATIC_CONFIG);
        assert_ne!(random_chars_1, random_chars_2);
        assert_eq!(random_chars_1.len(), 8);
    }




}