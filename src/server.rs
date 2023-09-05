use warp::{Filter};
use warp::http::{header, Method};
use crate::{bodies, get_actions, post_actions, socket_factory};

pub async fn set_and_start_server(port:u16)
{
    let post_url_route = warp::post()
        .and(warp::path::end())
        .and(bodies::json_body())
        .and_then(post_actions::post_url);

    let post_custom_url_route = warp::post()
        .and(warp::path("custom"))
        .and(warp::path::end())
        .and(bodies::json_body())
        .and_then(post_actions::post_custom_url);

    let post_password_url_route = warp::post()
        .and(warp::path("password"))
        .and(warp::path::end())
        .and(bodies::json_body())
        .and_then(post_actions::post_password_url);

    let post_password_custom_url_route = warp::post()
        .and(warp::path("post_password_custom_url"))
        .and(warp::path::end())
        .and(bodies::json_body())
        .and_then(post_actions::post_password_custom_url);

    let post_password_get_url_route = warp::post()
        .and(warp::path("code"))
        .and(warp::path::end())
        .and(bodies::json_body_password())
        .and_then(post_actions::post_password_get_url_route);

    let get_url_route = warp::get()
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_actions::get_url);

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ALLOW, header::ACCESS_CONTROL_ALLOW_HEADERS, header::ACCESS_CONTROL_ALLOW_ORIGIN,
                            header::ACCESS_CONTROL_REQUEST_METHOD, header::ACCEPT_ENCODING, header::ACCESS_CONTROL_ALLOW_METHODS])
        .allow_any_origin();

    let routes = post_url_route
        .or(post_custom_url_route)
        .or(post_password_url_route)
        .or(post_password_custom_url_route)
        .or(post_password_get_url_route)
        .or(get_url_route).with(cors);

    let socket = socket_factory::of(port);

    warp::serve(routes).run(socket).await;
}