CREATE TABLE IF NOT EXISTS users
(
    id SERIAL NOT NULL PRIMARY KEY,
    name CHARACTER VARYING(255) NOT NULL,
    username CHARACTER VARYING(255) NOT NULL
)