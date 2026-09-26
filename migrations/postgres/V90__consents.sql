CREATE TABLE consent_doc
(
    id         VARCHAR NOT NULL
        CONSTRAINT consent_doc_pk
            PRIMARY KEY,
    title      VARCHAR NOT NULL,
    url        VARCHAR NOT NULL,
    required   BOOLEAN NOT NULL,
    reconfirm  VARCHAR NOT NULL,
    enabled    BOOLEAN NOT NULL,
    version    INTEGER NOT NULL,
    updated_ts BIGINT  NOT NULL,
    created_by VARCHAR NOT NULL
);

CREATE TABLE user_consent
(
    user_id      VARCHAR NOT NULL
        CONSTRAINT user_consent_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    consent_id   VARCHAR NOT NULL
        CONSTRAINT user_consent_consent_doc_id_fk
            REFERENCES consent_doc
            ON UPDATE CASCADE ON DELETE CASCADE,
    version      INTEGER NOT NULL,
    accept_ts    BIGINT  NOT NULL,
    url_snapshot VARCHAR NOT NULL,
    content_hash VARCHAR,
    location     VARCHAR NOT NULL,
    withdrawn_at BIGINT,
    CONSTRAINT user_consent_pk
        PRIMARY KEY (user_id, consent_id, version)
);

CREATE UNIQUE INDEX user_consent_user_id_consent_id_version_uindex
    ON user_consent (user_id, consent_id, version);
