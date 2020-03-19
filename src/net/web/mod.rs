use askama::Template;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use crate::auth::login::{LoginUser, LoginIdentifier};
use rocket::response::Redirect;
use rocket::request::{FromForm, Form};
use rocket::http::{Cookie, Cookies};

mod gallery;
mod picture;

pub fn mount(rocket: Rocket) -> Rocket {
    let rocket = rocket.mount("/web", routes![index, login_logged_in, login, login_check]);
    let rocket = gallery::mount(rocket);
    let rocket = picture::mount(rocket);
    rocket.mount("/static", StaticFiles::from("web"))
}

#[derive(Template)]
#[template(path = "web/index.html")]
struct Index;

#[get("/")]
fn index() -> Index {
    Index
}

#[derive(Template)]
#[template(path = "web/login.html")]
struct LoginPage;

#[get("/login")]
fn login_logged_in(_user: LoginUser) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank = 2)]
fn login() -> LoginPage {
    LoginPage
}

#[derive(FromForm)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/login", data = "<credentials>")]
fn login_check(credentials: Form<Credentials>, mut cookies: Cookies) -> Redirect {
    match crate::auth::login::create_session(LoginIdentifier::Username(credentials.username.clone()), &credentials.password) {
        Ok(Some(s)) => {
            cookies.add_private(Cookie::new("session", s));
            Redirect::to("/")
        },
        _ => Redirect::to("/web/login"),
    }
}