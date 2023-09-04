use crate::requests::{PasswordRequest, Request};
use crate::response::Response;
use crate::{redis_handler, url_maker, STATIC_CONFIG};
use warp::http::StatusCode;
use warp::reply;

pub async fn post_url(r: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let mut generated_url = url_maker::get_generated_url(&STATIC_CONFIG.lock().unwrap());

    let mut url_redis = redis_handler::get_value(&generated_url, &STATIC_CONFIG.lock().unwrap());
    while !url_redis.is_empty() {
        generated_url = url_maker::get_generated_url(&STATIC_CONFIG.lock().unwrap());
        url_redis = redis_handler::get_value(&generated_url, &STATIC_CONFIG.lock().unwrap());
    }

    if !r.url.contains("/"){
        r.url.to_owned().push_str("/");
    }

    redis_handler::set_value(&generated_url, &r.url, &STATIC_CONFIG.lock().unwrap());

    return Ok(reply::json(&generated_url));
}

pub async fn post_custom_url(r: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let custom_url = url_maker::get_custom_url(&STATIC_CONFIG.lock().unwrap(), &r.custom_path);

    let url_redis = redis_handler::get_value(&custom_url, &STATIC_CONFIG.lock().unwrap());

    if !url_redis.is_empty() {
        return Ok(reply::with_status(
            reply::json(&custom_url),
            StatusCode::CONFLICT,
        ));
    }

    redis_handler::set_value(&custom_url, &r.url, &STATIC_CONFIG.lock().unwrap());

    return Ok(reply::with_status(reply::json(&custom_url), StatusCode::OK));
}

pub async fn post_password_url(r: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let password = url_maker::get_random_chars(&STATIC_CONFIG.lock().unwrap());
    let mut generated_url = url_maker::get_generated_url(&STATIC_CONFIG.lock().unwrap());

    let mut url_redis = redis_handler::get_value(&generated_url, &STATIC_CONFIG.lock().unwrap());
    while !url_redis.is_empty() {
        generated_url = url_maker::get_generated_url(&STATIC_CONFIG.lock().unwrap());
        url_redis = redis_handler::get_value(&generated_url, &STATIC_CONFIG.lock().unwrap());
    }

    redis_handler::set_value(&generated_url, &password, &STATIC_CONFIG.lock().unwrap());
    redis_handler::set_value(&password, &r.url, &STATIC_CONFIG.lock().unwrap());

    let resp = Response {
        url: generated_url,
        password: password,
    };

    return Ok(reply::with_status(reply::json(&resp), StatusCode::OK));
}

pub async fn post_password_custom_url(r: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let password = url_maker::get_random_chars(&STATIC_CONFIG.lock().unwrap());

    let custom_url = url_maker::get_custom_url(&STATIC_CONFIG.lock().unwrap(), &r.custom_path);

    let url_redis = redis_handler::get_value(&custom_url, &STATIC_CONFIG.lock().unwrap());
    if !url_redis.is_empty() {
        return Ok(reply::with_status(
            reply::json(&custom_url),
            StatusCode::CONFLICT,
        ));
    }

    redis_handler::set_value(&custom_url, &password, &STATIC_CONFIG.lock().unwrap());
    redis_handler::set_value(&password, &r.url, &STATIC_CONFIG.lock().unwrap());

    let resp = Response {
        url: custom_url,
        password: password,
    };

    return Ok(reply::with_status(reply::json(&resp), StatusCode::OK));
}

pub async fn post_password_get_url_route(r: PasswordRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let url_redis = redis_handler::get_value(&r.password, &STATIC_CONFIG.lock().unwrap());

    if url_redis.is_empty(){
        return Ok(reply::with_status(
            reply::json(&""),
            StatusCode::NOT_FOUND,
        ));
    }

    return Ok(reply::with_status(reply::json(&url_redis), StatusCode::OK));
}

