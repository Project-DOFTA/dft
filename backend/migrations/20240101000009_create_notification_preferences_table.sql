-- Create notification_preferences table
CREATE TABLE notification_preferences (
    member_id UUID PRIMARY KEY REFERENCES members(id),
    email_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    in_app_enabled BOOLEAN NOT NULL DEFAULT TRUE
);
