use warp::http::StatusCode;
use crate::{redis_handler};
use warp::reply;
use crate::url_maker::get_base_url_plus_path;

pub async fn get_url(min_path : String) -> Result<impl warp::Reply, warp::Rejection> {
    let url = get_base_url_plus_path(&*crate::STATIC_CONFIG, &min_path);
    let url_redis = redis_handler::get_value(&url   , &*crate::STATIC_CONFIG);

    if url_redis.is_empty(){
        return Ok(reply::with_status(
            reply::json(&url),
            StatusCode::NOT_FOUND,
        ));
    }

    if !url_redis.contains(&"/"){
        return Ok(reply::with_status(reply::json(&"TOP SECRET"), StatusCode::FORBIDDEN));
    }

    return Ok(reply::with_status(reply::json(&url_redis), StatusCode::OK));
}