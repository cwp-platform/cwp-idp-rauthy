use actix_web::{Scope, web};
use rauthy_data::entity::users::User;
use rauthy_error::{ErrorResponse, ErrorResponseType};

use crate::entity::{ConsentDoc, UserConsent};
use crate::types::{
    ConsentDocPublic, ConsentDocResponse, ConsentPendingItem, RegistrationConsent,
    UserConsentResponse,
};

pub mod api;
pub mod entity;
pub mod types;

/// All `/consents` routes under the `/auth/v1` scope.
pub fn consents_scope() -> Scope {
    web::scope("/consents")
        .service(api::get_consents)
        .service(api::post_consents)
        .service(api::delete_consent)
        .service(api::get_consents_user)
        .service(api::post_consent_revoke)
        .service(api::get_consents_public)
        .service(api::post_consents_pending)
        .service(api::post_consents_accept)
}

/// Validates the consent ids submitted during open registration. Every `required`
/// and enabled document must be present; unknown/disabled ids are rejected.
/// Returns the documents to persist, so the caller can record them after the user
/// was successfully created.
pub async fn validate_registration_consents(
    accepted_ids: Option<&[String]>,
) -> Result<Vec<RegistrationConsent>, ErrorResponse> {
    let docs = ConsentDoc::find_enabled().await?;
    let accepted: Vec<&str> = accepted_ids
        .unwrap_or(&[])
        .iter()
        .map(|s| s.as_str())
        .collect();

    for doc in &docs {
        if doc.required && !accepted.contains(&doc.id.as_str()) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Required consent missing: {}", doc.id),
            ));
        }
    }

    let mut out = Vec::with_capacity(accepted.len());
    for id in accepted {
        let Some(doc) = docs.iter().find(|d| d.id == id) else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Unknown consent id: {id}"),
            ));
        };
        if !doc.enabled {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("Consent is disabled: {id}"),
            ));
        }
        out.push(RegistrationConsent {
            id: doc.id.clone(),
            version: doc.version,
            url: doc.url.clone(),
        });
    }

    Ok(out)
}

/// Appends acceptance rows for the consents validated during registration.
pub async fn record_registration_consents(
    user_id: &str,
    consents: &[RegistrationConsent],
    ip: std::net::IpAddr,
    location: Option<String>,
) -> Result<(), ErrorResponse> {
    for c in consents {
        UserConsent::create(
            user_id,
            &c.id,
            c.version,
            c.url.clone(),
            None,
            ip,
            location.clone(),
        )
        .await?;
    }
    Ok(())
}

/// Returns the consent documents the user must (or may) re-confirm at login:
/// `reconfirm = login` and a newer version than the latest accepted one.
pub async fn needs_login_reconfirm(user: &User) -> Result<Vec<ConsentPendingItem>, ErrorResponse> {
    let docs = ConsentDoc::find_enabled().await?;

    let mut pending = Vec::new();
    for doc in docs {
        if doc.reconfirm != "login" {
            continue;
        }

        let accepted_version = UserConsent::find_latest(&user.id, &doc.id)
            .await?
            .map(|c| c.version)
            .unwrap_or(0);

        if doc.version > accepted_version {
            pending.push(ConsentPendingItem {
                id: doc.id.clone(),
                title: doc.title.clone(),
                url: doc.url.clone(),
                required: doc.required,
                version: doc.version,
            });
        }
    }

    Ok(pending)
}

impl From<ConsentDoc> for ConsentDocResponse {
    fn from(doc: ConsentDoc) -> Self {
        Self {
            id: doc.id,
            title: doc.title,
            url: doc.url,
            required: doc.required,
            reconfirm: doc.reconfirm,
            enabled: doc.enabled,
            version: doc.version,
            updated_ts: doc.updated_ts,
            created_by: doc.created_by,
        }
    }
}

impl From<ConsentDoc> for ConsentDocPublic {
    fn from(doc: ConsentDoc) -> Self {
        Self {
            id: doc.id,
            title: doc.title,
            url: doc.url,
            required: doc.required,
            version: doc.version,
        }
    }
}

impl From<UserConsent> for UserConsentResponse {
    fn from(c: UserConsent) -> Self {
        Self {
            user_id: c.user_id,
            consent_id: c.consent_id,
            version: c.version,
            accept_ts: c.accept_ts,
            url_snapshot: c.url_snapshot,
            content_hash: c.content_hash,
            location: c.location,
            withdrawn_at: c.withdrawn_at,
        }
    }
}
