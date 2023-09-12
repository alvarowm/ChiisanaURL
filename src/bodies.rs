use warp::Filter;

use crate::requests::{PasswordRequest, Request};

pub fn json_body() -> impl Filter<Extract=(Request, ), Error=warp::Rejection> + Clone {
    warp::body::content_length_limit(16384).and(warp::body::json())
}

pub fn json_body_password() -> impl Filter<Extract=(PasswordRequest, ), Error=warp::Rejection> + Clone {
    warp::body::content_length_limit(16384).and(warp::body::json())
}
