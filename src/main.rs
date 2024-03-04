use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[get("/user")]
async fn user() -> impl Responder {
    let contents: Result<String, std::io::Error> = fs::read_to_string("user.json");
    match contents {
        Ok(string) => HttpResponse::Ok().body(string),
        Err(error) => HttpResponse::NotFound().body(format!("{}", error)),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
}

#[post("/user")]
async fn user_post(user_info: web::Json<User>) ->  Result<String>  {
    let contents: Result<String, std::io::Error> = fs::read_to_string("user.json");
    let mut user_array: Vec<User> = match contents {
        Ok(content) => serde_json::from_str(&content)?,
        Err(error) => Vec::new(),
    };
    let new_user = User {
        name:user_info.name.clone()
    };
    user_array.push(new_user);

    fs::

    Ok(format!("{}","hello"))
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(submit).service(user).service(user_post))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
