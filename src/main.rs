use actix_web::HttpResponse;

mod logger;
mod models;
mod db;
mod routes;

type StdErr = Box<dyn std::error::Error>;

async fn health_check() -> HttpResponse{
    actix_web::HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    dotenvy::dotenv()?;
    logger::init()?;

    let db = db::Db::connect().await?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .route("/health_check", actix_web::web::get().to(health_check))
            .service(routes::api())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    
    Ok(())
}
