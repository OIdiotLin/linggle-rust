use unqlite::{UnQLite, KV};
use chrono::{DateTime, Utc};
use super::linggle;

lazy_static! {
    static ref token_cached: Option<linggle::CSRF> = get_csrf_token();
    static ref db: UnQLite = UnQLite::create(db_path());
}

fn db_path() -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(".linggle");
    path.to_str().unwrap().to_string()
}

pub fn get_csrf_token() -> Option<linggle::CSRF> {
    match db.kv_fetch("expires") {
        Ok(expires) => {
            let expires: DateTime<Utc> = DateTime::parse_from_rfc2822(std::str::from_utf8(&expires).unwrap()).unwrap().with_timezone(&Utc);
            if expires.timestamp_millis() < Utc::now().timestamp_millis() {   // expired!
                let csrf = linggle::get_csrf().unwrap();
                set_csrf_token(&csrf);
                return Some(csrf);
            } else {
                return Some(linggle::CSRF {
                    csrf_token: std::str::from_utf8(&db.kv_fetch("csrf_token").unwrap()).unwrap().to_string(),
                    expires,
                });
            }
        }
        Err(e) => {
            let csrf = linggle::get_csrf().unwrap();
            set_csrf_token(&csrf);
            return Some(csrf);
        }
    }

}

pub fn set_csrf_token(token: &linggle::CSRF) {
    db.kv_store("csrf_token", &token.csrf_token);
    db.kv_store("expires", &token.expires.to_rfc2822());
}

#[cfg(test)]
mod tests {
    use super::*;
}
