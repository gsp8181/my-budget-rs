DROP TRIGGER IF EXISTS prevent_currency_delete;

ALTER TABLE "item" RENAME TO "_item_new";

CREATE TABLE "item" (
    "id"       INTEGER,
    "oldId"    INTEGER,
    "category" TEXT NOT NULL CHECK(category IN ('bank', 'cardbalance', 'creditcard', 'cash', 'debt', 'misc', 'recurring')),
    "name"     TEXT NOT NULL,
    "day"      INTEGER,
    "amount"   TEXT NOT NULL,
    "cardid"   INTEGER,
    "dbName"   TEXT CHECK(dbName IN ('debit', 'credit')),
    PRIMARY KEY("id" AUTOINCREMENT)
);

INSERT INTO "item" ("id", "oldId", "category", "name", "day", "amount", "cardid", "dbName")
SELECT "id", "oldId", "category", "name", "day", "amount", "cardid", "dbName"
FROM "_item_new";

DROP TABLE "_item_new";

DROP TABLE IF EXISTS "currency";
