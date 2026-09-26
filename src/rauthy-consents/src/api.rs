use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, delete, get, post, web};
use rauthy_common::utils::real_ip_from_req;
use rauthy_data::entity::principal::Principal;
use rauthy_data::entity::users::User;
use rauthy_data::ipgeo::get_location;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use validator::Validate;

use crate::entity::{ConsentDoc, UserConsent};
use crate::types::{
    ConsentAcceptRequest, ConsentDocPublic, ConsentDocRequest, ConsentDocResponse,
    ConsentPendingItem, UserConsentResponse,
};
use crate::{needs_login_reconfirm, validate_registration_consents};

/// Returns all consent documents.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/consents",
    responses(
        (status = 200, description = "Ok", body = [ConsentDocResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("")]
pub async fn get_consents(
    principal: web::ReqData<Principal>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let resp = ConsentDoc::find_all()
        .await?
        .into_iter()
        .map(ConsentDocResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(resp))
}

/// Create a new consent document or update an existing one (version is bumped).
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/consents",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("")]
pub async fn post_consents(
    principal: web::ReqData<Principal>,
    payload: Json<ConsentDocRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let payload = payload.into_inner();
    payload.validate()?;
    validate_reconfirm(&payload.reconfirm)?;
    validate_doc_url(&payload.url)?;

    let user = User::find(principal.user_id()?.to_string()).await?;

    if ConsentDoc::find(&payload.id).await?.is_some() {
        ConsentDoc::update(
            &payload.id,
            payload.title,
            payload.url,
            payload.required,
            payload.reconfirm,
            payload.enabled,
            user.email,
        )
        .await?;
    } else {
        ConsentDoc::create(
            payload.id,
            payload.title,
            payload.url,
            payload.required,
            payload.reconfirm,
            payload.enabled,
            user.email,
        )
        .await?;
    }

    Ok(HttpResponse::Ok().finish())
}

/// Soft-disable a consent document. The row is never deleted.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/consents/{id}",
    responses(
        (status = 200, description = "Ok"),
        (status = 404, description = "NotFound", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/{id}")]
pub async fn delete_consent(
    principal: web::ReqData<Principal>,
    id: Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    if ConsentDoc::find(&id).await?.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "Consent document not found",
        ));
    }

    ConsentDoc::disable(&id).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Returns all acceptance records for a user.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/consents/user/{id}",
    responses(
        (status = 200, description = "Ok", body = [UserConsentResponse]),
        (status = 204, description = "NoContent"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/user/{id}")]
pub async fn get_consents_user(
    principal: web::ReqData<Principal>,
    id: Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let res = UserConsent::find_for_user(&id)
        .await?
        .into_iter()
        .map(UserConsentResponse::from)
        .collect::<Vec<_>>();

    if res.is_empty() {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::Ok().json(res))
    }
}

/// Revoke a consent for a user. Support-only operation; the audit row is kept.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/consents/user/{uid}/consent/{cid}/revoke",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/user/{uid}/consent/{cid}/revoke")]
pub async fn post_consent_revoke(
    principal: web::ReqData<Principal>,
    ids: Path<(String, String)>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let (uid, cid) = ids.into_inner();
    UserConsent::revoke(&uid, &cid).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Returns the active consent documents for the registration page (anonymous).
#[utoipa::path(
    get,
    path = "/consents/public",
    responses(
        (status = 200, description = "Ok", body = [ConsentDocPublic]),
    ),
)]
#[get("/public")]
pub async fn get_consents_public() -> Result<HttpResponse, ErrorResponse> {
    let resp = ConsentDoc::find_enabled()
        .await?
        .into_iter()
        .map(ConsentDocPublic::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(resp))
}

/// Returns consent documents the current user must (or may) re-confirm at login
/// (`reconfirm = login` and a newer version than the accepted one).
///
/// **Permissions**
/// - session in auth state
#[utoipa::path(
    post,
    path = "/consents/pending",
    responses(
        (status = 200, description = "Ok", body = [ConsentPendingItem]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/pending")]
pub async fn post_consents_pending(
    principal: web::ReqData<Principal>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let uid = principal.user_id()?.to_string();
    let user = User::find(uid).await?;
    let pending = needs_login_reconfirm(&user).await?;

    Ok(HttpResponse::Ok().json(pending))
}

/// Accept one or more consent documents at their current version.
///
/// **Permissions**
/// - session in auth state
#[utoipa::path(
    post,
    path = "/consents/accept",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
#[post("/accept")]
pub async fn post_consents_accept(
    principal: web::ReqData<Principal>,
    req: HttpRequest,
    payload: Json<ConsentAcceptRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth()?;

    let payload = payload.into_inner();
    payload.validate()?;

    let uid = principal.user_id()?.to_string();
    let ip = real_ip_from_req(&req)?;
    let loc = get_location(&req, ip)?;

    for item in payload.consents {
        let Some(doc) = ConsentDoc::find(&item.id).await? else {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Consent document not found",
            ));
        };
        if !doc.enabled {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Consent document not found",
            ));
        }
        if doc.version != item.version {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "Consent {} has a newer version than the accepted one",
                    doc.id
                ),
            ));
        }
        if UserConsent::find(&uid, &item.id, item.version)
            .await?
            .is_none()
        {
            UserConsent::create(&uid, &item.id, item.version, doc.url, None, ip, loc.clone())
                .await?;
        }
    }

    Ok(HttpResponse::Ok().finish())
}

/// Convenience hook used by the open-registration handler: validates the submitted
/// consent ids against the enabled documents before the user is created.
pub async fn handle_registration_consents(
    accepted_ids: Option<&[String]>,
) -> Result<Vec<crate::types::RegistrationConsent>, ErrorResponse> {
    validate_registration_consents(accepted_ids).await
}

fn validate_reconfirm(reconfirm: &str) -> Result<(), ErrorResponse> {
    match reconfirm {
        "login" | "email" | "none" => Ok(()),
        _ => Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "reconfirm must be one of: login, email, none",
        )),
    }
}

fn validate_doc_url(url: &str) -> Result<(), ErrorResponse> {
    let dev_mode = RauthyConfig::get().vars.dev.dev_mode;
    let Some((scheme, rest)) = url.split_once("://") else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "url must contain a scheme",
        ));
    };
    if scheme != "https" && !(dev_mode && scheme == "http") {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "url must use https (http is allowed in dev mode only)",
        ));
    }
    if rest.is_empty() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "url must contain a host",
        ));
    }
    Ok(())
}
