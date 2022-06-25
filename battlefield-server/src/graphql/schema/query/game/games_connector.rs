use juniper::FieldResult;
use sqlx::{Executor, Postgres};
use uuid::Uuid;

use super::Game;
use crate::database::{self, RawGame};
use crate::graphql::schema::connection::{
    connection, Connection, Connector, Cursor, Edge, PageInfo,
};
use crate::graphql::schema::Context;

pub struct GamesConnector<'a> {
    context: &'a Context,
}

impl<'a> GamesConnector<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

#[async_trait::async_trait]
impl Connector for GamesConnector<'_> {
    type Node = Game;

    async fn len(&self) -> FieldResult<usize> {
        let mut conn = self.context.database.acquire().await?;
        Ok(Self::count_all(&mut conn).await?)
    }

    async fn first(&self, count: usize, after: Cursor) -> FieldResult<Connection<Self::Node>> {
        let after = match after {
            Cursor::End => return Ok(Connection::empty_end()),
            Cursor::Start => None,
            Cursor::Node(value) => Some(value.parse()?),
        };
        let mut conn = self.context.database.acquire().await?;
        let games = Self::list_after(after, count as i64, &mut conn).await?;
        let page_info = Self::page_info(&games, &mut conn).await?;
        let games = games.into_iter().map(Game).map(Edge).collect::<Vec<_>>();
        Ok(Connection::new(games, page_info))
    }

    async fn last(&self, count: usize, before: Cursor) -> FieldResult<Connection<Self::Node>> {
        let before = match before {
            Cursor::Start => return Ok(Connection::empty_start()),
            Cursor::End => None,
            Cursor::Node(value) => Some(value.parse()?),
        };
        let mut conn = self.context.database.acquire().await?;
        let games = Self::list_before(before, count as i64, &mut conn).await?;
        let page_info = Self::page_info(&games, &mut conn).await?;
        let games = games.into_iter().map(Game).map(Edge).collect::<Vec<_>>();
        Ok(Connection::new(games, page_info))
    }
}

impl GamesConnector<'_> {
    async fn count_all(conn: impl Executor<'_, Database = Postgres>) -> anyhow::Result<usize> {
        Ok(sqlx::query_scalar!("SELECT count(id) FROM games")
            .fetch_one(conn)
            .await?
            .unwrap_or(0) as usize)
    }

    async fn list_after(
        cursor: Option<Uuid>,
        limit: i64,
        conn: impl Executor<'_, Database = Postgres>,
    ) -> anyhow::Result<Vec<database::Game>> {
        let query = match cursor {
            None => {
                sqlx::query_as!(
                    RawGame,
                    "SELECT * FROM games ORDER BY id ASC LIMIT $1",
                    limit
                )
                .fetch_all(conn)
                .await?
            }
            Some(cursor) => {
                sqlx::query_as!(
                    RawGame,
                    "SELECT * FROM games WHERE id > $1 ORDER BY id ASC LIMIT $2",
                    cursor,
                    limit
                )
                .fetch_all(conn)
                .await?
            }
        };
        query.into_iter().map(database::Game::try_from).collect()
    }

    async fn list_before(
        cursor: Option<Uuid>,
        limit: i64,
        conn: impl Executor<'_, Database = Postgres>,
    ) -> anyhow::Result<Vec<database::Game>> {
        let query = match cursor {
            None => {
                sqlx::query_as!(
                    RawGame,
                    "SELECT * FROM games ORDER BY id DESC LIMIT $1",
                    limit
                )
                .fetch_all(conn)
                .await?
            }
            Some(cursor) => {
                sqlx::query_as!(
                    RawGame,
                    "SELECT * FROM games WHERE id > $1 ORDER BY id DESC LIMIT $2",
                    cursor,
                    limit
                )
                .fetch_all(conn)
                .await?
            }
        };
        query
            .into_iter()
            .rev()
            .map(database::Game::try_from)
            .collect()
    }

    async fn page_info<E>(edges: &[database::Game], conn: &mut E) -> anyhow::Result<PageInfo>
    where
        for<'a> &'a mut E: Executor<'a, Database = Postgres>,
    {
        if edges.is_empty() {
            Ok(PageInfo::empty())
        } else {
            let start_cursor = edges.first().unwrap().id;
            let end_cursor = edges.first().unwrap().id;
            let has_previous_page = sqlx::query_scalar!(
                "SELECT EXISTS (SELECT 1 FROM games WHERE id < $1)",
                start_cursor
            )
            .fetch_one(&mut *conn)
            .await?
            .unwrap_or(false);
            let has_next_page = sqlx::query_scalar!(
                "SELECT EXISTS (SELECT 1 FROM games WHERE id > $1)",
                end_cursor
            )
            .fetch_one(&mut *conn)
            .await?
            .unwrap_or(false);
            Ok(PageInfo {
                has_next_page,
                has_previous_page,
                end_cursor: Cursor::Node(start_cursor.to_string()),
                start_cursor: Cursor::Node(start_cursor.to_string()),
            })
        }
    }
}

connection!(impl<'a> for GamesConnector<'a> as "GamesConnection");
