use actix_web::web::{Data, Json, Path};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::dev::HttpServiceFactory;
use actix_web::HttpResponse;


use crate::StdErr;
use crate::db::Db;
use crate::models::*;

fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
}

fn to_ok(_: ()) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// board routes

async fn boards(db: Data<Db>) -> Result<Json<Vec<Board>>, InternalError<StdErr>> {
    db.boards()
        .await
        .map(Json)
        .map_err(to_internal_error)
}

async fn create_board(db: Data<Db>, board: Json<NewBoard>) -> Result<Json<Board>, InternalError<StdErr>> {
    db.create_board(board.into_inner())
        .await
        .map(Json)
        .map_err(to_internal_error)
}


async fn board_summary(db: Data<Db>, path: Path<i64>) -> Result<Json<BoardSummary>, InternalError<StdErr>> {
    let id = path.into_inner();
    db.board_summary(id)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

async fn delete_board(db: Data<Db>, path: Path<i64>) -> Result<HttpResponse, InternalError<StdErr>> {
    let id = path.into_inner();
    db.delete_board(id)
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}

// card routes

async fn cards(db: Data<Db>, path: Path<i64>) -> Result<Json<Vec<Card>>, InternalError<StdErr>> {
    let id = path.into_inner();
    db.cards(id)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

async fn create_card(db: Data<Db>, card: Json<NewCard>) -> Result<Json<Card>, InternalError<StdErr>> {
    db.create_card(card.into_inner())
        .await
        .map(Json)
        .map_err(to_internal_error)
}

async fn update_card(db: Data<Db>, path: Path<i64>, card: Json<UpdateCard>) -> Result<Json<Card>, InternalError<StdErr>> {
    let id = path.into_inner();
    db.update_card(id, card.into_inner())
        .await
        .map(Json)
        .map_err(to_internal_error)
}

async fn delete_card(db: Data<Db>, path: Path<i64>) -> Result<HttpResponse, InternalError<StdErr>> {
    let id = path.into_inner();
    db.delete_card(id)
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}


// singel public function which returns all of the API request handlers
pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/api")
        .route("/boards", actix_web::web::get().to(boards))
        .route("/boards", actix_web::web::post().to(create_board))
        .route("/boards/{id}", actix_web::web::get().to(board_summary))
        .route("/boards/{id}", actix_web::web::delete().to(delete_board))
        .route("/boards/{id}/cards", actix_web::web::get().to(cards))
        .route("/boards/{id}/cards", actix_web::web::post().to(create_card))
        .route("/cards/{id}", actix_web::web::put().to(update_card))
        .route("/cards/{id}", actix_web::web::delete().to(delete_card))
}