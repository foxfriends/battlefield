ALTER TABLE games ADD COLUMN scenario JSONB NOT NULL DEFAULT '{}'::JSONB;
