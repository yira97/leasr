-- Your SQL goes here
create table post_tag (
  name varchar(256) not null primary key,
  created_at timestamp not null default now(),
  disable_at timestamp not null default '1970-01-01 00:00:00'
);

create table posts (
  id bigserial primary key,
  title varchar(512) not null,
  describe text not null,
  content text not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now(),
  disable_at timestamp not null default '1970-01-01 00:00:00',
  creator bigint not null,
  tag varchar(256)
);

create table post_change (
  id bigserial primary key,
  post_id bigint not null,
  created_at timestamp not null default now(),
  change_name varchar(64) not null,
  old text not null,
  new text not null
);
