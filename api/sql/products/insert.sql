INSERT
INTO "products" AS "P" ("name",
                        "description",
                        "price",
                        "picture_url",
                        "product_type_id",
                        "brand",
                        "quantity_in_stock")
VALUES ($1::varchar /*name*/,
        $2::varchar /*description*/,
        $3::decimal /*price*/,
        $4::varchar /*picture_url*/,
        $5::uuid /*product_type_id*/,
        $7::varchar /*brand*/,
        $8::int /*quantity_in_stock*/)
ON CONFLICT ("name") DO NOTHING
RETURNING "P"."id",
    "P"."name",
    "P"."description",
    "P"."price",
    "P"."picture_url",
    "P"."product_type_id",
    $6 AS "product_type_name!",
    "P"."brand",
    "P"."quantity_in_stock";