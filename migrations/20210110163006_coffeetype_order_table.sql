-- Add migration script here
ALTER TABLE orders
ADD coffee_type varchar(254) NOT NULL;