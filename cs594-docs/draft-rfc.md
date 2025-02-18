---
stand_alone: true
ipr: trust200902
submissiontype: IETF

cat: info
title: 'RS_Party'
author:
- name: Ryan Streur
  org: PSU

--- abstract

This memo describes the communication protocol for a client/server application which allows users to plan events and invite other users to their events.

--- middle

# API: Anonymous Routes

These routes are available for any Client, regardless of authentication status.

## POST /register

The Client must supply the Server with a JSON body conforming to the following:

{
  "email": "string",
  "name": "string",
  "password": "string"
}

The Server must hash the user's password using a secure hashing method. It will then save the user's data to a new record in the postgresql database. It will then create a new session in the database and return a UUID corresponding to the session key.

The Client may store the session key and use it in the Authorization HTTP header to make authenticated requests until the session expires.

## POST /login

The Client must supply the Server with a JSON body conforming to the following:

{
  "email": "string",
  "password": "string"
}

1. The Server will first attempt to retrieve a user with a matching email address. 
    - If it fails to find one, it will return 400. 
2.  If it does find one, it will attempt to verify the password against the hash from the database. 
    - If the verification fails it will return 400.
3. If the verification succeeds, the Server will check for an existing session and invalidate it if one exists.
4. It will then create a new session and return the UUID session key. The client may save the session key and include it in the header to make subsequent authenticated requests.

# API: Authenticated Routes 

## /event

Each event has a name, a start datetime, an end datetime, and a place (string).

### /event POST

Given new event data (listed above) in the JSON body, create a new event in the database and a new Role record listing the authenticated user as its owner.

### /event GET 

For a logged-in user, retrieve a list of all the events they have access to.

### /event/{id} GET 

If the user has access to the event specified by {id"}, return that event's information. Otherwise, or if the event does not exist, return 404.

### /event/{id} PUT

Given updated data about an existing event and a user who has the Owner or Organizer role, update the event data in the database and return the updated event.

### /event/{id} DELETE

Given a user who is an Owner or Organizer of the event with {id}, delete the event and return 200 if it succeeds.

## /invitation

Invitations have a guest_id corresponding to the invitee, an inviter_id corresponding to the inviter, an event_id which identifies the event the user is being invited to, and a response which stores the guest's RSVP information. They also include a role type, which is the role the user will be given for the event should they accept the invitation (guest, organizer, or owner).

### /invitation POST

Given a user authorized to invite users to the event specified in the invitation information in the JSON body, create a new invitation in the system with the given information.

### /invitation GET

Return a list of the invitations created for the user with information about each event.

### /invitation/{id} PATCH

Given a user whose ID matches the invitation's guest_id, update the "response" field to the value included in the JSON body.

### /invitation/{id} DELETE

Given a user who is either the owner of the corresponding event or the creator of the invitation, delete the invitation. Otherwise return 401.

# Acknowledgements

This document was created using kramdown-rfc, xml2rfc, and was started with [this example](https://github.com/cabo/kramdown-rfc/blob/master/examples/draft-rfcxml-general-template-bare-00.xml-edited.md) document in the kramdown-rfc repo.

--- back
