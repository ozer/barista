## Barista - Example of Hexagonal Architecture Rest API written in Rust

### Useful Articles
* [Hexagonal Architecture: three principles and an implementation example](https://blog.octo.com/en/hexagonal-architecture-three-principles-and-an-implementation-example/)
* [Hexagonal Architecture with Java and Spring](https://reflectoring.io/spring-hexagonal/)

### Stack
* [Tide](https://github.com/http-rs/tide)
* [SQLx](https://github.com/launchbadge/sqlx)

### Road Map
* [X] Mocking Ports with [mockall](https://github.com/asomers/mockall) and testing `order_coffee_service`  
* [X] Writing a Macro(`barista_response!`) for Response Structs in Rest Adapter
* [ ] Refactor Exception Handler in Rest Adapter
* [ ] Testing in Persistence Adapter
* [ ] Integration Test for Rest Adapter
* [ ] Dockerize & Deployment via GitHub Actions

### Prerequisite
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