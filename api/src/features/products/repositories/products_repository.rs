use crate::features::product_types::models as pt_models;
use crate::features::products::commands;
use crate::features::products::models;
use crate::features::products::rows;

#[derive(Debug)]
pub struct ProductsRepository<'c>(&'c mut sqlx::PgConnection);

impl<'c> ProductsRepository<'c> {
    pub fn new(connection: &'c mut sqlx::PgConnection) -> Self {
        Self(connection)
    }

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

    pub async fn exists_by_name(self, name: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_file_scalar!("sql/products/exists_by_name.sql", name)
            .fetch_one(self.0.as_mut())
            .await
    }

    pub async fn insert(
        self,
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
        .fetch_optional(self.0.as_mut())
        .await
    }
}

// sqlx::PgPool -- acquire --> PoolConnection<Postgres> -- deref_mut --> &mut PgConnection
// &mut PgConnection : sqlx::Executor

// sqlx::PgPool -- begin --> Transaction<'static, Postgres> -- deref_mut --> &mut PgConnection
