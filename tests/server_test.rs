#[cfg(test)]
mod server_test {
    use chiisana_url::properties_reader::*;
    use chiisana_url::server::*;

    #[test]
    fn get_port_test() {
        initialize_config("".to_string());
        let port = get_port();
        assert_eq!(port, 8080);
    }
}