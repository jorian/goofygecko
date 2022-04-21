-- Add migration script here
CREATE TABLE public.test
(
    id bigint not null,
    CONSTRAINT test_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE public.test
    OWNER to postgres;