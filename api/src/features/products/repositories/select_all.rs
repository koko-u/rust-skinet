use crate::features::products::rows;

pub async fn select_all(pool: &sqlx::PgPool) -> Result<Vec<rows::ProductRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::ProductRow, "sql/products/select_all.sql")
        .fetch_all(conn.as_mut())
        .await
}
