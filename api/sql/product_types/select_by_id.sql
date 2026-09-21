SELECT "id", "name"
FROM "product_types"
WHERE "id" = $1::uuid;