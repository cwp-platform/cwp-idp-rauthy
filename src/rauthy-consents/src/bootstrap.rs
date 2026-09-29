//! Идемпотентная нативная сидка consent-документов из `{bootstrap_dir}/consents.json`.
//!
//! Вызывается из `src/bin/src/server.rs` сразу после `DB::migrate()` (таблицы уже
//! созданы). Нет файла / пусто / документ уже существует — пропускаем; `version`
//! никогда не бампится (бамп — только осознанный шаг через Admin UI).

use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use tokio::fs;
use tracing::{debug, info};
use validator::Validate;

use crate::entity::ConsentDoc;
use crate::types::ConsentDocBootstrap;

const BOOTSTRAP_FILE: &str = "consents.json";

pub async fn seed() -> Result<(), ErrorResponse> {
    let dir = RauthyConfig::get().vars.bootstrap.bootstrap_dir.as_ref();
    let path = format!("{dir}/{BOOTSTRAP_FILE}");

    let content = match fs::read(&path).await {
        Ok(c) => c,
        Err(_) => {
            debug!("No file for bootstrapping consents: {path}");
            return Ok(());
        }
    };
    if content.iter().all(|b| b.is_ascii_whitespace()) {
        debug!("Empty file for bootstrapping consents: {path}");
        return Ok(());
    }

    let docs = serde_json::from_slice::<Vec<ConsentDocBootstrap>>(&content).map_err(|e| {
        ErrorResponse::new(
            rauthy_error::ErrorResponseType::BadRequest,
            format!("Invalid consents.json: {e}"),
        )
    })?;

    let mut inserted = 0u32;
    for doc in &docs {
        doc.validate().map_err(|e| {
            ErrorResponse::new(
                rauthy_error::ErrorResponseType::BadRequest,
                format!("Invalid consent doc '{}': {e}", doc.id),
            )
        })?;
        if !matches!(doc.reconfirm.as_str(), "login" | "email" | "none") {
            return Err(ErrorResponse::new(
                rauthy_error::ErrorResponseType::BadRequest,
                format!(
                    "consent '{}': reconfirm must be one of login, email, none",
                    doc.id
                ),
            ));
        }

        if ConsentDoc::find(&doc.id).await?.is_some() {
            debug!("Consent doc already exists, skipping: {}", doc.id);
            continue;
        }

        ConsentDoc::create(
            doc.id.clone(),
            doc.title.clone(),
            doc.url.clone(),
            doc.required,
            doc.reconfirm.clone(),
            doc.enabled,
            "bootstrap".to_string(),
        )
        .await?;
        inserted += 1;
    }

    info!(
        "Consents bootstrap done: {} inserted (file: {path})",
        inserted
    );
    Ok(())
}
