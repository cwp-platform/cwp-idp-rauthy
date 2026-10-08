CREATE TABLE consent_doc
(
    id         TEXT    NOT NULL
        CONSTRAINT consent_doc_pk
            PRIMARY KEY,
    title      TEXT    NOT NULL,
    url        TEXT    NOT NULL,
    required   INTEGER NOT NULL,
    reconfirm  TEXT    NOT NULL,
    enabled    INTEGER NOT NULL,
    version    INTEGER NOT NULL,
    updated_ts INTEGER NOT NULL,
    created_by TEXT    NOT NULL
) STRICT;

CREATE TABLE user_consent
(
    user_id      TEXT    NOT NULL
        CONSTRAINT user_consent_users_id_fk
            REFERENCES users
            ON UPDATE CASCADE ON DELETE CASCADE,
    consent_id   TEXT    NOT NULL
        CONSTRAINT user_consent_consent_doc_id_fk
            REFERENCES consent_doc
            ON UPDATE CASCADE ON DELETE CASCADE,
    version      INTEGER NOT NULL,
    accept_ts    INTEGER NOT NULL,
    url_snapshot TEXT    NOT NULL,
    content_hash TEXT,
    location     TEXT    NOT NULL,
    withdrawn_at INTEGER,
    CONSTRAINT user_consent_pk
        PRIMARY KEY (user_id, consent_id, version)
) STRICT;

CREATE UNIQUE INDEX user_consent_user_id_consent_id_version_uindex
    ON user_consent (user_id, consent_id, version);
