-- Connect to database
\c rs_party

-- Drop the tables
DROP TABLE IF EXISTS rs_party.invitation;
DROP TABLE IF EXISTS rs_party.role;
DROP TABLE IF EXISTS rs_party.request_log;
DROP TABLE IF EXISTS rs_party.session;
DROP TABLE IF EXISTS rs_party.event;
DROP TABLE IF EXISTS rs_party.user;


-- Drop the types
DROP TYPE IF EXISTS rs_party.response_type;
DROP TYPE IF EXISTS rs_party.role_type;

-- Drop the schema
DROP SCHEMA IF EXISTS rs_party;
