INSERT INTO "product_types" ("name")
VALUES ($1::varchar)
ON CONFLICT ("name") DO NOTHING
RETURNING "id", "name";