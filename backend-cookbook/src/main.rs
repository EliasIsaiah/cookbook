use std::time::Duration;

use rocket::tokio::time::sleep;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("waited for {} seconds", seconds)
}

// #[get("/foo/<_>/bar")]
// fn foo_bar() -> &'static str {
//     "Foo _____ bar!"
// }

#[get("/foo/<_..>")]
fn everything() -> &'static str {
    "Hey, you're here."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![delay])
        .mount("/", routes![world])
        // .mount("/", routes![foo_bar])
        .mount("/", routes![everything])
}
