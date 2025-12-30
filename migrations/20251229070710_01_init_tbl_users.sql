-- Add migration script here
CREATE TABLE IF NOT EXISTS tbl_users
(
    id         VARCHAR(36) PRIMARY KEY,
    username   VARCHAR(20)  NOT NULL UNIQUE,
    password   VARCHAR(254) NOT NULL,
    email      VARCHAR(254),
    is_admin   BOOLEAN      NOT NULL DEFAULT FALSE,
    is_deleted BOOLEAN      NOT NULL DEFAULT FALSE,
    status     SMALLINT     NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
    created_by VARCHAR(36) DEFAULT NULL,
    updated_by VARCHAR(36) DEFAULT NULL
);

-- index bổ sung (optional nhưng recommended)
CREATE INDEX IF NOT EXISTS idx_tbl_users_username
    ON tbl_users (username);
