use leptos::prelude::*;
use leptos_fluent::move_tr;
use serde::{Deserialize, Serialize};

use crate::icons::Icon;

#[cfg(feature = "ssr")]
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SponsorStats {
    monthly_goal: u32,
    pub(crate) current_monthly_income: u32,
    progress_percent: u32,
}

impl SponsorStats {
    fn fallback() -> Self {
        let monthly_goal = 400;
        let current_monthly_income = 32;
        let progress_percent = (current_monthly_income * 100) / monthly_goal;

        Self {
            monthly_goal,
            current_monthly_income,
            progress_percent,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SponsorsList {
    pub(crate) current: Vec<String>,
    pub(crate) past: Vec<String>,
}

pub(crate) struct SponsorTiers {
    pub(crate) special: Vec<String>,
    pub(crate) monthly: Vec<String>,
    pub(crate) one_time: Vec<String>,
}

/// Special sponsors are always displayed in their own tier, regardless of the
/// persisted store or what GitHub currently lists publicly.
const SPECIAL_SPONSORS: &[&str] = &["WillLillis", "shytzedaka"];

impl SponsorsList {
    /// Ensure every pinned special sponsor is present in `past`, deduped
    /// case-insensitively. Applied whenever the list is surfaced to the UI.
    fn with_special_sponsors(mut self) -> Self {
        for &login in SPECIAL_SPONSORS {
            if !self.past.iter().any(|p| p.eq_ignore_ascii_case(login)) {
                self.past.push(login.to_string());
            }
        }
        self
    }

    fn is_special_one_time(login: &str) -> bool {
        SPECIAL_SPONSORS
            .iter()
            .any(|special| special.eq_ignore_ascii_case(login))
    }

    fn regular_one_time(&self) -> impl Iterator<Item = &String> {
        self.past
            .iter()
            .filter(|login| !Self::is_special_one_time(login))
    }

    /// The three tiers the support page and the home page both render.
    pub(crate) fn tiers(&self) -> SponsorTiers {
        SponsorTiers {
            special: SPECIAL_SPONSORS.iter().map(|s| s.to_string()).collect(),
            monthly: self.current.clone(),
            one_time: self.regular_one_time().cloned().collect(),
        }
    }

    pub(crate) fn fallback() -> Self {
        Self {
            current: [
                "m110",
                "ozanshx",
                "FormalSnake",
                "baronunread",
                "UMCEKO",
                "nmariscal86",
                "Clippsly",
            ]
            .map(String::from)
            .to_vec(),
            past: [
                "Iamknownasfesal",
                "arda2k3",
                "bulakemun",
                "AniviaFlome",
                "SeriousPassenger",
            ]
            .map(String::from)
            .to_vec(),
        }
    }
}

#[cfg(feature = "ssr")]
fn slice_between<'a>(text: &'a str, start_marker: &str, end_marker: &str) -> Option<&'a str> {
    let start = text.find(start_marker)? + start_marker.len();
    let end = text[start..]
        .find(end_marker)
        .map(|offset| start + offset)
        .unwrap_or(text.len());
    Some(&text[start..end])
}

#[cfg(feature = "ssr")]
fn is_username_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-'
}

#[cfg(feature = "ssr")]
fn extract_github_usernames(text: &str) -> Vec<String> {
    // GitHub renders sponsor avatars as relative links, e.g.
    // `data-hovercard-url="/users/name/hovercard"` (or `/orgs/` for org sponsors),
    // not absolute `https://github.com/name` URLs.
    const MARKERS: &[&str] = &[
        "data-hovercard-url=\"/users/",
        "data-hovercard-url=\"/orgs/",
    ];

    let mut names = Vec::new();
    let mut seen = HashSet::new();

    for marker in MARKERS {
        let mut cursor = 0;

        while let Some(found) = text[cursor..].find(marker) {
            let start = cursor + found + marker.len();
            let mut end = start;

            for ch in text[start..].chars() {
                if is_username_char(ch) {
                    end += ch.len_utf8();
                } else {
                    break;
                }
            }

            if end > start {
                let name = &text[start..end];
                let lower = name.to_ascii_lowercase();
                let valid_len = (1..=39).contains(&name.len());
                if valid_len && lower != "temidaradev" && seen.insert(lower) {
                    names.push(name.to_string());
                }
            }

            cursor = end.max(cursor + found + marker.len());
            if cursor >= text.len() {
                break;
            }
        }
    }

    names
}

pub(crate) async fn fetch_sponsors_list() -> SponsorsList {
    #[cfg(feature = "ssr")]
    {
        if let Some(state) = use_context::<crate::sponsors::SponsorsState>() {
            let store = state.read().await;
            return SponsorsList {
                current: store.current.iter().map(|r| r.login.clone()).collect(),
                past: store.past.iter().map(|r| r.login.clone()).collect(),
            }
            .with_special_sponsors();
        }
    }

    fetch_sponsors_list_via_scrape()
        .await
        .with_special_sponsors()
}

pub(crate) async fn fetch_sponsors_list_via_scrape() -> SponsorsList {
    #[cfg(feature = "ssr")]
    {
        let url = "https://github.com/sponsors/temidaradev";
        let client = reqwest::Client::builder()
            .user_agent("kopuz-website/1.0")
            .build();

        if let Ok(client) = client {
            let response = client.get(url).send().await;
            if let Ok(response) = response {
                if let Ok(body) = response.text().await {
                    // Each section's sponsor grid is wrapped in a <remote-pagination> element;
                    // its closing tag is a tight, unambiguous boundary (unlike e.g. "Select a
                    // tier", which is far enough away to swallow unrelated tier-widget markup
                    // that repeats current sponsors' avatars).
                    let current_section =
                        slice_between(&body, "Current sponsors", "</remote-pagination>");
                    let past_section =
                        slice_between(&body, "Past sponsors", "</remote-pagination>");

                    if let (Some(current_section), Some(past_section)) =
                        (current_section, past_section)
                    {
                        let current = extract_github_usernames(current_section);
                        let past = extract_github_usernames(past_section);

                        if !current.is_empty() || !past.is_empty() {
                            return SponsorsList { current, past };
                        }
                    }
                }
            }
        }
    }

    SponsorsList::fallback()
}

#[cfg(feature = "ssr")]
fn parse_uint_after_marker(text: &str, marker: &str) -> Option<u32> {
    let start = text.find(marker)? + marker.len();
    let mut digits = String::new();

    for ch in text[start..].chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else if !digits.is_empty() {
            break;
        }
    }

    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

#[cfg(feature = "ssr")]
fn parse_uint_before_marker(text: &str, marker: &str) -> Option<u32> {
    let end = text.find(marker)?;
    let chars: Vec<char> = text[..end].chars().collect();
    let mut digits_reversed = String::new();

    for ch in chars.iter().rev() {
        if ch.is_ascii_digit() {
            digits_reversed.push(*ch);
        } else if !digits_reversed.is_empty() {
            break;
        }
    }

    if digits_reversed.is_empty() {
        None
    } else {
        let digits: String = digits_reversed.chars().rev().collect();
        digits.parse().ok()
    }
}

#[cfg(feature = "ssr")]
fn parse_goal_progress_percent(text: &str) -> Option<u32> {
    // GitHub renders the progress as inline style width on the goal bar.
    let marker = "sponsors-goal-progress-bar";
    let idx = text.find(marker)?;

    // width appears before the class in the same element in GitHub's markup.
    let lookback_start = idx.saturating_sub(300);
    let window = &text[lookback_start..idx];
    let width_idx = window.rfind("width:")? + "width:".len();
    let mut digits = String::new();

    for ch in window[width_idx..].chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else if !digits.is_empty() {
            break;
        }
    }

    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

async fn fetch_sponsor_stats() -> SponsorStats {
    #[cfg(feature = "ssr")]
    {
        if let Some(state) = use_context::<crate::sponsors::SponsorsState>() {
            let store = state.read().await;
            let monthly_goal: u32 = 400;
            let total_cents: i64 = store.current.iter().map(|r| r.monthly_price_in_cents).sum();
            let current_monthly_income = (total_cents / 100).max(0) as u32;
            let progress_percent = (current_monthly_income * 100) / monthly_goal;

            return SponsorStats {
                monthly_goal,
                current_monthly_income,
                progress_percent,
            };
        }
    }

    fetch_sponsor_stats_via_scrape().await
}

pub(crate) async fn fetch_sponsor_stats_via_scrape() -> SponsorStats {
    #[cfg(feature = "ssr")]
    {
        let url = "https://github.com/sponsors/temidaradev";
        let client = reqwest::Client::builder()
            .user_agent("kopuz-website/1.0")
            .build();

        if let Ok(client) = client {
            let response = client.get(url).send().await;
            if let Ok(response) = response {
                if let Ok(body) = response.text().await {
                    let monthly_goal = parse_uint_after_marker(&body, "goal is to")
                        .or_else(|| parse_uint_after_marker(&body, "towards"))
                        .unwrap_or(400);

                    let progress_percent = parse_goal_progress_percent(&body)
                        .or_else(|| parse_uint_before_marker(&body, "% towards"))
                        .unwrap_or(0);

                    let current_monthly_income = (monthly_goal * progress_percent) / 100;

                    if monthly_goal > 0 {
                        return SponsorStats {
                            monthly_goal,
                            current_monthly_income,
                            progress_percent,
                        };
                    }
                }
            }
        }
    }

    SponsorStats::fallback()
}

const CRYPTO: &[(&str, &str)] = &[
    ("SOL", "2fapJYRztnTRLpJbmyEUnsuZ36AzLK2JrMmmLEfDqKpN"),
    ("BTC", "bc1qz94yz9xvufa6hxlvjzaajgd2zyfu86arn68hu4"),
    (
        "XMR",
        "86mz3HxTrKyYpuvx78m6pufbXdwAnoyoZBztz6HyYrnM1XP5YVrMy9jTVRY5vzgGtkizACLpFwHEdafKTMoj6y8mAVgvWMz",
    ),
    ("ETH", "0xa490D50470cdFf837B6663F7f6cBe50B157224e5"),
];

const USDT: &str = "GYmnAcrA5MbF6cUxT2m5d5cwdfr14qSY9WFYRwXxaibW";

fn crypto_row(coin: &'static str, address: &'static str) -> impl IntoView {
    view! {
        <div class="crypto-row">
            <span class="crypto-coin">{coin}</span>
            <code>{address}</code>
            <button
                type="button"
                class="copy-btn"
                data-copy=address
                data-copied-label=move_tr!("copied")
                aria-label=move_tr!("copy-address", { "coin" => coin })
            >{move_tr!("copy")}</button>
        </div>
    }
}

/// A sponsor's GitHub avatar. The username is carried by `title` and
/// `aria-label`, so the image itself stays out of the accessibility tree.
/// Simple mode drops the avatar and shows the username instead.
pub(crate) fn sponsor_avatar(login: &str, px: u32) -> impl IntoView {
    let profile = format!("https://github.com/{login}");
    let src = format!("https://github.com/{login}.png?size={}", px * 2);
    let name = login.to_string();
    let label = login.to_string();

    view! {
        <a
            href=profile
            target="_blank"
            rel="noopener noreferrer"
            title=name.clone()
            aria-label=name
        >
            {if crate::shell::simple_mode() {
                view! { <span>{label}</span> }.into_any()
            } else {
                view! { <img src=src alt="" width=px height=px loading="lazy"/> }.into_any()
            }}
        </a>
    }
}

#[component]
pub(crate) fn Support() -> impl IntoView {
    let sponsor_stats = Resource::new(|| (), |_| async move { fetch_sponsor_stats().await });

    view! {
        <section class="sec support" id="support" data-title="Support">
            <div class="wrap">
                <div class="sec-head">
                    <h1>{move_tr!("support-title")}</h1>
                    <p class="lede">{move_tr!("support-subtitle")}</p>
                </div>
                <p class="prose support-note">{move_tr!("support-note")}</p>
                <Suspense fallback=|| view! { <div class="donation-progress-wrap"></div> }>
                    {move || {
                        let stats = sponsor_stats.get().unwrap_or_else(SponsorStats::fallback);
                        let percent = stats.progress_percent;
                        let current = stats.current_monthly_income;
                        let goal = stats.monthly_goal;
                        let width = format!("width: {}%;", percent.min(100));

                        view! {
                            <div class="donation-progress-wrap">
                                <p class="donation-progress">
                                    <span>{move_tr!("support-goal-label")}</span>
                                    <span class="tabular">
                                        {move_tr!(
                                            "support-goal-amount",
                                            { "current" => current, "goal" => goal }
                                        )}
                                    </span>
                                </p>
                                <div
                                    class="donation-progress-track"
                                    role="progressbar"
                                    aria-label=move_tr!("support-goal-aria")
                                    aria-valuemin="0"
                                    aria-valuemax="100"
                                    aria-valuenow=percent.to_string()
                                >
                                    <span class="donation-progress-fill" style=width></span>
                                </div>
                            </div>
                        }
                    }}
                </Suspense>
                <div class="give-row">
                    <a
                        href="https://github.com/sponsors/temidaradev"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn-primary"
                    >
                        <Icon name="heart" size=15/>
                        {move_tr!("support-gh")}
                    </a>
                    <a
                        href="https://buymeacoffee.com/temidaradev"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn-ghost"
                    >
                        {move_tr!("support-bmc")}
                    </a>
                </div>
                <details class="crypto">
                    <summary>
                        {move_tr!("support-crypto-summary")}
                        <Icon name="chevron-down" size=14 class="crypto-chevron"/>
                    </summary>
                    <div class="crypto-list">
                        {CRYPTO
                            .iter()
                            .map(|&(coin, address)| crypto_row(coin, address))
                            .collect_view()}
                        <div class="crypto-row">
                            <span class="crypto-coin">"USDT"</span>
                            <code>{USDT}</code>
                            <span class="crypto-note">{move_tr!("support-usdt-note")}</span>
                            <button
                                type="button"
                                class="copy-btn"
                                data-copy=USDT
                                data-copied-label=move_tr!("copied")
                                aria-label=move_tr!("copy-address", { "coin" => "USDT" })
                            >{move_tr!("copy")}</button>
                        </div>
                    </div>
                </details>
            </div>
        </section>
    }
}

#[component]
pub(crate) fn Sponsors() -> impl IntoView {
    let sponsors_list = Resource::new(|| (), |_| async move { fetch_sponsors_list().await });

    view! {
        <section class="sec sponsors" id="sponsors" data-title="Sponsors">
            <div class="wrap">
                <Suspense fallback=|| ()>
                    {move || {
                        let sponsors = sponsors_list.get().unwrap_or_else(SponsorsList::fallback);
                        let tiers = sponsors.tiers();

                        view! {
                            <SponsorTier
                                label=move_tr!("sponsors-tier-special")
                                logins=tiers.special
                                size=56
                            />
                            <SponsorTier
                                label=move_tr!("sponsors-tier-monthly")
                                logins=tiers.monthly
                                size=36
                            />
                            <SponsorTier
                                label=move_tr!("sponsors-tier-one-time")
                                logins=tiers.one_time
                                size=36
                            />
                        }
                    }}
                </Suspense>
                <a
                    class="text-link"
                    href="https://github.com/sponsors/temidaradev"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    {move_tr!("sponsors-cta")}
                </a>
            </div>
        </section>
    }
}

#[component]
fn SponsorTier(label: Signal<String>, logins: Vec<String>, size: u32) -> impl IntoView {
    let avatars_class = if size > 36 {
        "sponsor-avatars sponsor-avatars-lg"
    } else {
        "sponsor-avatars"
    };

    (!logins.is_empty()).then(move || {
        let count = logins.len();
        view! {
            <div class="tier">
                <h2>
                    {label}
                    " "
                    <span class="tier-count tabular">{count}</span>
                </h2>
                <div class=avatars_class>
                    {logins
                        .into_iter()
                        .map(|login| sponsor_avatar(&login, size))
                        .collect_view()}
                </div>
            </div>
        }
    })
}

#[component]
pub(crate) fn Community() -> impl IntoView {
    view! {
        <section class="sec" id="community" data-title="Community">
            <div class="wrap">
                <div class="sec-head">
                    <h2>{move_tr!("community-title")}</h2>
                </div>
                <ul class="link-list">
                    <li>
                        <a
                            class="text-link"
                            href="https://discord.gg/K6Bmzw2E4M"
                            target="_blank"
                            rel="noopener noreferrer"
                        >"Discord"</a>
                        <p>{move_tr!("community-discord-desc")}</p>
                    </li>
                    <li>
                        <a
                            class="text-link"
                            href="https://github.com/Kopuz-org/kopuz/issues"
                            target="_blank"
                            rel="noopener noreferrer"
                        >{move_tr!("community-issues-title")}</a>
                        <p>{move_tr!("community-issues-desc")}</p>
                    </li>
                    <li>
                        <a
                            class="text-link"
                            href="https://github.com/Kopuz-org/kopuz/discussions"
                            target="_blank"
                            rel="noopener noreferrer"
                        >{move_tr!("community-discussions-title")}</a>
                        <p>{move_tr!("community-discussions-desc")}</p>
                    </li>
                    <li>
                        <a
                            class="text-link"
                            href="https://github.com/Kopuz-org/kopuz"
                            target="_blank"
                            rel="noopener noreferrer"
                        >{move_tr!("community-contribute-title")}</a>
                        <p>{move_tr!("community-contribute-desc")}</p>
                    </li>
                </ul>
            </div>
        </section>
    }
}
