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
use json_patch::Patch;
use valico::json_schema;

#[get("/health")]
fn health() -> Status {
    Status::Ok
}

#[get("/<file>")]
fn read_json(file: String) -> Result<Json<Value>, Box<dyn Error>> {
    let path = content_json_file_path(&file);
    let value = read_json_file(&path)?;
    Ok(Json(value))
}

#[post("/<file>", data = "<json>", format = "json")]
fn save_json(file: String, json: Json<Value>) -> Result<(), Box<dyn Error>> {
    // TODO - return location
    // TODO - return 201

    let path = content_json_file_path(&file);
    write_json_file(&path, &json)
}

#[patch("/<file>", data = "<patch>", format = "application/json-patch+json")]
fn patch_json(file: String, patch: Json<Patch>) -> Result<(), Box<dyn Error>> {
    let path = content_json_file_path(&file);
    let mut json = read_json_file(&path)?;
    json_patch::patch(&mut json, &patch)?;
    write_json_file(&path, &json)
}

#[get("/schemas/<file>", format = "json")]
fn read_schema(file: String) -> Result<Json<Value>, Box<dyn Error>> {
    let path = static_json_file_path("schemas", &file);
    let value = read_json_file(&path)?;
    Ok(Json(value))
}

#[post("/schemas/<file>", data = "<schema>", format = "json")]
fn save_schema(file: String, schema: Json<Value>) -> Result<(), Box<dyn Error>> {
    // validate is schema
    let mut scope = json_schema::Scope::new();
    scope.compile(schema.clone(), false)
        .map_err(|e| format!("Schema error: {:?}", e))?; // TODO - pretty print

    // TODO - what about the $id
    // TODO - return location

    // save schema
    let path = static_json_file_path("schemas", &file);
    write_json_file(&path, &schema)
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![
            health,
            read_json, save_json, patch_json,
            read_schema, save_schema
        ])
}

fn main() {
    rocket().launch();
}

fn content_json_file_path(file: &str) -> PathBuf {
    static_json_file_path("content", file)
}

fn static_json_file_path(folder: &str, file: &str) -> PathBuf {
    let file = PathBuf::from(file).with_extension("json");
    Path::new("static").join(folder).join(file)
}

fn read_json_file(path: &PathBuf) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let value = serde_json::from_reader(file)?;
    Ok(value)
}

fn write_json_file(path: &PathBuf, json: &Value) -> Result<(), Box<dyn Error>> {
    let parent_directory = path.parent().unwrap();
    std::fs::create_dir_all(parent_directory)?;

    let file = File::create(path)?;
    serde_json::to_writer(file, json)?;
    Ok(())
}
