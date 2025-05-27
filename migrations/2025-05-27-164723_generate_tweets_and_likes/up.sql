-- Your SQL goes here
CREATE TABLE "tweets"(
	"id" UUID NOT NULL PRIMARY KEY,
	"created_at" TIMESTAMP NOT NULL,
	"message" TEXT NOT NULL
);

CREATE TABLE "likes"(
	"id" UUID NOT NULL PRIMARY KEY,
	"created_at" TIMESTAMP NOT NULL,
	"tweet_id" UUID NOT NULL,
	FOREIGN KEY ("tweet_id") REFERENCES "tweets"("id")
);

