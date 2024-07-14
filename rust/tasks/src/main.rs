#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[macro_use]
extern crate rocket_contrib;
use rocket_contrib::json::JsonValue;

use rusqlite::{Connection, Result};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Names {
    names: Vec<String>,
}

#[get("/<name>/<age>")]
fn test(name: &str, age: u8) -> Json<JsonValue> {
    Json(json!({
        "name" : name,
        "age" : age
    }))
}

#[get("/")]
fn get_all_table_names() -> Result<status::Custom<Json<Names>>, status::Custom<Json<String>>> {
    println!("conn Trying ◉");
    let conn = Connection::open("tasks.db")
        .map_err(|e| status::Custom(Status::InternalServerError, Json(e.to_string())))?;
    println!("conn success ✅");

    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table';")
        .map_err(|e| status::Custom(Status::InternalServerError, Json(e.to_string())))?;
    println!("stmt success ✅");

    let names: Result<Vec<String>, _> = stmt.query_map([], |row| row.get(0)).unwrap().collect();
    let names =
        names.map_err(|e| status::Custom(Status::InternalServerError, Json(e.to_string())))?;

    let json_resp = Names { names };
    Ok(status::Custom(Status::ImATeapot, Json(json_resp)))
}

#[get("/")]
fn index() -> &'static str {
    "Hello There"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/test", routes![test])
        .mount("/tables", routes![get_all_table_names])
}
