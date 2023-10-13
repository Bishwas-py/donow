CREATE TABLE todos
(
    id          serial                 NOT NULL,
    title       character varying(255) NOT NULL,
    description text                   NULL,
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP,

    CONSTRAINT videos_pkey PRIMARY KEY (id)
)