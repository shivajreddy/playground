#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello There"
}

#[get("/")]
fn say_hello() -> &'static str {
    "wow easy. woaw"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hi", routes![say_hello])
}
