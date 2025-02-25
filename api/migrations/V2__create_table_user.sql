CREATE TABLE "user"
(
    id         INT GENERATED BY DEFAULT AS IDENTITY,
    nickname   CITEXT                         NOT NULL,
    rating     SMALLINT                       NOT NULL,
    email      VARCHAR(255)                   NOT NULL,
    password   VARCHAR(255)                   NOT NULL,
    settings   JSONB                          NOT NULL,
    created_at TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX user_id_idx ON "user" (id);
CREATE UNIQUE INDEX user_email_idx ON "user" (email);
CREATE UNIQUE INDEX user_nickname_idx ON "user" (nickname);
