CREATE TABLE "item" (
	"id"	INTEGER,
	"oldId"	INTEGER,
	"category"	TEXT NOT NULL CHECK(category IN ('bank', 'cardbalance', 'creditcard', 'cash', 'debt', 'misc', 'recurring')),
	"name"	TEXT NOT NULL,
	"day"	INTEGER,
	"amount"	TEXT NOT NULL,
	"cardid"	INTEGER,
	"dbName"	INTEGER CHECK(dbname in ('debit', 'credit')),
	PRIMARY KEY("id" AUTOINCREMENT)
);