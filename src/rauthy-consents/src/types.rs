use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Admin create/update payload for a consent document.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ConsentDocRequest {
    /// Unique slug / id of the consent document.
    #[validate(length(min = 1, max = 64))]
    pub id: String,
    /// User-facing title.
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    /// External static URL of the document content.
    #[validate(length(min = 1, max = 500))]
    pub url: String,
    /// Whether the consent is mandatory at registration.
    pub required: bool,
    /// Behavior on version bump: `login`, `email` or `none`.
    #[validate(length(min = 1, max = 16))]
    pub reconfirm: String,
    /// Whether the document is shown at registration.
    pub enabled: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConsentDocResponse {
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

/// Public view used by the registration page (no `reconfirm`/`enabled`).
#[derive(Debug, Serialize, ToSchema)]
pub struct ConsentDocPublic {
    pub id: String,
    pub title: String,
    pub url: String,
    pub required: bool,
    pub version: i32,
}

/// A single acceptance record (append-only row).
#[derive(Debug, Serialize, ToSchema)]
pub struct UserConsentResponse {
    pub user_id: String,
    pub consent_id: String,
    pub version: i32,
    pub accept_ts: i64,
    pub url_snapshot: String,
    pub content_hash: Option<String>,
    pub location: String,
    pub withdrawn_at: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ConsentAcceptRequest {
    #[validate(length(min = 1, max = 64))]
    pub consents: Vec<ConsentAcceptItem>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ConsentAcceptItem {
    #[validate(length(min = 1, max = 64))]
    pub id: String,
    pub version: i32,
}

/// Consent that the user must (or may) re-confirm at login.
#[derive(Debug, Serialize, ToSchema)]
pub struct ConsentPendingItem {
    pub id: String,
    pub title: String,
    pub url: String,
    pub required: bool,
    pub version: i32,
}

/// Internal value returned by `validate_registration_consents`.
#[derive(Debug, Clone)]
pub struct RegistrationConsent {
    pub id: String,
    pub version: i32,
    pub url: String,
}
