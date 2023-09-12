#[cfg(test)]
mod param_reader_test {
    use chiisana_url::param_reader;

    #[test]
    fn read_params_test() {
        let mut args: Vec<String> = Vec::new();
        args.insert(0, "-properties".to_string());
        args.insert(1, "teste".to_string());
        args.insert(2, "".to_string());
        args.insert(3, "".to_string());
        let config_file: String = param_reader::read_params(args);
        assert_eq!(config_file, "teste".to_string());

        let mut args: Vec<String> = Vec::new();
        args.insert(0, "-properties".to_string());
        let config_file: String = param_reader::read_params(args);
        assert_eq!(config_file, "".to_string());

        let args: Vec<String> = Vec::new();
        let config_file: String = param_reader::read_params(args);
        assert_eq!(config_file, "".to_string());

        let mut args: Vec<String> = Vec::new();
        args.insert(0, "-another_args".to_string());
        args.insert(1, "teste".to_string());
        args.insert(2, "".to_string());
        args.insert(3, "".to_string());
        let config_file: String = param_reader::read_params(args);
        assert_eq!(config_file, "".to_string());
    }
}