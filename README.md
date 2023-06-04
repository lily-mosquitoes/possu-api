# Possu (backend)

![License](https://img.shields.io/github/license/lily-mosquitoes/possu-api?color=ff69b4&style=for-the-badge)
![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/lily-mosquitoes/possu-api?include_prereleases&style=for-the-badge)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/lily-mosquitoes/possu-api/test.yaml?label=tests&style=for-the-badge)

This API allows for registering expenses
while categorizing them, for personal organization.

## Technical info

This API is made with [Rust](https://rust-lang.org) using
the web framework [Rocket](https://rocket.rs)
and the library [sqlx](https://github.com/launchbadge/sqlx)
for database access. The database used is [SQLite](https://sqlite.org).

## Todos

- [X] make OpenAPI/Insomnia documentation
- [ ] make POST to `api/entry` return 400
on wrong content-type (instead of 404)?
- [ ] implement get categories endpoint
- [ ] implement some kind of protection
- [ ] improve logging
- [ ] dockerize application
- [ ] setup CI/CD
- [ ] implement post/delete category endpoints
- [ ] implement query for categories in get entries endpoint

## Tests

You'll need [Rust and Cargo installed](https://rust-lang.org/tools/install).

Simply run `cargo test` to run all tests.
This project includes unittests and integration tests.

## How to run

You'll need [Rust and Cargo installed](https://rust-lang.org/tools/install).

Create an `.env` file by copying the provided template like so:
`cp .env.template .env`.

Then, simply run `cargo run`, and the web service should be
launched at [`http://localhost:8000`](http://localhost:8000).

## Documentation

An OpenAPI 3.0 design documentation is available and was
created using Insomnia, the file `api-documentation.yaml`
contains the API design `yaml` plus the endpoint collection
for Insomnia.
Please import the file into [Insomnia](https://insomnia.rest)
or a similar program which supports the Insomnia v4 file format.
