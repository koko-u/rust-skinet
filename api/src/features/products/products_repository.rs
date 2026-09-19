use into_inner::IntoInner;

use crate::features::products::models;
use crate::features::products::rows;

#[derive(Debug, derive_more::From)]
pub struct ProductsRepository<'c>(&'c mut sqlx::PgConnection);

impl<'c> ProductsRepository<'c> {
    pub async fn select_all(self) -> Result<Vec<rows::ProductRow>, sqlx::Error> {
        sqlx::query_file_as!(rows::ProductRow, "sql/products/select_all.sql")
            .fetch_all(self.0.as_mut())
            .await
    }

    pub async fn select_by_id(self, id: models::ProductId) -> Result<Option<rows::ProductRow>, sqlx::Error> {
        sqlx::query_file_as!(rows::ProductRow, "sql/products/select_by_id.sql", id.into_inner())
            .fetch_optional(self.0.as_mut())
            .await
    }
}

// sqlx::PgPool -- acquire --> PoolConnection<Postgres> -- deref_mut --> &mut PgConnection
// &mut PgConnection : sqlx::Executor

// sqlx::PgPool -- begin --> Transaction<'static, Postgres> -- deref_mut --> &mut PgConnection
