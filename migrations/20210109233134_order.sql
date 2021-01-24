-- Add migration script here
CREATE SEQUENCE seq_orders;

CREATE TABLE orders (
    id INT PRIMARY KEY NOT NULL DEFAULT nextval('seq_orders')
);

ALTER SEQUENCE seq_orders
    OWNED BY orders.id;