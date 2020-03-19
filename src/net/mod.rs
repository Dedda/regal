use askama::Template;
use rocket::Rocket;
#[cfg(test)]
use rocket::local::Client;

mod auth;
mod gallery;
mod picture;
mod web;

pub fn launch() {
    let rocket = build_rocket();
    rocket.launch();
}

fn build_rocket() -> Rocket {
    let rocket = rocket::ignite();
    let rocket = rocket.mount("/", routes![index, favicon_ico]);
    let rocket = gallery::mount(rocket);
    let rocket = picture::mount(rocket);
    let rocket = web::mount(rocket);
    rocket
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {

}

#[get("/")]
fn index() -> Index {
    Index {}
}

#[get("/favicon.ico")]
fn favicon_ico() -> Vec<u8> {
    include_bytes!("favicon.ico").to_vec()
}

#[cfg(test)]
fn test_client() -> Client {
    Client::new(build_rocket()).unwrap()
}