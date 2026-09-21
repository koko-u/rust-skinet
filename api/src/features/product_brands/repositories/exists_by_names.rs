use crate::features::product_brands::rows;

pub async fn exists_by_names(
    pool: &sqlx::PgPool,
    names: &[String],
) -> Result<Vec<rows::BrandNameExists>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::BrandNameExists,
        "sql/product_brands/exists_by_names.sql",
        names
    )
    .fetch_all(conn.as_mut())
    .await
}
