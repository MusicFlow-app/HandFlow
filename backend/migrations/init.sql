-- Enable UUID extension for generating unique IDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create tabs table with score data and UUID-based uniqueness
CREATE TABLE IF NOT EXISTS tabs (
    id UUID PRIMARY KEY,  -- Generated from metadata
    file_size BIGINT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{
        "workTitle": "Unknown",
        "composer": "Unknown",
        "arranger": "Unknown",
        "difficulty": 2,
        "category": 2
    }'::jsonb,
    score_data JSONB NOT NULL DEFAULT '{
        "parts": []
    }'::jsonb,
    favorite_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP  -- Track when the file was last used
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_tabs_created_at ON tabs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_tabs_work_title ON tabs((metadata->>'workTitle'));
CREATE INDEX IF NOT EXISTS idx_tabs_metadata ON tabs USING gin (metadata);
CREATE INDEX IF NOT EXISTS idx_tabs_score_data ON tabs USING gin (score_data);
