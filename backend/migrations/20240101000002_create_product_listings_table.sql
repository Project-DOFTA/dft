-- Create product_listings table
CREATE TABLE product_listings (
    id UUID PRIMARY KEY,
    member_id UUID NOT NULL REFERENCES members(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    quantity DECIMAL(10,2) NOT NULL,
    unit_price DECIMAL(10,2) NOT NULL,
    availability VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX idx_product_listings_member_id ON product_listings(member_id);
CREATE INDEX idx_product_listings_availability ON product_listings(availability);
CREATE INDEX idx_product_listings_name ON product_listings(name);
