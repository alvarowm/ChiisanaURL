use crate::requests::{PasswordRequest, Request};
use crate::response::Response;
use crate::{redis_handler, url_maker};
use warp::http::StatusCode;
use warp::reply;

pub async fn post_url(r: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let mut generated_url = url_maker::get_generated_url( &*crate::STATIC_CONFIG);

    let mut url_redis = redis_handler::get_value(&generated_url, &*crate::STATIC_CONFIG);
    while !url_redis.is_empty() {
        generated_url = url_maker::get_generated_url(&*crate::STATIC_CONFIG);
        url_redis = redis_handler::get_value(&generated_url, &*crate::STATIC_CONFIG);
    }

    let url = set_url_with_slash(r);

    redis_handler::set_value(&generated_url, &url, &*crate::STATIC_CONFIG);

    return Ok(reply::json(&generated_url));
}

fn set_url_with_slash(r: Request) -> String {
    let mut url: String = r.url;
    if !url.contains("/") {
        url.push_str("/");
    }
    url
}

pub async fn post_custom_url(r: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let custom_url = url_maker::get_custom_url(&*crate::STATIC_CONFIG, &r.custom_path);

    let url_redis = redis_handler::get_value(&custom_url, &*crate::STATIC_CONFIG);


    if !url_redis.is_empty() {
        return Ok(reply::with_status(
            reply::json(&custom_url),
            StatusCode::CONFLICT,
        ));
    }

    let url = set_url_with_slash(r);

    redis_handler::set_value(&custom_url, &url, &*crate::STATIC_CONFIG);

    return Ok(reply::with_status(reply::json(&custom_url), StatusCode::OK));
}

pub async fn post_password_url(r: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let password = url_maker::get_random_chars(&*crate::STATIC_CONFIG);
    let mut generated_url = url_maker::get_generated_url(&*crate::STATIC_CONFIG);

    let mut url_redis = redis_handler::get_value(&generated_url, &*crate::STATIC_CONFIG);
    while !url_redis.is_empty() {
        generated_url = url_maker::get_generated_url(&*crate::STATIC_CONFIG);
        url_redis = redis_handler::get_value(&generated_url, &*crate::STATIC_CONFIG);
    }

    let url = set_url_with_slash(r);

    redis_handler::set_value(&generated_url, &password, &*crate::STATIC_CONFIG);
    redis_handler::set_value(&password, &url, &*crate::STATIC_CONFIG);

    let resp = Response {
        url: generated_url,
        password: password,
    };

    return Ok(reply::with_status(reply::json(&resp), StatusCode::OK));
}

pub async fn post_password_custom_url(r: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let password = url_maker::get_random_chars(&*crate::STATIC_CONFIG);

    let custom_url = url_maker::get_custom_url(&*crate::STATIC_CONFIG,&r.custom_path);

    let url_redis = redis_handler::get_value(&custom_url, &*crate::STATIC_CONFIG);
    if !url_redis.is_empty() {
        return Ok(reply::with_status(
            reply::json(&custom_url),
            StatusCode::CONFLICT,
        ));
    }

    let url = set_url_with_slash(r);

    redis_handler::set_value(&custom_url, &password, &*crate::STATIC_CONFIG);
    redis_handler::set_value(&password, &url, &*crate::STATIC_CONFIG);

    let resp = Response {
        url: custom_url,
        password: password,
    };

    return Ok(reply::with_status(reply::json(&resp), StatusCode::OK));
}

pub async fn post_password_get_url_route(r: PasswordRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let url_redis = redis_handler::get_value(&r.password, &*crate::STATIC_CONFIG);

    if url_redis.is_empty(){
        return Ok(reply::with_status(
            reply::json(&""),
            StatusCode::NOT_FOUND,
        ));
    }

    return Ok(reply::with_status(reply::json(&url_redis), StatusCode::OK));
}

