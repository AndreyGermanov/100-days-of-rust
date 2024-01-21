use std::collections::HashMap;
use std::sync::{Mutex};
use rand::Rng;
use rocket::{response::content, form::Form, State};
use rocket::http::Status;
use rocket::response::Redirect;

#[macro_use] extern crate rocket;

struct AppState {
    database: Mutex<HashMap<String, String>>
}

#[derive(FromForm)]
struct URLS {
    long_url: String,
    short_url: Option<String>,
}

#[rocket::main]
async fn main() {
    rocket::build()
        .manage(AppState {
            database: Mutex::new(HashMap::new()),
        })
        .mount("/", routes![index])
        .mount("/generate", routes![generate])
        .mount("/f", routes![forward])
        .launch().await.unwrap();
}

#[get("/")]
fn index() -> content::RawHtml<String> {
    content::RawHtml(std::fs::read_to_string("index.html").unwrap())
}

#[post("/", data="<urls>")]
fn generate(urls: Option<Form<URLS>>, state: &State<AppState>) -> Result<String,Status> {
    let urls = urls.unwrap();
    let long_url = urls.long_url.clone();
    if long_url.len() > 200 {
        return Err(Status::PayloadTooLarge);
    }
    if !long_url.to_uppercase().starts_with("HTTP://") &&
        !long_url.to_uppercase().starts_with("HTTPS://") {
            return Err(Status::BadRequest);
    }
    if state.database.lock().unwrap().iter().find(|(_,value)| *value == &long_url) != None {
        return Err(Status::Conflict);
    }
    let mut short_url = urls.short_url.clone().unwrap_or("".to_string());
    if short_url.len() == 0 {
        short_url = generate_short_for_url(&state.database.lock().unwrap(), &urls.long_url);
    } else {
        if state.database.lock().unwrap().contains_key(&short_url) {
            return Err(Status::Conflict);
        }
    };
    state.database.lock().unwrap().insert(short_url.clone(), long_url);
    return Ok(format!("http://localhost:8080/f/{}", short_url));
}

#[get("/<code>")]
fn forward(code: &str, state: &State<AppState>) -> Result<Redirect,Status> {
    let url = get_url_by_short(&state.database.lock().unwrap(), code);
    if url.len() > 0 {
        return Ok(Redirect::to(url));
    } else {
        return Err(Status::NotFound);
    }
}

fn get_url_by_short(database: &HashMap<String, String>, short: &str) -> String {
    database.get(short).unwrap_or(&"".to_string()).clone()
}

fn generate_short_for_url(database: &HashMap<String, String>, url: &str) -> String {
    let mut code = String::new();
    for _ in 0..8 {
        let num = rand::thread_rng().gen_range(65..123);
        if num >= 91 && num <=96 {
            return generate_short_for_url(database, url);
        }
        let ch = std::char::from_u32(num).unwrap();
        code = code + ch.to_string().as_str();

    }
    if get_url_by_short(database, url) != "" {
        return generate_short_for_url(database, url)
    }
    return code;
}
