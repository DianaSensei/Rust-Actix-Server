CREATE TABLE users
(
    id               SERIAL PRIMARY KEY,
    email            VARCHAR(200) NOT NULL,
    user_name        VARCHAR(50),
    hashed_password  VARCHAR      NOT NULL,
    first_name       VARCHAR(50),
    last_name        VARCHAR(50),
    phone_number     VARCHAR(20),
    status           VARCHAR(20)  NOT NULL,
    role             VARCHAR(50)  NOT NULL,
    created_by       VARCHAR      NOT NULL,
    created_time_utc TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by       VARCHAR      NOT NULL,
    updated_time_utc TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
)