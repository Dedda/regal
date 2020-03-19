pub mod login {

    use crate::database::provider;
    use sha::utils::{Digest, DigestExt};
    use rsgen::OutputCharsType::LatinAlphabetAndNumeric;

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use crate::database::model::User;

    lazy_static! {
        static ref SESSIONS: Arc<Mutex<HashMap<String, LoginUser>>> = Arc::new(Mutex::new(HashMap::new()));
    }

    #[derive(Clone)]
    pub struct LoginUser {
        pub id: i32,
    }

    impl From<User> for LoginUser {
        fn from(u: User) -> Self {
            Self {
                id: u.id
            }
        }
    }

    pub enum LoginError {
        UserNotFound,
        Database(crate::database::Error),
    }

    impl From<crate::database::Error> for LoginError {
        fn from(e: crate::database::Error) -> Self {
            LoginError::Database(e)
        }
    }

    pub enum LoginIdentifier {
        Username(String),
        Email(String),
    }

    pub fn create_session(ident: LoginIdentifier, password: &str) -> Result<Option<String>, LoginError> {
        use LoginIdentifier::*;
        let user = match ident {
            Username(username) => provider::user::by_username(&username),
            Email(email) => provider::user::by_email(&email),
        }?;
        if let Some(user) = user {
            if user.password.eq(&hash_pw(password)) {
                let sess_id = create_session_id();
                let mut sessions = SESSIONS.lock().unwrap();
                sessions.insert(sess_id.clone(), user.into());
                Ok(Some(sess_id))
            } else {
                Ok(None)
            }
        } else {
            Err(LoginError::UserNotFound)
        }
    }

    pub fn user_for_session(session_id: &str) -> Option<LoginUser> {
        let sessions = SESSIONS.lock().unwrap();
        return sessions.get(session_id).map(|a| a.clone())
    }

    pub fn end_session(session_id: &str) {
        let mut sessions = SESSIONS.lock().unwrap();
        sessions.remove(session_id);
    }

    fn create_session_id() -> String {
        let sessions = SESSIONS.lock().unwrap();
        loop {
            let id = rsgen::gen_random_string(20, LatinAlphabetAndNumeric {
                use_upper_case: true,
                use_lower_case: true,
            });
            if sessions.get(&id).is_none() {
                return id;
            }
        }
    }

    fn hash_pw(password: &str) -> String {
        sha::sha1::Sha1::default().digest(password.as_bytes()).to_hex()
    }

}

pub mod permission {

    pub enum PermissionError {
        Database(crate::database::Error),
    }

    impl From<crate::database::Error> for PermissionError {
        fn from(e: crate::database::Error) -> Self {
            PermissionError::Database(e)
        }
    }

    pub fn can_show_pic(_user_id: &i32, _pic_id: &i32) -> Result<bool, PermissionError> {
        Ok(true)
    }
}