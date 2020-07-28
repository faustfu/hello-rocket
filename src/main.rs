#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes, Rocket};

// Rocket uses attributes, which look like function decorators
//  in other languages, to make declaring routes easy.
// methods/attributes include get, put, post, delete, head, patch, or options.
#[get("/")]
fn root() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn hi(name: String) -> String {
    format!("Hi, {}!", name)
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![root]) //with multiple routes: routes![a, b, c].
        .mount("/hi", routes![hi])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket; // include the rocket constructor
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn hi() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hi/me").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hi, me!".into()));
    }
}
