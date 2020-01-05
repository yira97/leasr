-- Your SQL goes here

CREATE TABLE public.users ( id bigserial PRIMARY KEY,
  username character varying(128) NOT NULL UNIQUE,
  display_name character varying(128),
  email character varying(128),
  password character varying(256),
  created_at timestamp without time zone DEFAULT now() NOT NULL,
  updated_at timestamp without time zone DEFAULT now() NOT NULL,
  disable_at timestamp without time zone DEFAULT '1970-01-01 00:00:00'::timestamp without time zone NOT NULL,
  score integer DEFAULT 0 NOT NULL,
  weight integer DEFAULT 0 NOT NULL
);

