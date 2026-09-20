WITH "params" (
               "id",
               "name",
               "description",
               "price",
               "picture_url",
               "product_type_id",
               "brand",
               "quantity_in_unit"
    ) AS (VALUES ($1::uuid,
                  $2::varchar,
                  $3::varchar,
                  $4::decimal,
                  $5::varchar,
                  $6::uuid,
                  $7::varchar,
                  $8::int))
UPDATE "products" AS "P"
SET "name"              = "M"."name",
    "description"       = "M"."description",
    "price"             = "M"."price",
    "picture_url"       = "M"."picture_url",
    "product_type_id"   = "M"."product_type_id",
    "brand"             = "M"."brand",
    "quantity_in_stock" = "M"."quantity_in_unit"
FROM "params" AS "M"
WHERE "P"."id" = "M"."id"
RETURNING "P"."id",
    "P"."name",
    "P"."description",
    "P"."price",
    "P"."picture_url",
    "P"."product_type_id",
        (SELECT "name" FROM "product_types" WHERE "id" = "M"."product_type_id") AS "product_type_name!",
    "P"."brand",
    "P"."quantity_in_stock";


