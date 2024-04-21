use chrono::{DateTime, TimeDelta, Utc, Duration};
use random_string;
use super::{UserPermission, Permissions};
use hashbrown::HashMap;

const CACHE_LIFETIME: Duration = Duration::seconds(15);

fn generate_token() -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz";
    random_string::generate(15, charset)
}

fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

fn valid_time(time: &DateTime<Utc>, max_time: Duration) -> bool {
    // Get the current time
    let current_time = Utc::now();

    let within_range = time < &current_time && current_time <= *time + max_time;
    within_range
}

pub struct APITokenCache {
    cache: HashMap<String, (DateTime<Utc>, String, UserPermission)>,
}

impl APITokenCache {
    pub fn new() -> Self {
        APITokenCache {
            cache: HashMap::new(),
        }
    }

    pub fn insert(&mut self, user_id: &String, permission: UserPermission) -> String {
        let token = generate_token();
        println!("{}", token);
        self.cache
            .insert(user_id.clone(), (get_current_time(), token.clone(), permission));
        token
    }

    pub fn valid_user(&mut self, user_id: &String, token: &String, required_security: &Permissions) -> bool {
        match self.cache.get(user_id) {
            None => false,
            Some((mut time, correct_token, permission)) => {
                if correct_token != token {return false;};

                if !valid_time(&time, CACHE_LIFETIME) {return false;};
                time = Utc::now();
                match required_security {
                    Permissions::Read => permission.read_status(),
                    Permissions::Write => permission.write_status(),
                }
                
            }
        }
    }

    pub fn valid_user_token(&self, user_id: &String, token: &String) -> bool {
        match self.cache.get(user_id) {
            None => false,
            Some((mut time, correct_token, permission)) => {
                if correct_token != token {return false;};

                if !valid_time(&time, CACHE_LIFETIME) {return false;};

                true
            }
        }
    }
}

