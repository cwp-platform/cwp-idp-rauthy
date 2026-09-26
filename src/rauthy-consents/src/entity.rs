use chrono::Utc;
use hiqlite::macros::{FromRow, params};
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use rauthy_data::database::DB;
#[derive(Debug, Serialize, Deserialize, FromRow, FromPgRow)]
pub struct ConsentDoc {
    pub id: String,
    pub title: String,
    pub url: String,
    pub required: bool,
    pub reconfirm: String,
    pub enabled: bool,
    pub version: i32,
    pub updated_ts: i64,
    pub created_by: String,
}

impl ConsentDoc {
    pub async fn create(
        id: String,
        title: String,
        url: String,
        required: bool,
        reconfirm: String,
        enabled: bool,
        created_by: String,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = r#"
INSERT INTO consent_doc (id, title, url, required, reconfirm, enabled, version, updated_ts, created_by)
VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $8)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &id,
                        &title,
                        &url,
                        required,
                        &reconfirm,
                        enabled,
                        now,
                        &created_by
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &id,
                    &title,
                    &url,
                    &required,
                    &reconfirm,
                    &enabled,
                    &now,
                    &created_by,
                ],
            )
            .await?;
        }

        Ok(())
    }

    /// Persist admin edits. The version is always bumped so that reconfirm logic
    /// (`reconfirm = login | email`) can detect a changed document.
    pub async fn update(
        id: &str,
        title: String,
        url: String,
        required: bool,
        reconfirm: String,
        enabled: bool,
        created_by: String,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = r#"
UPDATE consent_doc
SET title = $2, url = $3, required = $4, reconfirm = $5, enabled = $6,
    version = version + 1, updated_ts = $7, created_by = $8
WHERE id = $1"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        id,
                        &title,
                        &url,
                        required,
                        &reconfirm,
                        enabled,
                        now,
                        &created_by
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &id,
                    &title,
                    &url,
                    &required,
                    &reconfirm,
                    &enabled,
                    &now,
                    &created_by,
                ],
            )
            .await?;
        }

        Ok(())
    }

    /// Soft-disable: the document is hidden from the registration page and can no
    /// longer be accepted, but existing acceptances stay untouched (no DELETE).
    pub async fn disable(id: &str) -> Result<(), ErrorResponse> {
        let sql = "UPDATE consent_doc SET enabled = false WHERE id = $1";

        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?;
        } else {
            DB::pg_execute(sql, &[&id]).await?;
        }

        Ok(())
    }

    pub async fn find(id: &str) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM consent_doc WHERE id = $1";
        let res: Option<Self> = if is_hiqlite() {
            DB::hql().query_as_optional(sql, params!(id)).await?
        } else {
            DB::pg_query_opt(sql, &[&id]).await?
        };
        Ok(res)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM consent_doc ORDER BY id";
        let res: Vec<Self> = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }

    pub async fn find_enabled() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM consent_doc WHERE enabled = true ORDER BY id";
        let res: Vec<Self> = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, FromPgRow)]
pub struct UserConsent {
    pub user_id: String,
    pub consent_id: String,
    pub version: i32,
    pub accept_ts: i64,
    pub url_snapshot: String,
    pub content_hash: Option<String>,
    pub location: String,
    pub withdrawn_at: Option<i64>,
}

impl UserConsent {
    pub async fn create(
        user_id: &str,
        consent_id: &str,
        version: i32,
        url_snapshot: String,
        content_hash: Option<String>,
        ip: IpAddr,
        location: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let accept_ts = Utc::now().timestamp();
        let location = format!("{} {}", ip, location.unwrap_or_default());
        let sql = r#"
INSERT INTO user_consent (user_id, consent_id, version, accept_ts, url_snapshot, content_hash, location, withdrawn_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, NULL)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        user_id,
                        consent_id,
                        version,
                        accept_ts,
                        &url_snapshot,
                        &content_hash,
                        &location
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &user_id,
                    &consent_id,
                    &version,
                    &accept_ts,
                    &url_snapshot,
                    &content_hash,
                    &location,
                ],
            )
            .await?;
        }

        Ok(())
    }

    /// Exact acceptance for a given (user, consent, version) — used for idempotency.
    pub async fn find(
        user_id: &str,
        consent_id: &str,
        version: i32,
    ) -> Result<Option<Self>, ErrorResponse> {
        let sql = r#"
SELECT * FROM user_consent
WHERE user_id = $1 AND consent_id = $2 AND version = $3
LIMIT 1"#;
        let res: Option<Self> = if is_hiqlite() {
            DB::hql()
                .query_as_optional(sql, params!(user_id, consent_id, version))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &consent_id, &version]).await?
        };
        Ok(res)
    }

    pub async fn find_for_user(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM user_consent WHERE user_id = $1 ORDER BY consent_id, version";
        let res: Vec<Self> = if is_hiqlite() {
            DB::hql().query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 0).await?
        };
        Ok(res)
    }

    /// Latest accepted (non-withdrawn) version for (user, consent).
    pub async fn find_latest(
        user_id: &str,
        consent_id: &str,
    ) -> Result<Option<Self>, ErrorResponse> {
        let sql = r#"
SELECT * FROM user_consent
WHERE user_id = $1 AND consent_id = $2 AND withdrawn_at IS NULL
ORDER BY version DESC
LIMIT 1"#;
        let res: Option<Self> = if is_hiqlite() {
            DB::hql()
                .query_as_optional(sql, params!(user_id, consent_id))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&user_id, &consent_id]).await?
        };
        Ok(res)
    }

    /// Support-only revocation. Rows are never deleted: `withdrawn_at` is set on the
    /// still-active acceptance, the audit trail stays intact.
    pub async fn revoke(user_id: &str, consent_id: &str) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let sql = r#"
UPDATE user_consent
SET withdrawn_at = $3
WHERE user_id = $1 AND consent_id = $2 AND withdrawn_at IS NULL"#;

        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(user_id, consent_id, now))
                .await?;
        } else {
            DB::pg_execute(sql, &[&user_id, &consent_id, &now]).await?;
        }

        Ok(())
    }
}
