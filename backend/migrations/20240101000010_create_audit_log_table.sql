-- Create audit_log table
CREATE TABLE audit_log (
    id UUID PRIMARY KEY,
    member_id UUID REFERENCES members(id),
    resource VARCHAR(255) NOT NULL,
    action VARCHAR(100) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX idx_audit_log_member_id ON audit_log(member_id);
CREATE INDEX idx_audit_log_timestamp ON audit_log(timestamp);
CREATE INDEX idx_audit_log_resource ON audit_log(resource);
