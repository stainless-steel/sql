# SQL [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a constructor of SQL statements.

## [Documentation][doc]

## Example

```rust
use sql::prelude::*;

// CREATE TABLE `users` (`id` INTEGER NOT NULL, `name` TEXT, `photo` BLOB)
println!("{}", create_table("users").column("id".integer().not_null())
                                    .column("name".string())
                                    .column("photo".binary())
                                    .compile().unwrap());

// DELETE FROM `users`
println!("{}", delete_from("users").compile().unwrap());

// INSERT INTO `users` (`id`, `name`) VALUES (?, ?), (?, ?)
println!("{}", insert_into("users").columns(&["id", "name"]).batch(2)
                                   .compile().unwrap());

// SELECT * FROM `users` WHERE `name` LIKE 'A%'
println!("{}", select_from("users").so_that(column("name").like("A%"))
                                   .compile().unwrap());

// SELECT * FROM `users` ORDER BY `name` DESC
println!("{}", select_from("users").order_by(column("name").descend())
                                   .compile().unwrap());

// SELECT `name`, `photo` FROM `users` LIMIT 1
println!("{}", select_from("users").columns(&["name", "photo"]).limit(1)
                                   .compile().unwrap());
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[doc]: https://stainless-steel.github.io/sql
[status-img]: https://travis-ci.org/stainless-steel/sql.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/sql
[version-img]: https://img.shields.io/crates/v/sql.svg
[version-url]: https://crates.io/crates/sql
