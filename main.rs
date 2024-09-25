use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
struct Item {
  id: String,
  name: String,
  quantity: u32,
}

struct AppState {
  items: Mutex<Vec<Item>>,
}

async fn get_items(data: web::Data<AppState>) -> impl Responder {
  let items = data.items.lock().unwrap();
  HttpResponse::Ok().json(items.clone()) 
}

async fn get_item(data: web::Data<AppState>, item_id: web::Path<String>) -> impl Responder {
  let items = data.items.lock().unwrap();
  let item = items.iter().find(|i| i.id == item_id.as_str());
  match item {
      Some(item) => HttpResponse::Ok().json(item),
      None => HttpResponse::NotFound().body("Item not found"),
  }
}

#[derive(Deserialize)]
struct NewItem {
  name: String,
  quantity: u32,
}

async fn add_item(data: web::Data<AppState>, new_item: web::Json<NewItem>) -> impl Responder {
  let mut items = data.items.lock().unwrap();
  let item = Item {
      id: Uuid::new_v4().to_string(),
      name: new_item.name.clone(),
      quantity: new_item.quantity,
  };
  items.push(item.clone());
  HttpResponse::Created().json(item)
}

async fn delete_item(data: web::Data<AppState>, item_id: web::Path<String>) -> impl Responder {
  let mut items = data.items.lock().unwrap();
  if let Some(pos) = items.iter().position(|i| i.id == item_id.as_str()) {
      items.remove(pos);
      HttpResponse::Ok().body("Item deleted")
  } else {
      HttpResponse::NotFound().body("Item not found")
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let data = web::Data::new(AppState {
      items: Mutex::new(vec![]),
  });

  HttpServer::new(move || {
      App::new()
          .app_data(data.clone())
          .route("/items", web::get().to(get_items)) 
          .route("/items/{id}", web::get().to(get_item)) 
          .route("/items", web::post().to(add_item)) 
          .route("/items/{id}", web::delete().to(delete_item)) 
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
