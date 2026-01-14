-- Create ratings table
CREATE TABLE ratings (
    id UUID PRIMARY KEY,
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    rater_id UUID NOT NULL REFERENCES members(id),
    rated_id UUID NOT NULL REFERENCES members(id),
    score INTEGER NOT NULL CHECK (score >= 1 AND score <= 5),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX idx_ratings_transaction_id ON ratings(transaction_id);
CREATE INDEX idx_ratings_rater_id ON ratings(rater_id);
CREATE INDEX idx_ratings_rated_id ON ratings(rated_id);

-- Ensure one rating per transaction per rater
CREATE UNIQUE INDEX idx_ratings_transaction_rater ON ratings(transaction_id, rater_id);
