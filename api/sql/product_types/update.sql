UPDATE "product_types"
SET "name" = $2::varchar
WHERE "id" = $1::uuid
RETURNING "id", "name";