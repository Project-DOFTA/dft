-- Create notifications table
CREATE TABLE notifications (
    id UUID PRIMARY KEY,
    recipient_id UUID NOT NULL REFERENCES members(id),
    notification_type VARCHAR(100) NOT NULL,
    message TEXT NOT NULL,
    sent_at TIMESTAMP
);

-- Create indexes for common queries
CREATE INDEX idx_notifications_recipient_id ON notifications(recipient_id);
CREATE INDEX idx_notifications_sent_at ON notifications(sent_at);
