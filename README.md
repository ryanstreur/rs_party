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

## Intent and Execution

Originally this project was intended to be a party planner application. Below is a list of the original features I wanted to implement, with the current status.

- [x] API Server Process                                          
- [x] API Client can create user account                          
- [x] API Client can log in (password)                            
- [x] API Client can log out                                      
- [x] API Client can make authenticated requests (Bearer JWT)     
- [x] Authenticated Client can create events                      
- [x] Owners and Organizers can list their events                 
- [x] Owners and Organizers can delete their events               
- [ ] Owners and Organizers can update their events               
- [ ] Owners and Organizers can invite other users to events      
- [ ] Owners and Organizers can assign user roles on their events 
- [ ] Owners and organizers can remove users from their events    
- [ ] Guests can RSVP to events                                   
- [ ] Authenticated Client can update their RSVPs                 
- [ ] Authenticated Client can view events they are invited to.   
- [ ] Requests and Responses are logged in database               

## Notes on Testing

To run the automated server tests, you can run `cargo test`. To test through the UI, you can 

- Visit the registration route in the UI to create a new account.
  - Don't worry about the "Confirm Password" field. It is, unfortunately, ornamental.
- Visit the login field to log in with the email address and password which you used in the registration flow.
  - There is also a secure admin account which gets created during database migration. It is very secure. The email address is "admin@example.com", and the password is "admin". You can also use this account to log into the system.
- You can visit the "New Event" route and submit the form to create a new event.
  - Note here: I have realized that, despite having start dates, start times, and a text field for "place", events currently have no "name" field. This makes the application less useful - the user will be able to record where they want to be and at what time, but not why.
- You can visit the "Events" route to view the events listed in the database for that user. They are currently listed as JSON.

## Notes on Execution - Wouldas, Couldas, Shouldas

Were I to go through this project again from scratch I would have

- Used an ORM instead of hand-coding the data access layer in raw SQL
- Stuck with Rocket or just started with Axum rather than switching from the former to the latter halfway through the term.
- Worry less about tracing and more about testing.
- Use `clap` to make the program configurable from the command line rather than relying exclusively on environment variables.

I should have

- Added more tests
- Added more documentation comments

## Sources of Inspiration

- User model - [Django web framework's `django.contrib.auth` module](https://docs.djangoproject.com/en/5.1/ref/contrib/auth/)
- Password storage and authentication: [OWASP ASVS](https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/OWASP%20Application%20Security%20Verification%20Standard%204.0.3-en.pdf)

## Useful Documentation Links

Documentation is built with

- [pandoc](https://pandoc.org/)
- [kramdown-rfc](https://github.com/cabo/kramdown-rfc)
- [xml2rfc](https://github.com/ietf-tools/xml2rfc)
