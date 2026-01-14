-- Create proposals table
CREATE TABLE proposals (
    id UUID PRIMARY KEY,
    creator_id UUID NOT NULL REFERENCES members(id),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(50) NOT NULL,
    votes_for INTEGER NOT NULL DEFAULT 0,
    votes_against INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    voting_ends_at TIMESTAMP NOT NULL
);

-- Create indexes for common queries
CREATE INDEX idx_proposals_creator_id ON proposals(creator_id);
CREATE INDEX idx_proposals_status ON proposals(status);
CREATE INDEX idx_proposals_voting_ends_at ON proposals(voting_ends_at);
