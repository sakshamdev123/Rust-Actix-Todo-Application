use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};

mod routes;
use routes::*;

mod database;
use database::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = database_connection()
        .await
        .expect("Failed to connect to database");
    println!("Database connection established");
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string()) // Default to 3000 if PORT is not set
        .parse()
        .expect("PORT must be a number");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service(home)
            .service(hello_user)
            .service(create_new_user)
            .service(create_new_todo)
            .service(get_all_todos)
            .service(update_todo_title)
            .service(update_todo_description)
            .service(mark_todo_completed)
            .service(delete_todo)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run();
    println!("Server Running at port {}", port);
    server.await
}
