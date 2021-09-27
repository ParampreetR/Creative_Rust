mod db_access;

use std::sync::Mutex;
use actix_web::{ HttpServer, HttpResponse, Responder, App, web::Data, web::Path, web, HttpRequest };
use serde::{Deserialize};


struct AppState {
    db: Mutex<db_access::DbConnection>,
}


async fn homepage() -> impl Responder {
    HttpResponse::Ok().body("
Valid URLs
/persons/ids
/persons/name_by_id/{id}
/persons
/person/{name}
        ")
}


async fn get_all_persons_ids(data: Data<AppState>) -> impl Responder {
    let db = &data.db.lock().unwrap();
    db.get_all_persons_ids()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

async fn get_person_name_by_id(
    state: web::Data<AppState>,
    info: Path<String>,
) -> impl Responder {
    println!("In get_person_name_by_id");
    let id = &info;
    let id = id.parse::<u32>();
    if id.is_err() {
        return HttpResponse::NotFound().finish();
    }
    let id = id.unwrap();
    let db_conn = &state.db.lock().unwrap();
    if let Some(name) = db_conn.get_person_name_by_id(id) {
        HttpResponse::Ok().content_type("text/plain").body(name)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[derive(Deserialize)]
pub struct Filter {
    partial_name: Option<String>,
}

async fn get_persons(state: web::Data<AppState>, query: web::Query<Filter>) -> impl Responder {
    println!("In get_persons");
    let db_conn = &state.db.lock().unwrap();
    db_conn
        .get_persons_id_and_name_by_partial_name(
            &query.partial_name.clone().unwrap_or_else(|| "".to_string()),
        )
        .map(|p| p.0.to_string() + ": " + &p.1)
        .collect::<Vec<_>>()
        .join("; ")
}

async fn insert_person(state: web::Data<AppState>, name: Path<String>) -> impl Responder {
    println!("In insert_person");
    let db_conn = &mut state.db.lock().unwrap();
    format!("{}", db_conn.insert_person(&name))
}

async fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

#[actix_web::main]
async fn main() {
    println!("Starting Server....");
    let data = Data::new(AppState{ db:  Mutex::new(db_access::DbConnection::new()), });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/persons/ids").route(web::get().to(get_all_persons_ids)))
            .service(web::resource("/persons/name_by_id/{id}").route(web::get().to(get_person_name_by_id)))
            .service(web::resource("/persons").route(web::get().to(get_persons)))
            .service(web::resource("/person/{name}").route(web::get().to(insert_person)))
            .service(web::resource("/").route(web::get().to(homepage)))
            .default_service(web::route().to(invalid_resource))
    })
    .bind("localhost:8080")
    .unwrap()
    .run();

    println!("Server running on http://127.0.0.1:8080");

    server
        .await
        .unwrap();
}
