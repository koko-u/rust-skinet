use crate::features::product_types::models as pt_models;
use crate::features::products::commands;
use crate::features::products::rows;
use crate::shared;

pub async fn insert<'a>(
    tx: &'a mut shared::Tx<'_>,
    command: &commands::CreateProduct,
    product_type_id: pt_models::ProductTypeId,
) -> Result<Option<rows::ProductRow>, sqlx::Error> {
    let commands::CreateProduct {
        name,
        description,
        price,
        picture_url,
        product_type,
        brand,
        quantity_in_stock,
    } = command;
    let picture_url = picture_url.as_ref().map(|url| url.as_str());

    sqlx::query_file_as!(
        rows::ProductRow,
        "sql/products/insert.sql",
        name,
        description.as_ref(),
        price,
        picture_url,
        product_type_id.into_inner(),
        product_type.as_str(),
        brand.as_ref(),
        quantity_in_stock,
    )
    .fetch_optional(tx.conn())
    .await
}
