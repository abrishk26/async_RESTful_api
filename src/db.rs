use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::models::*;
use crate::StdErr;

#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn connect() -> Result<Self, StdErr> {
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn boards(&self) -> Result<Vec<Board>, StdErr> {
        let boards = sqlx::query_as::<_, Board>("SELECT * FROM boards")
            .fetch_all(&self.pool)
            .await?;

        Ok(boards)
    }

    pub async fn board_summary(&self, board_id: i64) -> Result<BoardSummary, StdErr> {
        let counts = sqlx::query_as::<_, (i64, CardStatus)>(
            "SELECT COUNT(*), status FROM cards WHERE board_id = $1 GROUP BY status"
        )
            .bind(board_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(counts.into())
    }

    pub async fn create_board(&self, new_board: NewBoard) -> Result<Board, StdErr> {
        let board = sqlx::query_as::<_, Board>(
            "INSERT INTO boards (name) VALUES ($1) RETURNING *"
        )
            .bind(&new_board.name)
            .fetch_one(&self.pool)
            .await?;

        Ok(board)
    }

    pub async fn delete_board(&self, board_id: i64) -> Result<(), StdErr> {
        sqlx::query("DELETE FROM boards WHERE id = $1")
            .bind(board_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn cards(&self, board_id: i64) -> Result<Vec<Card>, StdErr> {
        let cards = sqlx::query_as::<_, Card>("SELECT * FROM cards WHERE board_id = $1")
            .bind(board_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(cards)
    }

    pub async fn create_card(&self, new_card: NewCard) -> Result<Card, StdErr> {
        let card = sqlx::query_as::<_, Card>(
            "INSERT INTO cards (board_id, description) VALUES ($1, $2) RETURNING *"
        )
            .bind(new_card.board_id)
            .bind(&new_card.description)
            .fetch_one(&self.pool)
            .await?;

        Ok(card)
    }

    pub async fn update_card(&self, card_id: i64, status: UpdateCard) -> Result<Card, StdErr> {
       let card =  sqlx::query_as("UPDATE cards SET description = $1, status = $2 WHERE id = $3 RETURNING *")
            .bind(&status.description)
            .bind(&status.status)
            .bind(card_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(card)
    }

    pub async fn delete_card(&self, card_id: i64) -> Result<(), StdErr> {
        sqlx::query("DELETE FROM cards WHERE id = $1")
            .bind(card_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }


    
}