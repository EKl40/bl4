-- Migration: Add unique constraint on attachments to prevent duplicates
-- Allows re-pushing without creating duplicate entries

CREATE UNIQUE INDEX IF NOT EXISTS idx_attachments_unique ON attachments(item_serial, name, view);
