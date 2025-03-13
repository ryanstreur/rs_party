# Engineering Notebook

## Mon 03/10/2025

Mon 03/10/2025 11:01

Having some trouble with getting the docker-compose setup working.

- When I run `docker-compose up` everything starts up properly, except the server. The server starts up, then crashes, saying it couldn't connect to the database.
- Update: Turns out I was modifying the default connection string thinking I was modifying the active one, but that connection string was being overridden by an environment variable. Time to check in the .env file and confirm this works if I clone the repo fresh.
  - The .env file was the culprit. Figured it out.
