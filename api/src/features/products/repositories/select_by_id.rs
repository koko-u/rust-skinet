use crate::features::products::models;
use crate::features::products::rows;

pub async fn select_by_id(
    pool: &sqlx::PgPool,
    id: models::ProductId,
) -> Result<Option<rows::ProductRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::ProductRow, "sql/products/select_by_id.sql", id.into_inner())
        .fetch_optional(conn.as_mut())
        .await
}
