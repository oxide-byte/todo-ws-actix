CREATE DATABASE todo;
CREATE USER todo WITH PASSWORD 'password';
GRANT ALL ON DATABASE todo TO todo;

CREATE DATABASE keycloak;
CREATE USER keycloak WITH PASSWORD 'keycloak';
GRANT ALL ON DATABASE keycloak TO keycloak;

-- To Do

\connect todo todo;
CREATE SCHEMA todo;

create table if not exists todo.todo
(
    id          int     not null,
    owner       varchar not null,
    name        varchar not null,
    description varchar not null,
    status      varchar not null,
    created     timestamp with time zone,
    modified    timestamp with time zone,
    PRIMARY KEY (id)
);

CREATE SEQUENCE todo.todo_seq
    INCREMENT 1
    START 1;

-- Keycloak

\connect keycloak keycloak;
CREATE SCHEMA keycloak;