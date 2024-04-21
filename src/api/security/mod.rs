use crate::database::api_keys_sqlite::{validate_credentials, validate_and_get_permission};
pub use crate::database::api_keys_sqlite::{Permissions, UserPermission};
use actix_web::HttpRequest;
use failure::{format_err, Error};
mod token;
use token::APITokenCache;
use lazy_static::lazy_static;
use std::sync::Mutex;


lazy_static!{
    static ref TOKEN_CACHE: Mutex<APITokenCache> = Mutex::new(APITokenCache::new());
}

pub fn login_request(req: HttpRequest) -> Result<String, Error> {
    let headers = req.headers();

    let user_id = match headers.get("user_id") {
        Some(user_id) => user_id.to_str().unwrap().to_string(),
        None => return Err(format_err!("Incorrect Login"))
    };

    let api_key = match headers.get("api_key") {
        Some(api_key) => api_key.to_str().unwrap().to_string(),
        None => return Err(format_err!("Incorrect Login"))
    };

    let permission = validate_and_get_permission(&user_id, &api_key).unwrap();

    let token = TOKEN_CACHE.lock().unwrap().insert(&user_id, permission);
    Ok(token)
}

pub fn validate_with_token(req: HttpRequest, required_security: Permissions) -> Result<bool, Error> {
    let headers = req.headers();

    let user_id = match headers.get("user_id") {
        Some(user_id) => user_id.to_str().unwrap().to_string(),
        None => return Err(format_err!("Incorrect Login user id"))
    };

    let token = match headers.get("auth_token") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return Err(format_err!("Incorrect Login auth token"))
    };

    let is_valid = TOKEN_CACHE.lock().unwrap().valid_user(&user_id, &token, &required_security);
    Ok(is_valid)
}

pub fn validate_token_status(req: HttpRequest) -> Result<bool, Error> {
    let headers = req.headers();

    let user_id = match headers.get("user_id") {
        Some(user_id) => user_id.to_str().unwrap().to_string(),
        None => return Err(format_err!("Incorrect Login user id"))
    };

    let token = match headers.get("auth_token") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return Err(format_err!("Incorrect Login auth token"))
    };

    let is_valid = TOKEN_CACHE.lock().unwrap().valid_user_token(&user_id, &token);
    Ok(is_valid)
}

pub fn validate_request(req: HttpRequest, required_security: Permissions) -> bool {
    let headers = req.headers();

    // Pulls data out of header
    let api_key = match headers.get("api_key") {
        None => {
            return false;
        }
        Some(val) => val.to_str().unwrap().to_string(),
    };
    let user_id = match headers.get("user_id") {
        None => {
            return false;
        }
        Some(val) => val.to_str().unwrap().to_string(),
    };

    // Checks permissions
    if let Ok(res) = validate_credentials(&user_id, &api_key, required_security) {
        res
    } else {
        false
    }
}
