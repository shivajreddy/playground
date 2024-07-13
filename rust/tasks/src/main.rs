#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello There"
}

#[get("/")]
fn all_tables() -> &'static str {
    "All tables here"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/all", routes![all_tables])
}
