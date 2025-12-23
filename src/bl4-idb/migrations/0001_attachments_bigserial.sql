-- Migration: Change attachments.id from SERIAL (int4) to BIGINT (int8)
-- This is PostgreSQL-specific; SQLite INTEGER PRIMARY KEY is already 64-bit

ALTER TABLE attachments ALTER COLUMN id TYPE BIGINT;
