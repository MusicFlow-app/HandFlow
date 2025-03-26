-- Enable UUID extension for generating unique IDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create tabs table with XML content and UUID-based uniqueness
CREATE TABLE IF NOT EXISTS tabs (
    favorite_count INTEGER NOT NULL DEFAULT 0,
    id UUID PRIMARY KEY,  -- Generated from content, used for uniqueness
    filename VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    mscx_content TEXT NOT NULL,  -- Store the XML content
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP  -- Track when the file was last used
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_tabs_created_at ON tabs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_tabs_filename ON tabs(filename);
CREATE INDEX IF NOT EXISTS idx_tabs_metadata ON tabs USING gin (metadata);
