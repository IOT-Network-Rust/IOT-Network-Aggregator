use crate::database::api_keys::validate_credentials;
pub use crate::database::api_keys::Permissions;
use actix_web::HttpRequest;

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
