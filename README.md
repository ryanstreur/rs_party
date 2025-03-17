# CS 523 Project: Party Planning App

| Term       | Winter 2025                            |
| :--------- | :------------------------------------- |
| Name       | Ryan Streur                            |
| GitHub URL | https://github.com/ryanstreur/rs_party |

Project for Winter Classes 2025.

## How to Build

System Requirements: Either Docker Desktop OR

- [Rust / Cargo](https://www.rust-lang.org/tools/install)
- [Postgresql](https://www.postgresql.org/download/)
- [NodeJS](https://nodejs.org/en/download)

If using Docker, this application can be run with either `make` or `docker compose up`.

If not using Docker:

1. Update Makefile database connection variables to match local postgresql configuration
2. Run `make db_setup` to create application database.
3. Run `make server_setup` to run database migrations
4. Run `make server_run` to start server.
5. In separate terminal window, run `make client_setup` to install node modules.
6. Run `make client_run` to start client server.
7. Visit [localhost:5173](http://localhost:5173) to interact with client.
8. When finished, you can run `make clean` to drop the app database and delete the Rust target directory and the node_modules directory.

| Make Command        | Effect                                                             |
| :------------------ | :----------------------------------------------------------------- |
| `make` / `make all` | Run entire project in Docker Compose (recommended)                 |
| `make db_setup`     | Create application database with connection parameters in Makefile |
| `make db_teardown`  | Drop application database with connection parameters in Makefile   |
| `make server_setup` | Run database migrations (requires active db connection)                   |
| `make server_test`  | Run server tests (requires active db connection)                   |
| `make server_run`   | Run API server (requires active db connection)                     |
| `make client_setup` | Install node modules required to run the client application        |
| `make client_run`   | Run client application on local node server                        |

## Intent

Originally this project was intended to be a party planner application. 

## Notes on Execution

## Sources of Inspiration

- User model - [Django web framework's `django.contrib.auth` module](https://docs.djangoproject.com/en/5.1/ref/contrib/auth/)
- Password storage and authentication: [OWASP ASVS](https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/OWASP%20Application%20Security%20Verification%20Standard%204.0.3-en.pdf)

## Useful Documentation Links

Documentation is built with

- [pandoc](https://pandoc.org/)
- [kramdown-rfc](https://github.com/cabo/kramdown-rfc)
- [xml2rfc](https://github.com/ietf-tools/xml2rfc)
