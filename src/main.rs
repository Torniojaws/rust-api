#![feature(plugin)]
#![plugin(rocket_codegen)]

// Imports
extern crate rocket;
extern crate rocket_contrib;

// JSON definitions
use rocket_contrib::{Json, Value};
mod mystuff;
use mystuff::{MyStuff};

// Endpoints. The arrow -> means the return type
#[post("/")]
fn create(mystuff: Json<MyStuff>) -> Json<MyStuff> {
    mystuff
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "Hello",
        "World"
    ]))
}

#[put("/<id>", data = "<mystuff>")]
fn update(id: i32, mystuff: Json<MyStuff>) -> Json<MyStuff> {
    mystuff
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

fn main() {
    rocket::ignite()
        .mount("/mystuff", routes!(create, update, delete))
        .mount("/gimmestuff", routes!(read))
        .launch();
}