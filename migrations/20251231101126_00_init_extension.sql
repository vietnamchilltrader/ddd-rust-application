-- Add migration script here
create extension if not exists "uuid-ossp";
create extension if not exists "pgcrypto";

CREATE OR REPLACE FUNCTION uuid_generate_v7()
    RETURNS uuid
    LANGUAGE plpgsql
    VOLATILE
AS $$
DECLARE
    ts_ms BIGINT;
    b BYTEA;
BEGIN
    -- Unix timestamp in milliseconds (48 bits)
    ts_ms := (extract(epoch FROM clock_timestamp()) * 1000)::BIGINT;

    -- 16 bytes UUID
    b := gen_random_bytes(16);

    -- Set timestamp (48 bits, big-endian)
    b := set_byte(b, 0, (ts_ms >> 40) & 255);
    b := set_byte(b, 1, (ts_ms >> 32) & 255);
    b := set_byte(b, 2, (ts_ms >> 24) & 255);
    b := set_byte(b, 3, (ts_ms >> 16) & 255);
    b := set_byte(b, 4, (ts_ms >> 8)  & 255);
    b := set_byte(b, 5, ts_ms & 255);

    -- Set version = 7 (0111)
    b := set_byte(b, 6, (get_byte(b, 6) & 15) | 112);

    -- Set variant = RFC 4122 (10xxxxxx)
    b := set_byte(b, 8, (get_byte(b, 8) & 63) | 128);

    RETURN encode(b, 'hex')::uuid;
END;
$$;

CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

