#[cfg(test)]
mod url_maker_test {
    use std::collections::HashMap;

    use chiisana_url::properties_reader::*;
    use chiisana_url::redis_handler::clear_all;
    use chiisana_url::response::Response;
    use chiisana_url::url_maker::get_random_chars;

    fn initialize() {
        initialize_config("".to_string());
        clear_all(&*STATIC_CONFIG);
    }

    #[tokio::test]
    async fn post_url_test() {
        initialize();

        let mut r = HashMap::new();
        r.insert("url", "https://www.linkedin.com/in/alvarowm/");
        r.insert("custom_path", "");

        let client = reqwest::Client::new();

        let response = client.post("http://localhost:8080/")
            .json(&r)
            .send()
            .await;

        let returned_url_2 = client.post("http://localhost:8080/")
            .json(&r)
            .send()
            .await
            .expect("ERROR")
            .text()
            .await
            .unwrap()
            .replace("\"", "");

        match response {
            Ok(o) => {
                let returned_url = o
                    .text()
                    .await
                    .unwrap()
                    .replace("\"", "");
                assert_eq!(returned_url.to_string().contains("localhost"), true);
                assert_eq!(returned_url.len(), 19);
                assert_ne!(returned_url, returned_url_2);
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }

    #[tokio::test]
    async fn post_custom_url_test() {
        initialize();

        let mut r = HashMap::new();
        r.insert("url", "https://www.linkedin.com/in/alvarowm/");
        r.insert("custom_path", "test");

        let client = reqwest::Client::new();

        let response = client.post("http://localhost:8080/custom")
            .json(&r)
            .send()
            .await;

        let response2 = client.post("http://localhost:8080/custom")
            .json(&r)
            .send()
            .await;

        match response {
            Ok(o) => {
                let returned_url = o

                    .text()
                    .await
                    .unwrap().replace("\"", "");
                assert_eq!(returned_url.to_string(), "localhost:8080/test");
                assert_eq!(response2.unwrap().status(), reqwest::StatusCode::CONFLICT);
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }

    #[tokio::test]
    async fn post_password_url_test() {
        initialize();

        let mut r = HashMap::new();
        r.insert("url", "https://www.linkedin.com/in/alvarowm/");
        r.insert("custom_path", "");

        let client = reqwest::Client::new();

        let response_from_post = client.post("http://localhost:8080/password")
            .json(&r)
            .send()
            .await;

        let response_from_post2 = client.post("http://localhost:8080/password")
            .json(&r)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let response_from_post2: Response = serde_json::from_str(&*response_from_post2).unwrap();

        match response_from_post {
            Ok(o) => {
                let returned_json = o
                    .text()
                    .await
                    .unwrap();

                let r: Response = serde_json::from_str(&*returned_json).unwrap();

                assert_eq!(r.url.contains("localhost"), true);
                assert_eq!(r.password.to_string().len(), 8);
                assert_eq!(r.url.len(), 19);

                assert_ne!(r.password.to_string(), response_from_post2.password);
                assert_ne!(r.url.to_string(), response_from_post2.url);

                return;
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }

    #[tokio::test]
    async fn post_password_custom_url_test() {
        initialize();

        let mut r = HashMap::new();
        r.insert("url", "https://www.linkedin.com/in/alvarowm/");
        let binding = get_random_chars(&*STATIC_CONFIG);
        r.insert("custom_path", &*binding);

        let client = reqwest::Client::new();

        let response_from_post = client.post("http://localhost:8080/post_password_custom_url")
            .json(&r)
            .send()
            .await;

        let response_from_post_2 = client.post("http://localhost:8080/post_password_custom_url")
            .json(&r)
            .send()
            .await;

        match response_from_post {
            Ok(o) => {
                let returned_json = o
                    .text()
                    .await
                    .unwrap();

                let r: Response = serde_json::from_str(&*returned_json).unwrap();

                assert_eq!(r.url.contains("localhost"), true);
                assert_eq!(r.password.to_string().len(), 8);
                assert_eq!(response_from_post_2.unwrap().status(), reqwest::StatusCode::CONFLICT);
                return;
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }

    #[tokio::test]
    async fn post_password_get_url_route_test() {
        initialize();

        let mut r = HashMap::new();
        r.insert("url", "https://www.linkedin.com/in/alvarowm/");
        r.insert("custom_path", "teste");

        let client = reqwest::Client::new();

        let response_from_post = client.post("http://localhost:8080/post_password_custom_url")
            .json(&r)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let r_json: Response = serde_json::from_str(&*response_from_post).unwrap();

        let mut r = HashMap::new();
        r.insert("url", &r_json.url);
        r.insert("password", &r_json.password);

        let response_from_post = client.post("http://localhost:8080/code")
            .json(&r)
            .send()
            .await;

        match response_from_post {
            Ok(o) => {
                let returned_json = o
                    .text()
                    .await
                    .unwrap();

                let r: String = serde_json::from_str(&*returned_json).unwrap();
                let r = r.replace("\"", "");

                assert_eq!(r, "https://www.linkedin.com/in/alvarowm/");
                return;
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }

    #[tokio::test]
    async fn get_url_test() {
        initialize();

        let mut r = HashMap::new();
        r.insert("url", "https://www.linkedin.com/in/alvarowm/");
        r.insert("custom_path", "");

        let client = reqwest::Client::new();

        let returned_url = client.post("http://localhost:8080/")
            .json(&r)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .replace("\"", "");

        let returned_url: String = "http://".to_string() + &*returned_url.to_string();

        let original_url = reqwest::get(returned_url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .replace("\"", "");

        assert_eq!(original_url, "https://www.linkedin.com/in/alvarowm/");

        let response_from_post = client.post("http://localhost:8080/password")
            .json(&r)
            .send()
            .await;

        match response_from_post {
            Ok(o) => {
                let returned_json = o
                    .text()
                    .await
                    .unwrap();

                let r: Response = serde_json::from_str(&*returned_json).unwrap();

                let returned_url: String = "http://".to_string() + &*r.url.to_string();

                let get_response = reqwest::get(returned_url)
                    .await;

                assert_eq!(get_response.unwrap().status(), reqwest::StatusCode::FORBIDDEN);

                return;
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }
}