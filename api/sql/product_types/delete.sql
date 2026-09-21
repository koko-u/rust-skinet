DELETE
FROM "product_types"
WHERE "id" = $1::uuid
RETURNING "id", "name";