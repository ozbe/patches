#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde_json;

use rocket::http::Status;
use rocket::Rocket;
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::error::Error;
use serde_json::value::Value;

#[get("/health")]
fn health() -> Status {
    Status::Ok
}

#[get("/<file>")]
fn read_json(file: String) -> Result<Json<Value>, Box<dyn Error>> {
    let path = json_file_path(&file);
    let value = read_json_file(&path)?;
    Ok(Json(value))
}

#[post("/<file>", data = "<json>", format = "json")]
fn save_json(file: String, json: Json<Value>) -> Result<(), Box<dyn Error>> {
    let path = json_file_path(&file);
    write_json_file(&path, &json)
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![health, read_json, save_json])
}

fn main() {
    rocket().launch();
}

fn json_file_path(file: &String) -> PathBuf {
    let file = PathBuf::from(file).with_extension("json");
    Path::new("static/").join(file)
}

fn read_json_file(path: &PathBuf) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let value = serde_json::from_reader(file)?;
    Ok(value)
}

fn write_json_file(path: &PathBuf, json: &Value) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    serde_json::to_writer(file, json)?;
    Ok(())
}
