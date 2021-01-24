-- Add migration script here
ALTER TABLE orders
    ADD price integer NOT NULL;