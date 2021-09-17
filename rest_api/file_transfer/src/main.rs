use actix_web::{
    http::header::{ContentDisposition, DispositionParam, DispositionType},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder
};

use std::fs::File;
use std::io::Write;
use futures::{StreamExt};
use rand::prelude::*;

async fn delete_file(
    web::Path((filename,)): web::Path<(String,)>,
    req: HttpRequest,
) -> impl Responder {
    let client_addr: String = match req.peer_addr() {
        Some(addr) => addr.to_string(),
        None => "Unknown".to_string(),
    };

    match std::fs::remove_file(&filename) {
        Ok(_) => {
            println!("{} Deleted by {}", &filename, &client_addr);
            HttpResponse::Ok().body("File Deleted")
        }
        Err(er) => {
            eprintln!("Error: {:?} by {}", er.kind(), &client_addr);
            HttpResponse::NotFound().body("File not found")
        }
    }
}

async fn download_file(
    web::Path((filename,)): web::Path<(String,)>,
    req: HttpRequest,
) -> impl Responder {
    use actix_files::NamedFile;
    let client_addr: String = match req.peer_addr() {
        Some(addr) => addr.to_string(),
        None => "Unknown".to_string(),
    };

    println!("{} is downloading {}", client_addr, &filename);
    let content_disp = ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![
            DispositionParam::Name(String::from("file")),
            DispositionParam::Filename(String::from(&filename)),
        ],
    };

    NamedFile::open(filename)
        .unwrap()
        .set_content_disposition(content_disp)
        .into_response(&req)
        .unwrap()
}

async fn upload_file(web::Path((filename,)): web::Path<(String,)>, mut payload: web::Payload) -> impl Responder {
    println!("Uploading file {}", &filename);

    let mut content = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        content.extend_from_slice(&item.unwrap());
    }

    let f = File::create(&filename);
    if f.is_err() {
        println!("Failed to create file {}", &filename);
        return HttpResponse::NotFound().body("Unable to create a file");
    }
    
    if f.unwrap().write_all(&content).is_err() {
        println!("Failed to write to file {}", &filename);
        return HttpResponse::NotFound().body("Unable to write to file");
    }

    
    println!("Uploaded file {}", &filename);
    HttpResponse::Ok().body("File uploaded")
}


async fn upload_new_file(web::Path((filename,)): web::Path<(String,)>, mut payload: web::Payload) -> impl Responder {
    println!("Uploading file {}", &filename);

    let mut content = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        content.extend_from_slice(&item.unwrap());
    }
    let mut rng = rand::thread_rng();
    let filename = format!("{}{}.txt", filename, rng.gen_range(0..999));

    let f = File::create(&filename);
    if f.is_err() {
        println!("Failed to create file {}", &filename);
        return HttpResponse::NotFound().body("Unable to create a file");
    }
    
    if f.unwrap().write_all(&content).is_err() {
        println!("Failed to write to file {}", &filename);
        return HttpResponse::NotFound().body("Unable to write to file");
    }

    
    println!("Uploaded file {}", &filename);
    HttpResponse::Ok().body(format!("File uploaded as {}", filename))
}






async fn invalid_url(web::Path((filename,)): web::Path<(String,)>) -> impl Responder {
    println!("{}", filename);
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Listening on {}", &server_address);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to(upload_file))
                    .route(web::post().to(upload_new_file))
            )
            .default_service(web::route().to(invalid_url))
    })
    .bind(server_address)?
    .run()
    .await
}
