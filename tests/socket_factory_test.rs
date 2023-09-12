#[cfg(test)]
mod socket_factory_test {
    use chiisana_url::socket_factory::*;

    #[test]
    fn of_test() {
        let socket = of(8080);
        assert_eq!(socket.ip().to_string(), "127.0.0.1".to_string());
        assert_eq!(socket.port().to_string(), "8080".to_string());
    }
}