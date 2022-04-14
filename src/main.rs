#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Is color orange?"
}
#[get("/orange")]
fn hello() -> &'static str {
    "Is orange!"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}