-- Create Schema
CREATE SCHEMA IF NOT EXISTS rs_party;

CREATE TYPE rs_party.response_type AS ENUM(
  'yes'
  , 'no'
  , 'maybe'
  , 'pending'
);

CREATE TYPE role_type AS ENUM(
  'owner'
  , 'organizer'
  , 'guest'
);

CREATE TABLE IF NOT EXISTS rs_party.user (
  id bigint GENERATED ALWAYS AS IDENTITY
  , name            varchar NOT NULL DEFAULT ''
  , email_address   varchar NOT NULL DEFAULT ''
  , password        varchar NOT NULL DEFAULT ''
  , is_superuser    boolean NOT NULL DEFAULT false
  , email_confirmed boolean NOT NULL DEFAULT false
  , CONSTRAINT user_id PRIMARY KEY (id)
);


-- Inspired by the session model used in the Django web framework
-- https://github.com/django/django/blob/main/django/contrib/sessions/base_session.py#L27
CREATE TABLE IF NOT EXISTS rs_party.session (
  id bigint GENERATED ALWAYS AS IDENTITY
  , session_key uuid
  , user_id bigint
  , session_data text
  , created timestamptz
  , updated timestamptz
  , CONSTRAINT session_user_fk FOREIGN KEY (user_id) REFERENCES rs_party.user (id));


CREATE TABLE IF NOT EXISTS rs_party.event (
  id bigint GENERATED ALWAYS AS IDENTITY
  , start_date  date NOT NULL
  , end_date    date NOT NULL
  , start_time  time
  , end_time    time
  , place       varchar NOT NULL DEFAULT ''
  , CONSTRAINT event_id PRIMARY KEY (id)
);

-- Table for tracking invitations to parties
CREATE TABLE IF NOT EXISTS rs_party.invitation (
  id bigint GENERATED ALWAYS AS IDENTITY
  , guest_id    bigint
  , inviter_id  bigint
  , event_id    bigint
  , response    rs_party.response_type 
                NOT NULL DEFAULT 'pending'
  , CONSTRAINT invitation_id PRIMARY KEY (id)
  , CONSTRAINT guest_id_fk FOREIGN KEY (guest_id) REFERENCES rs_party.user (id)
  , CONSTRAINT inviter_id_fk FOREIGN KEY (inviter_id) REFERENCES rs_party.user (id)
  , CONSTRAINT event_id_fk FOREIGN KEY (event_id) REFERENCES rs_party.event (id)
);

-- User roles for authorization of actions in the app
CREATE TABLE IF NOT EXISTS rs_party.role (
  id bigint GENERATED ALWAYS AS IDENTITY
  , role_type role_type
  , user_id bigint
  , event_id bigint
  , CONSTRAINT role_user_fk FOREIGN KEY (user_id) REFERENCES rs_party.user (id)
  , CONSTRAINT role_event_fk FOREIGN KEY (event_id) REFERENCES rs_party.event (id)
);

CREATE TABLE IF NOT EXISTS rs_party.request_log (
  id bigint GENERATED ALWAYS AS IDENTITY
  , time_received timestamptz
  , time_logged timestamptz
  , method text
  , req_url text
  , req_headers text
);

-- Insert Superuser
INSERT INTO rs_party.user (name, email_address, password, is_superuser, email_confirmed)
VALUES (
  'Admin', 'admin@example.com', '$2b$12$5ZtPng4BuPu/9hjd2J23.u/Dp5hbrcDj9DqA.3R7VLxXmLDUy0J5m', true, true
);
