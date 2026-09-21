use crate::features::product_types::rows;

pub async fn exists_by_names(
    pool: &sqlx::PgPool,
    names: &[String],
) -> Result<Vec<rows::TypeNameExists>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::TypeNameExists,
        "sql/product_types/exists_by_names.sql",
        names
    )
    .fetch_all(conn.as_mut())
    .await
}
