-- Add migration script here
DROP TABLE public.test;

CREATE TABLE public.user_register
(
    discord_user_id bigint not null,
    CONSTRAINT user_register_pkey PRIMARY KEY (discord_user_id)
)

TABLESPACE pg_default;

ALTER TABLE public.user_register
    OWNER to postgres;

