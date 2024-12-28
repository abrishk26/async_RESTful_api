
// This file contains the data models for the application. The models are used to represent the data in the application and are used to serialize and deserialize the data.

// For GET Requests

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub name: String,
    pub description: String,
    pub status: CardStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CardStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub in_progress: i64,
    pub done: i64,
}


// For POST Requests

#[derive(serde::Deserialize)]
pub struct NewBoard {
    pub name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCard {
    pub board_id: i64,
    pub description: String,
}


// For PATCH Requests

#[derive(serde::Deserialize)]
pub struct UpdateCard {
    pub description: String,
    pub status: CardStatus,
}