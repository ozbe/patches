#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::Rocket;

#[get("/health")]
fn health() -> Status {
    Status::Ok
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![health])
}

fn main() {
    rocket().launch();
}
