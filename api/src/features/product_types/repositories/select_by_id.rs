use crate::features::product_types::models;
use crate::features::product_types::rows;

pub async fn select_by_id(
    pool: &sqlx::PgPool,
    id: models::ProductTypeId,
) -> Result<Option<rows::ProductTypeRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::ProductTypeRow,
        "sql/product_types/select_by_id.sql",
        id.into_inner()
    )
    .fetch_optional(conn.as_mut())
    .await
}
