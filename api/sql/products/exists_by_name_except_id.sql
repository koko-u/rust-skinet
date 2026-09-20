SELECT EXISTS (SELECT 1
               FROM "products"
               WHERE "name" = $2::varchar
                 AND "id" <> $1::uuid) AS "exists!";