-- Add migration script here
CREATE SEQUENCE seq_customers;

CREATE TABLE customers
(
    id INTEGER PRIMARY KEY NOT NULL DEFAULT nextval('seq_customers'),
    name varchar (254) NOT NULL
);

ALTER SEQUENCE seq_customers
    OWNED BY customers.id;