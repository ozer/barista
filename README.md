## Barista - Example of Hexagonal Architecture Rest API written in Rust

### Requisites
* PostgreSQL - You can spin up one using `docker-compose.yml` inside the project, check out the `.env` file for detail.
```
docker-compose up
```
* Run Migration scripts using [sqlx-cli](https://crates.io/crates/sqlx-cli)
```
DATABASE_URL=postgres://ozer:123456@localhost:5409/coffeeshop sqlx migrate run
```

### SQL Migration

* The below will generate migration file, you'll have to write your migration script
```
DATABASE_URL=postgres://ozer:123456@localhost:5409/coffeeshop sqlx migrate add migration-name
```

* And then, run your migration
```
DATABASE_URL=postgres://ozer:123456@localhost:5409/coffeeshop sqlx migrate run
```