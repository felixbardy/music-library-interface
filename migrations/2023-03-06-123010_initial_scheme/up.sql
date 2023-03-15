-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE "track" (
	"local_id"	INTEGER UNIQUE,
	"title"	TEXT NOT NULL,
	"artist"	TEXT,
	"album"	TEXT,
	"album_artist"	TEXT,
	"track_number"	INTEGER,
	"genre"	TEXT,
	"composer"	TEXT,
	"length"	FLOAT NOT NULL,
	"sample_rate"	INTEGER NOT NULL,
	"codec"	TEXT NOT NULL,
	"filepath"	TEXT UNIQUE NOT NULL,
	PRIMARY KEY("local_id" AUTOINCREMENT)
);
