#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes, Rocket};

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

use rocket_contrib::serve::StaticFiles;

// 1. Rocket uses attributes, which look like function decorators
//  in other languages, to make declaring routes easy.
// 2. Methods/attributes include get, put, post, delete, head, patch, or options.
#[get("/")]
fn root() -> &'static str {
    "Hello, world!"
}

// 1. Dynamic path with variables.
// 2. Variables could be any types that implemented trait:FromParam.
#[get("/<name>")]
fn hi(name: String) -> String {
    format!("Hi, {}!", name)
}

// 1. Use multiple segment variable to map download route into static folder.
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("download/").join(file)).ok()
}

// 1. Forward the request if types are not matched.
#[get("/<id>")]
fn user_int(id: usize) -> String {
    format!("Hi, user#{}!", id)
}

// 1. Use rank parameter to resolve path collision.
#[get("/<id>", rank = 2)]
fn user(id: String) -> String {
    format!("Hi, user {}!", id)
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![root]) //with multiple routes: routes![a, b, c].
        .mount("/hi", routes![hi])
        .mount("/download", routes![files])
        .mount("/public", StaticFiles::from("/static"))
        .mount("/user", routes![user_int, user])
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
