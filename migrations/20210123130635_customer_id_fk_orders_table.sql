-- Add migration script here
ALTER TABLE orders
    ADD customer_id INTEGER REFERENCES customers (id);