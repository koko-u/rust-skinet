DELETE
FROM "products"
WHERE "id" = $1::uuid
RETURNING "id",
    "name",
    "description",
    "price",
    "picture_url",
    "product_type_id",
        (SELECT "name" FROM "product_types" WHERE "id" = "product_type_id") AS "product_type_name!",
    "product_brand_id",
        (SELECT "name" FROM "product_brands" WHERE "id" = "product_brand_id") AS "product_brand_name",
    "quantity_in_stock"