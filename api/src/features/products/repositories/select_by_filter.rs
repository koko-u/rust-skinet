use crate::features::products::commands;
use crate::features::products::rows;

pub async fn select_by_filter(
    pool: &sqlx::PgPool,
    filter: &commands::FilterProducts,
) -> Result<Vec<rows::ProductRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;

    let commands::FilterProducts { brands, types } = filter;
    let product_brand_names = brands.as_ref().map(|b| b.as_slice());
    let product_type_names = types.as_ref().map(|t| t.as_slice());

    sqlx::query_file_as!(
        rows::ProductRow,
        "sql/products/select_by_filter.sql",
        product_type_names,
        product_brand_names
    )
    .fetch_all(&mut *conn)
    .await
}
