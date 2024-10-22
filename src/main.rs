use actix_web::{App, HttpServer, web::Data};
use actix_files::Files;

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
            .service(Files::new("/", "./static").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run();
    println!("Server Running at port 8080");
    server.await
}
