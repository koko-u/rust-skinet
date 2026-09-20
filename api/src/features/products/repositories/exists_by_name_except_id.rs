use crate::features::products::models;

pub async fn exists_by_name_except_id(
    pool: &sqlx::PgPool,
    id: models::ProductId,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_scalar!("sql/products/exists_by_name_except_id.sql", id.into_inner(), name)
        .fetch_one(conn.as_mut())
        .await
}
