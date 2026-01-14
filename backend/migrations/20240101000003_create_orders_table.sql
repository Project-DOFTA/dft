-- Create orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    buyer_id UUID NOT NULL REFERENCES members(id),
    seller_id UUID NOT NULL REFERENCES members(id),
    product_listing_id UUID NOT NULL REFERENCES product_listings(id),
    quantity DECIMAL(10,2) NOT NULL,
    total_amount DECIMAL(10,2) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX idx_orders_buyer_id ON orders(buyer_id);
CREATE INDEX idx_orders_seller_id ON orders(seller_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_product_listing_id ON orders(product_listing_id);
