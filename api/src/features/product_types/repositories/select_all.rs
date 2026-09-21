use crate::features::product_types::rows;

pub async fn select_all(pool: &sqlx::PgPool) -> Result<Vec<rows::ProductTypeRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::ProductTypeRow, "sql/product_types/select_all.sql")
        .fetch_all(conn.as_mut())
        .await
}
