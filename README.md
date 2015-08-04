# SQL [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a constructor of SQL statements.

## [Documentation][doc]

## Example

```rust
use sql::prelude::*;

// CREATE TABLE `users` (`id` INTEGER NOT NULL, `name` TEXT, `photo` BLOB)
println!("{}", create_table("users").column(column("id").integer().not_null())
                                    .column(column("name").string())
                                    .column(column("photo").binary())
                                    .compile().unwrap());

// INSERT INTO `users` (`id`, `name`) VALUES (?, ?), (?, ?)
println!("{}", insert_into("users").columns(&["id", "name"]).batch(2)
                                   .compile().unwrap());

// SELECT * FROM `users` WHERE `name` LIKE 'A%'
println!("{}", select_from("users").so_that(column("name").like("A%"))
                                   .compile().unwrap());

// SELECT `name`, `photo` FROM `users` LIMIT 1
println!("{}", select_from("users").columns(&["name", "photo"]).limit(1)
                                   .compile().unwrap());
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: http://stainless-steel.github.io/images/crates.svg
[version-url]: https://crates.io/crates/sql
[status-img]: https://travis-ci.org/stainless-steel/sql.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/sql
[doc]: https://stainless-steel.github.io/sql
