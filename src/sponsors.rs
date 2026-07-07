use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::app::{fetch_sponsor_stats_via_scrape, fetch_sponsors_list_via_scrape};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SponsorRecord {
    pub login: String,
    pub monthly_price_in_cents: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SponsorsStore {
    pub current: Vec<SponsorRecord>,
    pub past: Vec<SponsorRecord>,
}

pub type SponsorsState = Arc<RwLock<SponsorsStore>>;

#[derive(Clone)]
pub struct WebhookConfig {
    pub secret: Arc<String>,
    pub state_path: Arc<PathBuf>,
}

fn load(path: &Path) -> Option<SponsorsStore> {
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

fn persist(path: &Path, store: &SponsorsStore) {
    if let Ok(json) = serde_json::to_string_pretty(store) {
        let _ = fs::write(path, json);
    }
}

pub async fn load_or_bootstrap(path: &Path) -> SponsorsState {
    if let Some(store) = load(path) {
        return Arc::new(RwLock::new(store));
    }

    let list = fetch_sponsors_list_via_scrape().await;
    let stats = fetch_sponsor_stats_via_scrape().await;

    let per_sponsor_cents = if list.current.is_empty() {
        0
    } else {
        (stats.current_monthly_income as i64 * 100) / list.current.len() as i64
    };

    let store = SponsorsStore {
        current: list
            .current
            .into_iter()
            .map(|login| SponsorRecord {
                login,
                monthly_price_in_cents: per_sponsor_cents,
            })
            .collect(),
        past: list
            .past
            .into_iter()
            .map(|login| SponsorRecord {
                login,
                monthly_price_in_cents: 0,
            })
            .collect(),
    };

    persist(path, &store);
    Arc::new(RwLock::new(store))
}

#[derive(Debug, Deserialize)]
struct SponsorshipEvent {
    action: String,
    sponsorship: SponsorshipPayload,
}

#[derive(Debug, Deserialize)]
struct SponsorshipPayload {
    sponsor: SponsorUser,
    tier: SponsorshipTier,
}

#[derive(Debug, Deserialize)]
struct SponsorUser {
    login: String,
}

#[derive(Debug, Deserialize)]
struct SponsorshipTier {
    monthly_price_in_cents: i64,
    is_one_time: bool,
}

fn apply_event(store: &mut SponsorsStore, event: &SponsorshipEvent) {
    let login = &event.sponsorship.sponsor.login;
    let monthly_price_in_cents = event.sponsorship.tier.monthly_price_in_cents;

    match event.action.as_str() {
        "created" => {
            store.current.retain(|r| !r.login.eq_ignore_ascii_case(login));
            store.past.retain(|r| !r.login.eq_ignore_ascii_case(login));

            let record = SponsorRecord {
                login: login.clone(),
                monthly_price_in_cents,
            };

            if event.sponsorship.tier.is_one_time {
                store.past.push(record);
            } else {
                store.current.push(record);
            }
        }
        "cancelled" => {
            if let Some(pos) = store
                .current
                .iter()
                .position(|r| r.login.eq_ignore_ascii_case(login))
            {
                let mut record = store.current.remove(pos);
                record.monthly_price_in_cents = monthly_price_in_cents;
                store.past.push(record);
            }
        }
        "tier_changed" | "edited" => {
            if let Some(r) = store
                .current
                .iter_mut()
                .find(|r| r.login.eq_ignore_ascii_case(login))
            {
                r.monthly_price_in_cents = monthly_price_in_cents;
            }
        }
        _ => {}
    }
}

fn hex_decode(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }

    let chars: Vec<char> = hex.chars().collect();
    let mut bytes = Vec::with_capacity(chars.len() / 2);

    for pair in chars.chunks(2) {
        let hi = pair[0].to_digit(16)?;
        let lo = pair[1].to_digit(16)?;
        bytes.push(((hi << 4) | lo) as u8);
    }

    Some(bytes)
}

fn verify_signature(secret: &[u8], body: &[u8], header_value: &str) -> bool {
    let Some(hex_sig) = header_value.strip_prefix("sha256=") else {
        return false;
    };
    let Some(sig_bytes) = hex_decode(hex_sig) else {
        return false;
    };
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret) else {
        return false;
    };

    mac.update(body);
    mac.verify_slice(&sig_bytes).is_ok()
}

pub async fn webhook_handler(
    State(sponsors): State<SponsorsState>,
    State(config): State<WebhookConfig>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    if config.secret.is_empty() {
        return StatusCode::SERVICE_UNAVAILABLE;
    }

    let Some(signature) = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
    else {
        return StatusCode::BAD_REQUEST;
    };

    if !verify_signature(config.secret.as_bytes(), &body, signature) {
        return StatusCode::UNAUTHORIZED;
    }

    if headers.get("X-GitHub-Event").and_then(|v| v.to_str().ok()) == Some("ping") {
        return StatusCode::OK;
    }

    let Ok(event) = serde_json::from_slice::<SponsorshipEvent>(&body) else {
        return StatusCode::BAD_REQUEST;
    };

    let updated = {
        let mut store = sponsors.write().await;
        apply_event(&mut store, &event);
        store.clone()
    };
    persist(&config.state_path, &updated);

    StatusCode::OK
}
