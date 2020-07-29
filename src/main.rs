#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes, Rocket};

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

use rocket_contrib::serve::StaticFiles;

use rocket::request::{Form, FromForm};

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

// 1. Use query parameters with form.
#[derive(FromForm, Debug)]
struct User {
    name: String,
    account: usize,
}

#[get("/add?<id>&<user..>")]
fn user_add_form(id: usize, user: Form<User>) -> String {
    format!("Hi, user {} with {:?}!", id, user)
}

// 1. Use segment keyword and variables.
#[get("/?nonsense&<name>")]
fn query(name: String) -> String {
    format!("Hi, {}!", name)
}

// 1. Use optional variables.
#[get("/?nonsense&<name>")]
fn query_optional(name: Option<String>) -> String {
    name.map(|name| format!("Hi, {}!", name))
        .unwrap_or_else(|| "Hi!".into())
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![root])
        .mount("/hi", routes![hi])
        .mount("/download", routes![files])
        .mount("/public", StaticFiles::from("/static"))
        .mount("/user", routes![user_int, user, user_add_form])
        .mount("/query", routes![query])
        .mount("/query_optional", routes![query_optional])
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
    fn query_before() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/query/?nonsense&name=me").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hi, me!".into()));
    }

    #[test]
    fn query_after() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/query/?name=me&nonsense").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hi, me!".into()));
    }

    #[test]
    fn query_optional() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/query_optional/?nonsense").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hi!".into()));
    }

    #[test]
    fn hi() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hi/me").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hi, me!".into()));
    }

    #[test]
    fn user_id_number() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/user/123").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hi, user#123!".into()));
    }

    #[test]
    fn user_id_string() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/user/123a").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hi, user 123a!".into()));
    }

    #[test]
    fn user_add_form() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/user/add?id=100&name=sandal&account=400").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hi, user 100 with Form(User { name: \"sandal\", account: 400 })!".into()));
    }
}
