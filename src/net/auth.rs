use rocket::request::{FromRequest, Outcome};
use crate::auth::login::LoginUser;
use askama::rocket::Request;

impl<'a, 'r> FromRequest<'a, 'r> for LoginUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        if let Some(session_id) = request.cookies().get_private("session") {
            if let Some(user) = crate::auth::login::user_for_session(session_id.value()) {
                Outcome::Success(user)
            } else {
                Outcome::Forward(())
            }
        } else {
            Outcome::Forward(())
        }
    }
}