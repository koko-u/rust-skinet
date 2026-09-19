-- migrate:up
CREATE TABLE "products"
(
    "id"                uuid           NOT NULL DEFAULT uuidv7(),
    "name"              varchar(255)   NOT NULL,
    "description"       text           NULL,
    "price"             DECIMAL(12, 2) NOT NULL,
    "picture_url"       varchar(2048)  NULL,
    "product_type_id"   uuid           NOT NULL,
    "brand"             varchar(255)   NULL,
    "quantity_in_stock" int            NOT NULL,
    "created_at"        TIMESTAMPTZ    NOT NULL DEFAULT now(),
    "updated_at"        TIMESTAMPTZ    NOT NULL DEFAULT now(),
    CONSTRAINT "products_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "product_name_unique" UNIQUE ("name"),
    CONSTRAINT "product_type_fkey" FOREIGN KEY ("product_type_id") REFERENCES "product_types" ("id")
);

CREATE INDEX "product_name_idx_like" ON "products" USING gin ("name" gin_trgm_ops);
CREATE INDEX "product_type_idx" ON "products" ("product_type_id");

CREATE TRIGGER "product_update_at_tgr"
    BEFORE UPDATE
    ON products
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "products";
