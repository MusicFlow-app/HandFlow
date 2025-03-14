-- Enable UUID extension for generating unique IDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create file_usage table with XML content and UUID-based uniqueness
CREATE TABLE IF NOT EXISTS file_usage (
    id UUID PRIMARY KEY,  -- Generated from content, used for uniqueness
    filename VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    mscx_content TEXT NOT NULL,  -- Store the XML content
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP  -- Track when the file was last used
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_file_usage_created_at ON file_usage(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_file_usage_filename ON file_usage(filename);
CREATE INDEX IF NOT EXISTS idx_file_usage_metadata ON file_usage USING gin (metadata);
