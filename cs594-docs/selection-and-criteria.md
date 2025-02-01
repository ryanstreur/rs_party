# CS 594 - Project Selection and Grading Criteria: *party.ryanstreur.com*

*Ryan Streur*

As this is a combined project with two other classes, and one of those classes requires an individual assignment, I will be doing this project individually.

## Description

This is a client/server application allowing users to plan events and parties. The server component will be a REST API written in Rust which will serve as a middle tier between the client component (Vue.js), and the database (postgresql). Users will be able to create events, manage members of their events, and create invitations to their events. It will use password authentication and JWT bearer tokens to authenticate client application instances, with a database table for tracking user sessions. It will use a role-based authorization strategy to determine which users are authorized to perform which actions on specific events. It will also log each incoming HTTP request, the response code for that request, and the time it took the request to complete.

## Grading Criteria

|      | Item                                                        | Points | Scored |
| ---: | :---------------------------------------------------------- | -----: | -----: |
|    1 | RFC Document                                                |     20 |        |
|    2 | API Server Process                                          |      3 |        |
|    3 | API Client can create user account                          |     10 |        |
|    4 | API Client can log in (password)                            |     15 |        |
|    5 | API Client can log out                                      |     10 |        |
|    6 | API Client can make authenticated requests (Bearer JWT)     |     15 |        |
|    7 | Authenticated Client can create events                      |      3 |        |
|    8 | Owners and Organizers can list their events                 |      3 |        |
|    9 | Owners and Organizers can delete their events               |      3 |        |
|   10 | Owners and Organizers can update their events               |      3 |        |
|   11 | Owners and Organizers can invite other users to events      |      3 |        |
|   12 | Owners and Organizers can assign user roles on their events |      3 |        |
|   13 | Owners and organizers can remove users from their events    |      3 |        |
|   14 | Guests can RSVP to events                                   |      3 |        |
|   15 | Authenticated Client can update their RSVPs                 |      3 |        |
|   16 | Authenticated Client can view events they are invited to.   |      2 |        |
|   17 | Requests and Responses are logged in database               |     10 |        |
|   18 | Programming Style                                           |     10 |        |

Total: 120 Points

## Extra Credit Features

|      | Item                                                         | Points | Scored |
| ---: | :----------------------------------------------------------- | -----: | -----: |
|      | Secure Password Storage                                      |      5 |        |
|      | Nginx Reverse Proxy                                          |      5 |        |
|      | Deployed Instance at party.ryanstreur.com                    |     10 |        |
|      | Creating an invitation sends an email notification over SMTP |     10 |        |
