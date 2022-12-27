use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use std::{fs, io::Error};
use actix_files::Files; 
use serde::{Deserialize, Serialize};
use serde_json;
use dotenv;
use reqwest;


const PORT: u32 = 8080;

struct Notification {
  token: String,
  user: String,
  message: String,
  title: String,
  url: String,
  responce: Option<reqwest::Response>
}
#[derive(Deserialize)]
struct PushoverResponce {
  status: i32
}

async fn send_message(message: String) -> i32 {
  let user_token = dotenv::var("PUSHOVER_USER").unwrap();
  let pass_token = dotenv::var("PUSHOVER_TOKEN").unwrap();
  
  let params = [
      ("token", pass_token),
      ("user", user_token),
      ("message", message),
      ("title", "Portfolio Contact Me".to_string()),
      ("priority", "0".to_string())
  ];
  let url = reqwest::Url::parse_with_params("https://api.pushover.net/1/messages.json", params);
  let client = reqwest::Client::new();
  let _resp = client.post(url.expect("wow")).send().await.unwrap();
  let result: i32 =  _resp.json::<PushoverResponce>().await.unwrap().status;
  result
}
#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
    content: String
}

#[derive(Serialize, Deserialize)]
struct PushResponce {
  push_status: String,
  internal_push_status: i32
}

#[post("/api/push")]
async fn push(form_data: web::Json<FormData>) -> impl Responder {
  let message = format!("Name: {} \nEmail: {} \nContent: \n{}", form_data.name, form_data.email, form_data.content);

  let result = send_message(message).await;
  let mut status = String::new();
  if result == 1 {
    status = "Sent the Message".to_string();
  } else {
    status = "Failed to send".to_string();
  }

  let obj = PushResponce {
    push_status: status,
    internal_push_status: result
  };
  let response = serde_json::to_string(&obj).unwrap();
  HttpResponse::Ok()
    .content_type("application/json")
    .body(response)
}

async fn single_page_app() -> impl Responder{

  let file_contents = fs::read_to_string("./public/index.html").expect("Fail to read index.html");

  HttpResponse::Ok()
  .content_type("text/html")
  .body(file_contents)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  println!("Server is Starting On {:?}", PORT);

  return HttpServer::new(move || {
    App::new()
      .route("/", web::get().to(single_page_app))
      .route("/portfolio", web::get().to(single_page_app))
      .route("/portfolio/{portfolioItem}", web::get().to(single_page_app))
      .route("/contactme", web::get().to(single_page_app))
      .service(push)
      .service(Files::new("/", "./public"))
}).workers(8)
  .bind(format!("127.0.0.1:{:?}", PORT))
  .expect("Failed to bind")
  .run()
  .await;

}
