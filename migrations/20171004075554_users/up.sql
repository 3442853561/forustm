-- Your SQL goes here

create extension pgcrypto;

CREATE TABLE ruser (
  id uuid primary key default gen_random_uuid(),
  account VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  salt VARCHAR NOT NULL,
  nickname VARCHAR NOT NULL,
  groups smallint not null default 2,
  avatar VARCHAR,
  wx_openid VARCHAR,
  say VARCHAR,
  signup_time timestamp not null default current_timestamp
);

