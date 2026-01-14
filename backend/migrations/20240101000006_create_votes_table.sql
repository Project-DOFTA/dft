-- Create votes table
CREATE TABLE votes (
    proposal_id UUID NOT NULL REFERENCES proposals(id),
    member_id UUID NOT NULL REFERENCES members(id),
    vote_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (proposal_id, member_id)
);

-- Create index for member's votes
CREATE INDEX idx_votes_member_id ON votes(member_id);
