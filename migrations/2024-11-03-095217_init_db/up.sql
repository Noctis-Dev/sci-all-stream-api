-- Your SQL goes here
CREATE TABLE streams(
    uuid VARCHAR(36) PRIMARY KEY,
    access_token VARCHAR(36) NOT NULL,
    user_uuid VARCHAR(36) NOT NULL,
    created_at DATE NOT NULL,
    finalized_at DATE NULL
)