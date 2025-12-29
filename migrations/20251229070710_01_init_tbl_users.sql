-- Add migration script here
CREATE TABLE IF NOT EXISTS tbl_users
(
    id         VARCHAR(36) PRIMARY KEY,
    username   VARCHAR(20)  NOT NULL UNIQUE,
    password   VARCHAR(254) NOT NULL,
    email      VARCHAR(254),
    created_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT now()
);

-- index bổ sung (optional nhưng recommended)
CREATE INDEX IF NOT EXISTS idx_tbl_users_username
    ON tbl_users (username);