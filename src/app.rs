use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr, I18n};
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::{use_location, use_query_map},
    StaticSegment,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use std::collections::HashSet;

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

const STYLE_SOURCE: &str = include_str!("../style/main.scss");

const THEME_BOOT_SCRIPT: &str = r##"
(function () {
    const root = document.documentElement;
    const readCookie = (name) => {
        const prefix = `${name}=`;
        for (const part of document.cookie.split(";")) {
            const cookie = part.trim();
            if (cookie.startsWith(prefix)) return cookie.slice(prefix.length);
        }
        return null;
    };

    let theme;
    if (new URLSearchParams(window.location.search).has("moe")) {
        theme = "moe";
    } else {
        const saved = readCookie("kopuz-theme");
        const legacy = readCookie("kopuz-moe");
        theme = saved === "dark" || (saved === null && legacy === "0")
            ? "dark"
            : saved === "light" || (saved === null && legacy === "1")
                ? "light"
                : window.matchMedia("(prefers-color-scheme: dark)").matches
                    ? "dark"
                    : "light";
    }

    root.dataset.theme = theme;
    root.style.colorScheme = theme === "dark" ? "dark" : "light";
    root.style.backgroundColor = theme === "dark"
        ? "#17140f"
        : theme === "moe"
            ? "#ffbfe6"
            : "#f4f0e8";
})();
"##;

fn css_cache_bust() -> u64 {
    // FNV-1a hash of the SCSS source so cache key changes whenever styles change.
    STYLE_SOURCE
        .bytes()
        .fold(1469598103934665603u64, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(1099511628211)
        })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SponsorStats {
    monthly_goal: u32,
    pub(crate) current_monthly_income: u32,
    current_sponsors: u32,
    progress_percent: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SponsorsList {
    pub(crate) current: Vec<String>,
    pub(crate) past: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ReleaseNotes {
    tag_name: String,
    name: String,
    published_at: String,
    body: String,
}

impl ReleaseNotes {
    fn fallback() -> Self {
        Self {
            tag_name: "Latest release".into(),
            name: "Kopuz release notes".into(),
            published_at: String::new(),
            body: "Release notes are temporarily unavailable. Please check again soon.".into(),
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    published_at: Option<String>,
    body: Option<String>,
}

async fn fetch_latest_release() -> ReleaseNotes {
    #[cfg(feature = "ssr")]
    {
        let client = reqwest::Client::builder()
            .user_agent("kopuz-website/1.0")
            .build();
        if let Ok(client) = client {
            let response = client
                .get("https://api.github.com/repos/Kopuz-org/kopuz/releases/latest")
                .header("Accept", "application/vnd.github+json")
                .send()
                .await;
            if let Ok(response) = response {
                if let Ok(body) = response.text().await {
                    if let Ok(release) = serde_json::from_str::<GitHubRelease>(&body) {
                        return ReleaseNotes {
                            name: release.name.unwrap_or_else(|| release.tag_name.clone()),
                            tag_name: release.tag_name,
                            published_at: release.published_at.unwrap_or_default(),
                            body: release.body.unwrap_or_default(),
                        };
                    }
                }
            }
        }
    }

    ReleaseNotes::fallback()
}

#[derive(Debug, PartialEq, Eq)]
enum ReleaseInline {
    Text(String),
    Strong(String),
    Code(String),
    Link { label: String, href: String },
}

#[derive(Debug, PartialEq, Eq)]
enum ReleaseBlock {
    Heading2(Vec<ReleaseInline>),
    Heading3(Vec<ReleaseInline>),
    Heading4(Vec<ReleaseInline>),
    Paragraph(Vec<ReleaseInline>),
    UnorderedList(Vec<Vec<ReleaseInline>>),
}

#[derive(Clone, Copy)]
enum InlineMarker {
    Strong,
    Code,
    Link,
}

fn safe_release_link(href: &str) -> bool {
    href.starts_with("https://") || href.starts_with("http://")
}

fn parse_release_link(text: &str) -> Option<(&str, &str, usize)> {
    let label_end = text.find("](")?;
    let href_start = label_end + 2;
    let href_end = href_start + text[href_start..].find(')')?;
    let href = &text[href_start..href_end];
    safe_release_link(href).then_some((&text[1..label_end], href, href_end + 1))
}

fn parse_release_inlines(text: &str) -> Vec<ReleaseInline> {
    let mut inlines = Vec::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        let next_marker = [
            remaining
                .find("**")
                .map(|offset| (offset, InlineMarker::Strong)),
            remaining
                .find('`')
                .map(|offset| (offset, InlineMarker::Code)),
            remaining
                .find('[')
                .map(|offset| (offset, InlineMarker::Link)),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(offset, _)| *offset);

        let Some((offset, marker)) = next_marker else {
            inlines.push(ReleaseInline::Text(remaining.to_string()));
            break;
        };

        if offset > 0 {
            inlines.push(ReleaseInline::Text(remaining[..offset].to_string()));
            remaining = &remaining[offset..];
            continue;
        }

        match marker {
            InlineMarker::Strong => {
                if let Some(end) = remaining[2..].find("**") {
                    let end = end + 2;
                    inlines.push(ReleaseInline::Strong(remaining[2..end].to_string()));
                    remaining = &remaining[end + 2..];
                } else {
                    inlines.push(ReleaseInline::Text("**".to_string()));
                    remaining = &remaining[2..];
                }
            }
            InlineMarker::Code => {
                if let Some(end) = remaining[1..].find('`') {
                    let end = end + 1;
                    inlines.push(ReleaseInline::Code(remaining[1..end].to_string()));
                    remaining = &remaining[end + 1..];
                } else {
                    inlines.push(ReleaseInline::Text("`".to_string()));
                    remaining = &remaining[1..];
                }
            }
            InlineMarker::Link => {
                if let Some((label, href, consumed)) = parse_release_link(remaining) {
                    inlines.push(ReleaseInline::Link {
                        label: label.to_string(),
                        href: href.to_string(),
                    });
                    remaining = &remaining[consumed..];
                } else {
                    inlines.push(ReleaseInline::Text("[".to_string()));
                    remaining = &remaining[1..];
                }
            }
        }
    }

    inlines
}

fn flush_release_paragraph(blocks: &mut Vec<ReleaseBlock>, lines: &mut Vec<String>) {
    if !lines.is_empty() {
        blocks.push(ReleaseBlock::Paragraph(parse_release_inlines(
            &std::mem::take(lines).join(" "),
        )));
    }
}

fn flush_release_list(blocks: &mut Vec<ReleaseBlock>, items: &mut Vec<String>) {
    if !items.is_empty() {
        blocks.push(ReleaseBlock::UnorderedList(
            std::mem::take(items)
                .into_iter()
                .map(|item| parse_release_inlines(&item))
                .collect(),
        ));
    }
}

fn parse_release_markdown(body: &str) -> Vec<ReleaseBlock> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut list = Vec::new();

    for line in body.lines().map(str::trim) {
        if line == "## What's Changed" || line == "## What’s Changed" {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            break;
        }
        if line.is_empty() {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
        } else if let Some(text) = line.strip_prefix("#### ") {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            blocks.push(ReleaseBlock::Heading4(parse_release_inlines(text)));
        } else if let Some(text) = line.strip_prefix("### ") {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            blocks.push(ReleaseBlock::Heading3(parse_release_inlines(text)));
        } else if let Some(text) = line.strip_prefix("## ").or_else(|| line.strip_prefix("# ")) {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            blocks.push(ReleaseBlock::Heading2(parse_release_inlines(text)));
        } else if let Some(text) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            list.push(text.to_string());
        } else if let Some(text) = line
            .strip_prefix("**")
            .and_then(|text| text.strip_suffix("**"))
            .filter(|text| !text.contains("**"))
        {
            flush_release_paragraph(&mut blocks, &mut paragraph);
            flush_release_list(&mut blocks, &mut list);
            blocks.push(ReleaseBlock::Heading4(parse_release_inlines(text)));
        } else {
            flush_release_list(&mut blocks, &mut list);
            paragraph.push(line.to_string());
        }
    }

    flush_release_paragraph(&mut blocks, &mut paragraph);
    flush_release_list(&mut blocks, &mut list);
    blocks
}

fn release_inlines_view(inlines: Vec<ReleaseInline>) -> impl IntoView {
    inlines
        .into_iter()
        .map(|inline| match inline {
            ReleaseInline::Text(text) => view! { <span>{text}</span> }.into_any(),
            ReleaseInline::Strong(text) => view! { <strong>{text}</strong> }.into_any(),
            ReleaseInline::Code(text) => view! { <code>{text}</code> }.into_any(),
            ReleaseInline::Link { label, href } => view! {
                <a href=href target="_blank" rel="noopener noreferrer">{label}</a>
            }
            .into_any(),
        })
        .collect_view()
}

#[cfg(test)]
mod release_markdown_tests {
    use super::*;

    #[test]
    fn groups_paragraph_lines_and_list_items() {
        let blocks = parse_release_markdown(
            "First line\ncontinues here.\n\n## Highlights\n\n- One\n- Two\n\n## What's Changed\n- Hidden",
        );

        assert!(matches!(&blocks[0], ReleaseBlock::Paragraph(_)));
        assert!(matches!(&blocks[1], ReleaseBlock::Heading2(_)));
        assert!(matches!(&blocks[2], ReleaseBlock::UnorderedList(items) if items.len() == 2));
        assert_eq!(blocks.len(), 3);
    }

    #[test]
    fn preserves_safe_inline_markdown() {
        assert_eq!(
            parse_release_inlines("**Fast.** Uses `cache` and [notes](https://example.com)."),
            vec![
                ReleaseInline::Strong("Fast.".into()),
                ReleaseInline::Text(" Uses ".into()),
                ReleaseInline::Code("cache".into()),
                ReleaseInline::Text(" and ".into()),
                ReleaseInline::Link {
                    label: "notes".into(),
                    href: "https://example.com".into(),
                },
                ReleaseInline::Text(".".into()),
            ]
        );
    }
}

impl SponsorStats {
    fn fallback() -> Self {
        let monthly_goal = 400;
        let current_monthly_income = 32;
        let current_sponsors = 10;
        let progress_percent = (current_monthly_income * 100) / monthly_goal;

        Self {
            monthly_goal,
            current_monthly_income,
            current_sponsors,
            progress_percent,
        }
    }
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

    fn special_sponsors() -> impl Iterator<Item = &'static str> {
        SPECIAL_SPONSORS.iter().copied()
    }

    fn regular_one_time(&self) -> impl Iterator<Item = &String> {
        self.past
            .iter()
            .filter(|login| !Self::is_special_one_time(login))
    }

    fn fallback() -> Self {
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

async fn fetch_sponsors_list() -> SponsorsList {
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
            let current_sponsors = store.current.len() as u32;
            let progress_percent = (current_monthly_income * 100) / monthly_goal;

            return SponsorStats {
                monthly_goal,
                current_monthly_income,
                current_sponsors,
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

                    let current_sponsors =
                        parse_uint_after_marker(&body, "Current sponsors").unwrap_or(0);

                    let current_monthly_income = (monthly_goal * progress_percent) / 100;

                    if monthly_goal > 0 {
                        return SponsorStats {
                            monthly_goal,
                            current_monthly_income,
                            current_sponsors,
                            progress_percent,
                        };
                    }
                }
            }
        }
    }

    SponsorStats::fallback()
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script id="theme-init" inner_html=THEME_BOOT_SCRIPT></script>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
                <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Press+Start+2P&family=VT323&display=swap"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let css_version = css_cache_bust();

    leptos_fluent! {
        translations: [TRANSLATIONS],
        languages: "./locales/languages.json",
        locales: "./locales",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lf-lang",
        cookie_attrs: "SameSite=Strict; Path=/",
        initial_language_from_cookie: true,
        set_language_to_cookie: true,
        initial_language_from_url_param: true,
        url_param: "lang",
        initial_language_from_accept_language_header: true,
        initial_language_from_navigator: true,
    };

    view! {
        <Stylesheet id="leptos" href=format!("/pkg/kopuz-website.css?v={css_version}")/>
        <Link rel="icon" href="/favicon.ico"/>
        <Meta name="author" content="temidaradev"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:image" content="https://kopuz.moe/banner.png"/>
        <Meta property="og:image:alt" content=move_tr!("og-image-alt")/>
        <Meta property="og:site_name" content="Kopuz"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:image" content="https://kopuz.moe/banner.png"/>
        <Router>
            <Routes fallback=|| view! { <NotFoundPage/> }.into_view()>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("features") view=crate::pages::FeaturesPage/>
                <Route path=StaticSegment("download") view=crate::pages::DownloadPage/>
                <Route path=StaticSegment("guides") view=crate::pages::GuidesPage/>
                <Route path=StaticSegment("support") view=crate::pages::SupportPage/>
                <Route path=StaticSegment("j") view=JoinPage/>
                <Route path=StaticSegment("privacy") view=crate::privacy::PrivacyPage/>
            </Routes>
        </Router>
    }
}

/// Bounce a "Listen on Kopuz" link into the desktop app.
///
/// Discord only accepts http/https in a Rich Presence button, so the button
/// points here and this hands off to `kopuz://`. The queue rides in the URL
/// **fragment**, which browsers never send to the server — so despite this being
/// an SSR app, nobody's queue ever reaches the box or its logs. Everything below
/// runs client-side for the same reason: there is nothing to render on the
/// server, because the server cannot see the payload.
#[component]
fn JoinPage() -> impl IntoView {
    // `None` until the effect runs: on the server, and on first paint, we
    // genuinely don't know yet whether there's a payload.
    let payload: RwSignal<Option<String>> = RwSignal::new(None);
    let took_too_long = RwSignal::new(false);

    Effect::new(move |_| {
        let hash = web_sys::window()
            .and_then(|w| w.location().hash().ok())
            .unwrap_or_default();
        let encoded = hash.trim_start_matches('#').to_string();
        if encoded.is_empty() {
            took_too_long.set(true);
            return;
        }
        payload.set(Some(encoded.clone()));

        // Navigating to an unregistered scheme fails silently, so there is no
        // error to catch — assume failure after a beat and offer the download
        // instead. A handoff that worked has already backgrounded this tab.
        if let Some(win) = web_sys::window() {
            let _ = win.location().set_href(&format!("kopuz://j/{encoded}"));
        }
        set_timeout(
            move || took_too_long.set(true),
            std::time::Duration::from_millis(1500),
        );
    });

    view! {
        <Title text=move_tr!("join-title")/>
        // A one-shot handoff link is worthless in an index.
        <Meta name="robots" content="noindex, nofollow"/>
        <main>
        <section class="join">
            <h1>{move_tr!("join-opening")}</h1>
            <Show when=move || took_too_long.get()>
                <p>
                    {move || if payload.get().is_some() {
                        move_tr!("join-fallback")
                    } else {
                        move_tr!("join-no-payload")
                    }}
                </p>
                <a href=internal_href("/download") class="btn-primary">{move_tr!("join-download")}</a>
            </Show>
        </section>
        </main>
    }
}

#[derive(Clone, Copy)]
enum StoredTheme {
    Current(bool),
    Legacy(bool),
}

fn browser_document() -> Option<web_sys::HtmlDocument> {
    use wasm_bindgen::JsCast;
    web_sys::window()?
        .document()?
        .dyn_into::<web_sys::HtmlDocument>()
        .ok()
}

fn apply_document_theme(theme: &str) {
    let (color_scheme, background) = match theme {
        "dark" => ("dark", "#17140f"),
        "moe" => ("light", "#ffbfe6"),
        _ => ("light", "#f4f0e8"),
    };

    if let Some(root) = browser_document().and_then(|document| document.document_element()) {
        let _ = root.set_attribute("data-theme", theme);
        let _ = root.set_attribute(
            "style",
            &format!("color-scheme: {color_scheme}; background-color: {background};"),
        );
    }
}

fn read_theme_cookie() -> Option<StoredTheme> {
    let cookies = browser_document()?.cookie().ok()?;
    if let Some(dark) =
        cookies
            .split(';')
            .find_map(|cookie| match cookie.trim().strip_prefix("kopuz-theme=")? {
                "dark" => Some(true),
                "light" => Some(false),
                _ => None,
            })
    {
        return Some(StoredTheme::Current(dark));
    }

    cookies
        .split(';')
        .find_map(|cookie| match cookie.trim().strip_prefix("kopuz-moe=")? {
            "0" => Some(StoredTheme::Legacy(true)),
            "1" => Some(StoredTheme::Legacy(false)),
            _ => None,
        })
}

fn write_theme_cookie(dark: bool) {
    if let Some(html_doc) = browser_document() {
        let _ = html_doc.set_cookie(&format!(
            "kopuz-theme={}; Path=/; Max-Age=31536000; SameSite=Strict",
            if dark { "dark" } else { "light" }
        ));
    }
}

fn clear_legacy_theme_cookie() {
    if let Some(html_doc) = browser_document() {
        let _ = html_doc.set_cookie("kopuz-moe=; Path=/; Max-Age=0; SameSite=Strict");
    }
}

fn query_without_moe(search: &str) -> String {
    let query = search
        .strip_prefix('?')
        .unwrap_or(search)
        .split('&')
        .filter(|part| {
            !part.is_empty()
                && part
                    .split_once('=')
                    .map_or(*part != "moe", |(name, _)| name != "moe")
        })
        .collect::<Vec<_>>()
        .join("&");

    if query.is_empty() {
        String::new()
    } else {
        format!("?{query}")
    }
}

fn leave_moe_mode() {
    if let Some(window) = web_sys::window() {
        let location = window.location();
        let path = location.pathname().unwrap_or_else(|_| "/".to_string());
        let search = location
            .search()
            .map(|search| query_without_moe(&search))
            .unwrap_or_default();
        let hash = location.hash().unwrap_or_default();
        let _ = location.set_href(&format!("{path}{search}{hash}"));
    }
}

#[derive(Clone, Copy)]
struct MoeQuery(bool);

#[derive(Clone, Copy)]
pub(crate) struct SiteTheme {
    pub(crate) dark: RwSignal<bool>,
    pub(crate) moe: bool,
}

pub(crate) fn internal_href(path: &str) -> String {
    let preserve_moe = use_context::<MoeQuery>()
        .map(|mode| mode.0)
        .unwrap_or(false);
    if !preserve_moe {
        return path.to_owned();
    }

    let (base, fragment) = match path.split_once('#') {
        Some((base, fragment)) => (base, Some(fragment)),
        None => (path, None),
    };
    let separator = if base.contains('?') { '&' } else { '?' };
    let mut href = format!("{base}{separator}moe");
    if let Some(fragment) = fragment {
        href.push('#');
        href.push_str(fragment);
    }
    href
}

/// Resolve the regular color scheme and the explicit `?moe` easter egg.
///
/// The regular theme follows a saved choice, then the operating-system color
/// scheme. `?moe` overrides both without changing the saved light/dark choice.
pub(crate) fn provide_site_theme() -> SiteTheme {
    let query = use_query_map();
    let moe = query.with_untracked(|q| q.get("moe").is_some());
    let dark: RwSignal<bool> = RwSignal::new(false);
    let theme = SiteTheme { dark, moe };
    provide_context(theme);
    provide_context(MoeQuery(moe));

    Effect::new(move |_| {
        if moe {
            apply_document_theme("moe");
            return;
        }
        if let Some(saved) = read_theme_cookie() {
            let (saved, migrate) = match saved {
                StoredTheme::Current(saved) => (saved, false),
                StoredTheme::Legacy(saved) => (saved, true),
            };
            dark.set(saved);
            apply_document_theme(if saved { "dark" } else { "light" });
            if migrate {
                write_theme_cookie(saved);
                clear_legacy_theme_cookie();
            }
            return;
        }

        let prefers_dark = web_sys::window()
            .and_then(|window| {
                window
                    .match_media("(prefers-color-scheme: dark)")
                    .ok()
                    .flatten()
            })
            .is_some_and(|query| query.matches());
        dark.set(prefers_dark);
        apply_document_theme(if prefers_dark { "dark" } else { "light" });
    });

    theme
}

#[component]
pub(crate) fn ThemeColorMeta() -> impl IntoView {
    let theme = expect_context::<SiteTheme>();
    view! {
        <Meta
            name="theme-color"
            content=move || if theme.moe {
                "#ffbfe6"
            } else if theme.dark.get() {
                "#17140f"
            } else {
                "#f4f0e8"
            }
        />
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let theme = provide_site_theme();
    view! {
        <Title text=move_tr!("home-title")/>
        <Meta name="description" content=move_tr!("home-meta-desc")/>
        <Meta name="keywords" content=move_tr!("home-meta-keywords")/>
        <Meta name="robots" content="index, follow"/>
        <Meta property="og:title" content=move_tr!("og-title")/>
        <Meta property="og:description" content=move_tr!("og-desc")/>
        <Meta property="og:url" content="https://kopuz.moe"/>
        <Meta name="twitter:title" content=move_tr!("twitter-title")/>
        <Meta name="twitter:description" content=move_tr!("twitter-desc")/>
        <Link rel="canonical" href="https://kopuz.moe"/>
        <ThemeColorMeta/>
        <div
            class="site"
            class:light=move || !theme.dark.get() && !theme.moe
            class:dark=move || theme.dark.get() && !theme.moe
            class:moe=move || theme.moe
        >
            <Nav/>
            <main>
                <Hero/>
                <HomeDirectory/>
                <WhatsNew/>
                <AboutName/>
            </main>
            <Footer/>
        </div>
    }
}

#[component]
fn HomeDirectory() -> impl IntoView {
    let features_href = internal_href("/features");
    let download_href = internal_href("/download");
    let guides_href = internal_href("/guides");
    let support_href = internal_href("/support");

    view! {
        <section class="home-directory" aria-label="Explore Kopuz">
            <a href=features_href>
                <h2>{move_tr!("features-title")}</h2>
                <p>{move_tr!("features-chip")}</p>
            </a>
            <a href=download_href>
                <h2>{move_tr!("platforms-title")}</h2>
                <p>{move_tr!("platforms-subtitle")}</p>
            </a>
            <a href=guides_href>
                <h2>{move_tr!("guides-title")}</h2>
                <p>{move_tr!("guides-subtitle")}</p>
            </a>
            <a href=support_href>
                <h2>{move_tr!("support-title")}</h2>
                <p>{move_tr!("support-subtitle")}</p>
            </a>
        </section>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    let theme = provide_site_theme();
    let home_href = internal_href("/");

    view! {
        <Title text="Page not found | Kopuz"/>
        <Meta name="robots" content="noindex, follow"/>
        <ThemeColorMeta/>
        <div
            class="site page"
            class:light=move || !theme.dark.get() && !theme.moe
            class:dark=move || theme.dark.get() && !theme.moe
            class:moe=move || theme.moe
        >
            <Nav/>
            <main class="not-found">
                <header class="page-intro">
                    <h1>"Page not found."</h1>
                    <p><a class="text-link" href=home_href>"Return to Kopuz"</a></p>
                </header>
            </main>
            <Footer/>
        </div>
    }
}

#[component]
fn WhatsNew() -> impl IntoView {
    let latest_release = Resource::new(|| (), |_| async move { fetch_latest_release().await });

    view! {
        <section class="content-section whats-new" id="whats-new">
            <div class="section-header">
                <h2>{move_tr!("new-title")}</h2>
                <p>{move_tr!("new-subtitle")}</p>
            </div>
            <Suspense fallback=|| view! { <div class="release-notes release-loading"><p>"Loading the latest release…"</p></div> }>
                {move || latest_release.get().map(|release| {
                    let date = release.published_at.get(..10).unwrap_or("").to_string();
                    let meta = if date.is_empty() { release.tag_name.clone() } else { format!("{} · {}", release.tag_name, date) };
                    let blocks = parse_release_markdown(&release.body);
                    view! {
                        <details class="release-notes">
                            <summary class="release-summary">
                                <span>
                                    <span class="release-version">{meta}</span>
                                    <strong class="release-name">{release.name}</strong>
                                </span>
                                <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                            </summary>
                            <div class="release-body">
                                {blocks.into_iter().map(|block| match block {
                                    ReleaseBlock::Heading2(inlines) => view! { <h3>{release_inlines_view(inlines)}</h3> }.into_any(),
                                    ReleaseBlock::Heading3(inlines) => view! { <h4>{release_inlines_view(inlines)}</h4> }.into_any(),
                                    ReleaseBlock::Heading4(inlines) => view! { <h4>{release_inlines_view(inlines)}</h4> }.into_any(),
                                    ReleaseBlock::Paragraph(inlines) => view! { <p>{release_inlines_view(inlines)}</p> }.into_any(),
                                    ReleaseBlock::UnorderedList(items) => view! {
                                        <ul>
                                            {items.into_iter().map(|item| view! {
                                                <li>{release_inlines_view(item)}</li>
                                            }).collect_view()}
                                        </ul>
                                    }.into_any(),
                                }).collect_view()}
                            </div>
                        </details>
                    }
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn DonationBanner() -> impl IntoView {
    let sponsor_stats = Resource::new(|| (), |_| async move { fetch_sponsor_stats().await });

    view! {
        <div class="donation-banner" role="group" aria-label="Support the developer">
            <div class="donation-banner-label">
                <i class="fa-solid fa-heart"></i>
                <span>"Support Notice"</span>
            </div>
            <p class="donation-banner-text">
                <strong>"Temidaradev here."</strong>
                " I am a student and I do not have a stable income. I work very hard on this project, and I need your help."
                " Please consider donating so I can raise "
                <strong class="donation-goal">"$400/month"</strong>
                " as general income support while I study and cover things I need to buy."
            </p>
            <div class="donation-banner-meta">
                <Suspense fallback=|| view! { <div class="donation-progress-wrap"></div> }>
                    {move || {
                        let stats = sponsor_stats
                            .get()
                            .unwrap_or_else(SponsorStats::fallback);
                        let bar_width = stats.progress_percent.min(100);

                        view! {
                            <div class="donation-progress-wrap">
                                <p class="donation-progress">
                                    <i class="fa-brands fa-github"></i>
                                    " GitHub Sponsors: "
                                    <strong>{format!("${}/{} per month", stats.current_monthly_income, stats.monthly_goal)}</strong>
                                    {format!(" ({}% goal, {} current sponsors)", stats.progress_percent, stats.current_sponsors)}
                                </p>
                                <div
                                    class="donation-progress-track"
                                    role="progressbar"
                                    aria-label="GitHub Sponsors goal progress"
                                    aria-valuemin="0"
                                    aria-valuemax="100"
                                    aria-valuenow=stats.progress_percent.to_string()
                                >
                                    <span class="donation-progress-fill" style=format!("width: {}%;", bar_width)></span>
                                </div>
                            </div>
                        }
                    }}
                </Suspense>
                <a
                    href="https://github.com/sponsors/temidaradev"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="donation-sponsor-link"
                >
                    <i class="fa-solid fa-heart"></i>
                    " Sponsor on GitHub"
                </a>
            </div>
        </div>
    }
}

#[component]
fn LanguageSwitcher() -> impl IntoView {
    let i18n = expect_context::<I18n>();
    let on_change = move |ev: leptos::ev::Event| {
        let v = event_target_value(&ev);
        if let Some(lang) = i18n.languages.iter().find(|l| l.id.to_string() == v) {
            i18n.language.set(lang);
        }
    };

    view! {
        <select
            class="nav-lang"
            aria-label=move_tr!("nav-lang-label")
            on:change=on_change
        >
            {i18n.languages.iter().map(|lang| {
                let lang_id = lang.id.to_string();
                let lang_id_cmp = lang_id.clone();
                let name = lang.name;
                view! {
                    <option
                        value=lang_id
                        selected=move || i18n.language.get().id.to_string() == lang_id_cmp
                    >
                        {name}
                    </option>
                }
            }).collect_view()}
        </select>
    }
}

#[component]
pub(crate) fn Nav() -> impl IntoView {
    let pathname = use_location().pathname;
    let home_href = internal_href("/");
    let features_href = internal_href("/features");
    let download_href = internal_href("/download");
    let guides_href = internal_href("/guides");
    let support_href = internal_href("/support");

    view! {
        <nav class="nav" aria-label="Primary navigation">
            <div class="nav-row">
                <a
                    href=home_href
                    class="nav-logo"
                    aria-current=move || (pathname.get() == "/").then_some("page")
                >
                    <img src="/logo.svg" alt="" width="26" height="26"/>"Kopuz"
                </a>
                <div class="nav-tabs">
                    <a
                        href=features_href
                        class="nav-tab"
                        class:nav-tab-active=move || pathname.get() == "/features"
                        aria-current=move || (pathname.get() == "/features").then_some("page")
                    >{move_tr!("nav-features")}</a>
                    <a
                        href=download_href
                        class="nav-tab"
                        class:nav-tab-active=move || pathname.get() == "/download"
                        aria-current=move || (pathname.get() == "/download").then_some("page")
                    >{move_tr!("nav-download")}</a>
                    <a
                        href=guides_href
                        class="nav-tab"
                        class:nav-tab-active=move || pathname.get() == "/guides"
                        aria-current=move || (pathname.get() == "/guides").then_some("page")
                    >{move_tr!("guides-title")}</a>
                    <a
                        href=support_href
                        class="nav-tab"
                        class:nav-tab-active=move || pathname.get() == "/support"
                        aria-current=move || (pathname.get() == "/support").then_some("page")
                    >{move_tr!("support-title")}</a>
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" class="nav-tab">{move_tr!("nav-github")}</a>
                  </div>
                  <div class="nav-controls">
                      <LanguageSwitcher/>
                      <ThemeToggle/>
                  </div>
            </div>
        </nav>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let theme = expect_context::<SiteTheme>();
    view! {
        <button
            type="button"
            class="theme-toggle"
            aria-label=move || if theme.moe {
                "Leave moe mode"
            } else if theme.dark.get() {
                "Use light theme"
            } else {
                "Use dark theme"
            }
            aria-pressed=move || (theme.dark.get() && !theme.moe).to_string()
            title=move || if theme.moe {
                "Leave moe mode"
            } else if theme.dark.get() {
                "Use light theme"
            } else {
                "Use dark theme"
            }
            on:click=move |_| {
                if theme.moe {
                    leave_moe_mode();
                    return;
                }
                let dark = !theme.dark.get_untracked();
                theme.dark.set(dark);
                write_theme_cookie(dark);
                apply_document_theme(if dark { "dark" } else { "light" });
            }
        >
            <i
                class=move || if theme.moe {
                    "fa-solid fa-book-open"
                } else if theme.dark.get() {
                    "fa-solid fa-sun"
                } else {
                    "fa-solid fa-moon"
                }
                aria-hidden="true"
            ></i>
        </button>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let download_href = internal_href("/download");

    view! {
        <section class="hero">
            <div class="hero-left">
                <h1>{move_tr!("hero-title-1")}<br/>{move_tr!("hero-title-2")}</h1>
                <p>{move_tr!("hero-desc")}</p>
                <div class="hero-ctas">
                    <a href=download_href class="btn-primary">{move_tr!("hero-cta-download")}</a>
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" class="btn-secondary">{move_tr!("hero-cta-github")}</a>
                </div>
            </div>
            <HeroScreenshot/>
        </section>
    }
}

#[component]
fn HeroScreenshot() -> impl IntoView {
    let expanded: RwSignal<bool> = RwSignal::new(false);

    view! {
        <div class="hero-right">
            <button
                type="button"
                class="hero-screenshot-button"
                aria-label=move_tr!("hero-screenshot-alt")
                on:click=move |_| expanded.set(true)
            >
                <img src="/normal-home.png" alt=move_tr!("hero-screenshot-alt") class="hero-screenshot"/>
            </button>
        </div>

        <Show when=move || expanded.get()>
            <div class="lightbox" on:click=move |_| expanded.set(false)>
                <div class="lightbox-box hero-lightbox-box" on:click=move |ev| ev.stop_propagation()>
                    <div class="lightbox-topbar">
                        <span class="lightbox-label">{move_tr!("hero-screenshot-alt")}</span>
                        <button class="lightbox-close" on:click=move |ev| { ev.stop_propagation(); expanded.set(false); } >"×"</button>
                    </div>
                    <img src="/normal-home.png" alt=move_tr!("hero-screenshot-alt") class="lightbox-img hero-lightbox-img"/>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub(crate) fn Features() -> impl IntoView {
    view! {
        <section class="features" id="features">
            <div class="section-header features-header">
                <h1>{move_tr!("features-title")}</h1>
                <p>{move_tr!("features-chip")}</p>
            </div>
            <div class="features-sources-bar">
                <span class="sources-label">{move_tr!("features-works-with")}</span>
                <div class="sources-list">
                    <span class="source-tag"><i class="fa-solid fa-folder-open"></i>" "{move_tr!("features-source-local")}</span>
                    <span class="source-tag"><i class="fa-solid fa-server"></i>" "{move_tr!("features-source-jellyfin")}</span>
                    <span class="source-tag"><i class="fa-solid fa-server"></i>" "{move_tr!("features-source-navidrome")}</span>
                    <span class="source-tag"><i class="fa-solid fa-satellite-dish"></i>" "{move_tr!("features-source-subsonic")}</span>
                    <span class="source-tag"><i class="fa-solid fa-cloud"></i>" "{move_tr!("features-source-nextcloud")}</span>
                    <span class="source-tag"><i class="fa-brands fa-youtube"></i>" "{move_tr!("features-source-ytmusic")}</span>
                    <span class="source-tag"><i class="fa-brands fa-apple"></i>" "{move_tr!("features-source-applemusic")}</span>
                    <span class="source-tag"><i class="fa-brands fa-soundcloud"></i>" "{move_tr!("features-source-soundcloud")}</span>
                    <span class="source-tag"><i class="fa-brands fa-spotify"></i>" "{move_tr!("features-source-spotify")}</span>
                </div>
            </div>
            <div class="features-grid features-featured">
                <FeatureCard icon="fa-solid fa-music" title_key="feat-local-title" desc_key="feat-local-desc"/>
                <FeatureCard icon="fa-brands fa-apple" title_key="feat-applemusic-title" desc_key="feat-applemusic-desc"/>
                <FeatureCard icon="fa-solid fa-cloud" title_key="feat-nextcloud-title" desc_key="feat-nextcloud-desc"/>
                <FeatureCard icon="fa-solid fa-align-left" title_key="feat-lyrics-title" desc_key="feat-lyrics-desc"/>
                <FeatureCard icon="fa-solid fa-sliders" title_key="feat-eq-title" desc_key="feat-eq-desc"/>
                <FeatureCard icon="fa-solid fa-star" title_key="feat-fav-title" desc_key="feat-fav-desc"/>
                <FeatureCard icon="fa-solid fa-palette" title_key="feat-theming-title" desc_key="feat-theming-desc"/>
                <FeatureCard icon="fa-solid fa-display" title_key="feat-native-title" desc_key="feat-native-desc"/>
                <FeatureCard icon="fa-brands fa-android" title_key="feat-android-title" desc_key="feat-android-desc"/>
            </div>
            <div class="features-compact">
                <FeatureItem icon="fa-solid fa-magnifying-glass" title_key="feat-search-title"/>
                <FeatureItem icon="fa-brands fa-youtube" title_key="feat-youtube-title"/>
                <FeatureItem icon="fa-brands fa-soundcloud" title_key="feat-soundcloud-title"/>
                <FeatureItem icon="fa-brands fa-spotify" title_key="feat-spotify-title"/>
                <FeatureItem icon="fa-solid fa-tower-broadcast" title_key="feat-scrobble-title"/>
                <FeatureItem icon="fa-solid fa-radio" title_key="feat-radio-title"/>
                <FeatureItem icon="fa-solid fa-cloud-arrow-down" title_key="feat-offline-title"/>
                <FeatureItem icon="fa-solid fa-font" title_key="feat-fonts-title"/>
                <FeatureItem icon="fa-brands fa-discord" title_key="feat-discord-title"/>
                <FeatureItem icon="fa-solid fa-tags" title_key="feat-genre-title"/>
                <FeatureItem icon="fa-solid fa-clock" title_key="feat-logs-title"/>
                <FeatureItem icon="fa-solid fa-globe" title_key="feat-i18n-title"/>
                <FeatureItem icon="fa-solid fa-download" title_key="feat-ytdlp-title"/>
                <FeatureItem icon="fa-solid fa-shuffle" title_key="feat-crossfade-title"/>
                <FeatureItem icon="fa-solid fa-headphones" title_key="feat-channels-title"/>
                <FeatureItem icon="fa-solid fa-image" title_key="feat-metadata-title"/>
                <FeatureItem icon="fa-solid fa-file-lines" title_key="feat-debug-title"/>
                <FeatureItem icon="fa-solid fa-broom" title_key="feat-cleanup-title"/>
                <FeatureItem icon="fa-solid fa-window-minimize" title_key="feat-miniplayer-title"/>
                <FeatureItem icon="fa-solid fa-inbox" title_key="feat-tray-title"/>
                <FeatureItem icon="fa-solid fa-file-audio" title_key="feat-badges-title"/>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    #[prop(into)] icon: String,
    #[prop(into)] title_key: &'static str,
    #[prop(into)] desc_key: &'static str,
) -> impl IntoView {
    let title = feature_title(title_key);
    let desc = feature_desc(desc_key);
    view! {
        <div class="feature-card">
            <i class=format!("feature-icon {icon}")></i>
            <h3>{title}</h3>
            <p>{desc}</p>
        </div>
    }
}

#[component]
fn FeatureItem(#[prop(into)] icon: String, #[prop(into)] title_key: &'static str) -> impl IntoView {
    let title = feature_title(title_key);
    view! {
        <div class="feature-item">
            <i class=format!("{icon}")></i>
            <span>{title}</span>
        </div>
    }
}
fn feature_title(key: &'static str) -> Signal<String> {
    match key {
        "feat-local-title" => move_tr!("feat-local-title"),
        "feat-theming-title" => move_tr!("feat-theming-title"),
        "feat-native-title" => move_tr!("feat-native-title"),
        "feat-android-title" => move_tr!("feat-android-title"),
        "feat-lyrics-title" => move_tr!("feat-lyrics-title"),
        "feat-eq-title" => move_tr!("feat-eq-title"),
        "feat-fav-title" => move_tr!("feat-fav-title"),
        "feat-scrobble-title" => move_tr!("feat-scrobble-title"),
        "feat-radio-title" => move_tr!("feat-radio-title"),
        "feat-offline-title" => move_tr!("feat-offline-title"),
        "feat-fonts-title" => move_tr!("feat-fonts-title"),
        "feat-discord-title" => move_tr!("feat-discord-title"),
        "feat-search-title" => move_tr!("feat-search-title"),
        "feat-genre-title" => move_tr!("feat-genre-title"),
        "feat-logs-title" => move_tr!("feat-logs-title"),
        "feat-i18n-title" => move_tr!("feat-i18n-title"),
        "feat-ytdlp-title" => move_tr!("feat-ytdlp-title"),
        "feat-crossfade-title" => move_tr!("feat-crossfade-title"),
        "feat-channels-title" => move_tr!("feat-channels-title"),
        "feat-youtube-title" => move_tr!("feat-youtube-title"),
        "feat-applemusic-title" => move_tr!("feat-applemusic-title"),
        "feat-nextcloud-title" => move_tr!("feat-nextcloud-title"),
        "feat-metadata-title" => move_tr!("feat-metadata-title"),
        "feat-debug-title" => move_tr!("feat-debug-title"),
        "feat-cleanup-title" => move_tr!("feat-cleanup-title"),
        "feat-soundcloud-title" => move_tr!("feat-soundcloud-title"),
        "feat-spotify-title" => move_tr!("feat-spotify-title"),
        "feat-miniplayer-title" => move_tr!("feat-miniplayer-title"),
        "feat-tray-title" => move_tr!("feat-tray-title"),
        "feat-badges-title" => move_tr!("feat-badges-title"),
        _ => Signal::derive(|| String::new()),
    }
}

fn feature_desc(key: &'static str) -> Signal<String> {
    match key {
        "feat-local-desc" => move_tr!("feat-local-desc"),
        "feat-theming-desc" => move_tr!("feat-theming-desc"),
        "feat-native-desc" => move_tr!("feat-native-desc"),
        "feat-android-desc" => move_tr!("feat-android-desc"),
        "feat-lyrics-desc" => move_tr!("feat-lyrics-desc"),
        "feat-eq-desc" => move_tr!("feat-eq-desc"),
        "feat-fav-desc" => move_tr!("feat-fav-desc"),
        "feat-scrobble-desc" => move_tr!("feat-scrobble-desc"),
        "feat-radio-desc" => move_tr!("feat-radio-desc"),
        "feat-offline-desc" => move_tr!("feat-offline-desc"),
        "feat-fonts-desc" => move_tr!("feat-fonts-desc"),
        "feat-discord-desc" => move_tr!("feat-discord-desc"),
        "feat-search-desc" => move_tr!("feat-search-desc"),
        "feat-genre-desc" => move_tr!("feat-genre-desc"),
        "feat-logs-desc" => move_tr!("feat-logs-desc"),
        "feat-i18n-desc" => move_tr!("feat-i18n-desc"),
        "feat-ytdlp-desc" => move_tr!("feat-ytdlp-desc"),
        "feat-crossfade-desc" => move_tr!("feat-crossfade-desc"),
        "feat-channels-desc" => move_tr!("feat-channels-desc"),
        "feat-youtube-desc" => move_tr!("feat-youtube-desc"),
        "feat-applemusic-desc" => move_tr!("feat-applemusic-desc"),
        "feat-nextcloud-desc" => move_tr!("feat-nextcloud-desc"),
        "feat-metadata-desc" => move_tr!("feat-metadata-desc"),
        "feat-debug-desc" => move_tr!("feat-debug-desc"),
        "feat-cleanup-desc" => move_tr!("feat-cleanup-desc"),
        "feat-soundcloud-desc" => move_tr!("feat-soundcloud-desc"),
        "feat-spotify-desc" => move_tr!("feat-spotify-desc"),
        "feat-miniplayer-desc" => move_tr!("feat-miniplayer-desc"),
        "feat-tray-desc" => move_tr!("feat-tray-desc"),
        "feat-badges-desc" => move_tr!("feat-badges-desc"),
        _ => Signal::derive(|| String::new()),
    }
}

#[component]
pub(crate) fn Performance() -> impl IntoView {
    view! {
        <section class="perf disclosure-section" id="performance">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">{move_tr!("perf-title")}</span>
                        <span class="disclosure-description">{move_tr!("perf-subtitle")}</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="perf-grid disclosure-body">
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-skip-label")}</span>
                    <p>{move_tr!("perf-skip-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-parallel-label")}</span>
                    <p>{move_tr!("perf-parallel-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-art-label")}</span>
                    <p>{move_tr!("perf-art-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-lazy-label")}</span>
                    <p>{move_tr!("perf-lazy-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-io-label")}</span>
                    <p>{move_tr!("perf-io-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-http-label")}</span>
                    <p>{move_tr!("perf-http-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-sort-label")}</span>
                    <p>{move_tr!("perf-sort-desc")}</p>
                </div>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn Install() -> impl IntoView {
    view! {
        <section class="install disclosure-section" id="install">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">{move_tr!("install-title")}</span>
                        <span class="disclosure-description">{move_tr!("install-quick-note")}</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="install-grid disclosure-body">
                <div class="install-card">
                    <h3>{move_tr!("install-quick-title")}</h3>
                    <p>{move_tr!("install-quick-desc")}</p>
                    <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="btn-secondary">{move_tr!("install-quick-cta")}</a>
                    <p class="install-note">{move_tr!("install-quick-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-cargo-title")}</h3>
                    <p>{move_tr!("install-cargo-desc")}</p>
                    <pre><code>"cargo install --locked kopuz"</code></pre>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-nix-title")}</h3>
                    <p>{move_tr!("install-nix-run")}</p>
                    <pre><code>"nix run github:temidaradev/kopuz"</code></pre>
                    <p>{move_tr!("install-nix-profile")}</p>
                    <pre><code>"nix profile add github:temidaradev/kopuz"</code></pre>
                    <p class="install-note">{move_tr!("install-nix-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-aur-title")}</h3>
                    <p>{move_tr!("install-aur-desc")}</p>
                    <pre><code>"yay -S kopuz-bin
# or
paru -S kopuz-bin"</code></pre>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-flatpak-title")}</h3>
                    <p>{move_tr!("install-flatpak-desc")}</p>
                    <pre><code>"flatpak install flathub moe.kopuz.kopuz"</code></pre>
                    <p class="install-note">{move_tr!("install-flatpak-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-appimage-title")}</h3>
                    <p>{move_tr!("install-appimage-desc-1")}" "<code>"webkit2gtk-4.1"</code>{move_tr!("install-appimage-desc-2")}" "<code>"gtk3"</code>{move_tr!("install-appimage-desc-3")}</p>
                    <p class="install-note">{move_tr!("install-appimage-note-1")}" "<code>"LD_LIBRARY_PATH=/usr/lib"</code>{move_tr!("install-appimage-note-2")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-macos-title")}" "<span class="install-chip">{move_tr!("install-macos-chip")}</span></h3>
                    <p>{move_tr!("install-macos-homebrew")}</p>
                    <pre><code>"brew install --cask --no-quarantine kopuz-org/tap/kopuz"</code></pre>
                    <p>{move_tr!("install-macos-desc-1")}" "<code>".dmg"</code>{move_tr!("install-macos-desc-2")}</p>
                    <pre><code>"xattr -d com.apple.quarantine /Applications/Kopuz.app"</code></pre>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-android-title")}" "<span class="install-chip">{move_tr!("install-android-chip")}</span></h3>
                    <p>{move_tr!("install-android-desc")}</p>
                    <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="btn-secondary">{move_tr!("install-quick-cta")}</a>
                    <p class="install-note">{move_tr!("install-android-note")}</p>
                </div>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn YtMusic() -> impl IntoView {
    view! {
        <section class="install disclosure-section" id="ytmusic">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">{move_tr!("ytmusic-title")}</span>
                        <span class="disclosure-description">{move_tr!("ytmusic-subtitle")}</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="install-grid disclosure-body">
                <div class="install-card">
                    <h3>{move_tr!("ytmusic-token-title")}</h3>
                    <p>{move_tr!("ytmusic-token-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("ytmusic-signin-title")}</h3>
                    <p>{move_tr!("ytmusic-signin-desc")}</p>
                    <p class="install-note">{move_tr!("ytmusic-signin-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("ytmusic-anon-title")}</h3>
                    <p>{move_tr!("ytmusic-anon-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("ytmusic-premium-title")}</h3>
                    <p>{move_tr!("ytmusic-premium-desc")}</p>
                </div>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn NextcloudGuide() -> impl IntoView {
    view! {
        <section class="install disclosure-section" id="nextcloud">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">{move_tr!("nextcloud-title")}</span>
                        <span class="disclosure-description">{move_tr!("nextcloud-subtitle")}</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="install-grid disclosure-body">
                <div class="install-card">
                    <h3>{move_tr!("nextcloud-connect-title")}</h3>
                    <p>{move_tr!("nextcloud-connect-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("nextcloud-folders-title")}</h3>
                    <p>{move_tr!("nextcloud-folders-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("nextcloud-webdav-title")}</h3>
                    <p>{move_tr!("nextcloud-webdav-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("nextcloud-playback-title")}</h3>
                    <p>{move_tr!("nextcloud-playback-desc")}</p>
                </div>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn AppleMusicGuide() -> impl IntoView {
    view! {
        <section class="install disclosure-section" id="applemusic">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">{move_tr!("applemusic-title")}</span>
                        <span class="disclosure-description">{move_tr!("applemusic-subtitle")}</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="install-grid disclosure-body">
                <div class="install-card">
                    <h3>{move_tr!("applemusic-signin-title")}</h3>
                    <p>{move_tr!("applemusic-signin-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("applemusic-playback-title")}</h3>
                    <p>{move_tr!("applemusic-playback-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("applemusic-features-title")}</h3>
                    <p>{move_tr!("applemusic-features-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("applemusic-android-title")}</h3>
                    <p>{move_tr!("applemusic-android-desc")}</p>
                </div>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn SoundCloud() -> impl IntoView {
    view! {
        <section class="install disclosure-section" id="soundcloud">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">{move_tr!("soundcloud-title")}</span>
                        <span class="disclosure-description">{move_tr!("soundcloud-subtitle")}</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="install-grid disclosure-body">
                <div class="install-card">
                    <h3>{move_tr!("soundcloud-signin-title")}</h3>
                    <p>{move_tr!("soundcloud-signin-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("soundcloud-features-title")}</h3>
                    <p>{move_tr!("soundcloud-features-desc")}</p>
                </div>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn SpotifyGuide() -> impl IntoView {
    view! {
        <section class="content-section guide-section disclosure-section" id="spotify">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">{move_tr!("spotify-guide-title")}</span>
                        <span class="disclosure-description">{move_tr!("spotify-guide-subtitle")}</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="disclosure-body">
                    <div class="steps-grid">
                <article class="step-card"><span>"01"</span><h3>{move_tr!("spotify-step-1-title")}</h3><p>{move_tr!("spotify-step-1-desc")}</p><code>"http://127.0.0.1:8898/callback"</code></article>
                <article class="step-card"><span>"02"</span><h3>{move_tr!("spotify-step-2-title")}</h3><p>{move_tr!("spotify-step-2-desc")}</p></article>
                <article class="step-card"><span>"03"</span><h3>{move_tr!("spotify-step-3-title")}</h3><p>{move_tr!("spotify-step-3-desc")}</p></article>
                    </div>
                    <div class="callout"><i class="fa-solid fa-circle-info"></i><p>{move_tr!("spotify-requirement")}</p></div>
                    <a class="text-link" href="https://github.com/Kopuz-org/kopuz#spotify-setup" target="_blank">{move_tr!("spotify-full-guide")}</a>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn Privacy() -> impl IntoView {
    view! {
        <section class="content-section" id="privacy">
            <div class="section-header"><h2>{move_tr!("privacy-title")}</h2><p>{move_tr!("privacy-subtitle")}</p></div>
            <div class="privacy-grid">
                <article><i class="fa-solid fa-database"></i><h3>{move_tr!("privacy-local-title")}</h3><p>{move_tr!("privacy-local-desc")}</p></article>
                <article><i class="fa-solid fa-key"></i><h3>{move_tr!("privacy-accounts-title")}</h3><p>{move_tr!("privacy-accounts-desc")}</p></article>
                <article><i class="fa-solid fa-folder-tree"></i><h3>{move_tr!("privacy-files-title")}</h3><p>{move_tr!("privacy-files-desc")}</p></article>
            </div>
            <details class="paths-details"><summary>{move_tr!("privacy-paths-title")}</summary><div><p><strong>"Linux: "</strong><code>"~/.config/kopuz/kopuz.db"</code></p><p><strong>"macOS: "</strong><code>"~/Library/Application Support/moe.kopuz.kopuz/kopuz.db"</code></p><p><strong>"Windows: "</strong><code>"%APPDATA%\\kopuz\\kopuz\\config\\kopuz.db"</code></p></div></details>
        </section>
    }
}

#[component]
pub(crate) fn Requirements() -> impl IntoView {
    view! {
        <section class="content-section disclosure-section" id="requirements">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">{move_tr!("requirements-title")}</span>
                        <span class="disclosure-description">{move_tr!("requirements-subtitle")}</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="requirements-list disclosure-body">
                <div><strong>"Spotify"</strong><span>"Premium is required for playback; a personal Client ID and supported browser are required."</span></div>
                <div><strong>"Apple Music"</strong><span>"Desktop playback requires a Widevine CDM. Sign-in works on Android, but playback is not yet supported there."</span></div>
                <div><strong>"Nextcloud"</strong><span>"Raw WebDAV has no playlists or radio; prefer a Subsonic endpoint from Nextcloud Music when available."</span></div>
                <div><strong>"Android"</strong><span>"Release APKs require arm64-v8a and Android 7.0 / API 24 or newer. Desktop integrations such as Discord RPC and the system tray are unavailable."</span></div>
                <div><strong>"AppImage"</strong><span>"Requires webkit2gtk-4.1 and GTK 3. The tray additionally needs an appindicator library."</span></div>
                <div><strong>"YouTube Music"</strong><span>"Anonymous mode cannot play Premium-only tracks; yt-dlp can help signed-in playback fallbacks."</span></div>
                <div><strong>"Crossfade"</strong><span>"Available for native desktop playback; browser-owned Spotify audio uses normal transitions."</span></div>
                <div><strong>"Spotify limits"</strong><span>"Development Mode limits search, makes playlists read-only, and disables downloads, radio, tag editing, and Kopuz audio effects."</span></div>
                </div>
            </details>
        </section>
    }
}

#[component]
fn AboutName() -> impl IntoView {
    view! {
        <section class="content-section about-name" id="about-name">
            <div class="about-mark"><img src="/logo.svg" alt=""/></div>
            <div>
                <h2>{move_tr!("about-title")}</h2>
                <p>{move_tr!("about-desc-1")}</p>
                <p>{move_tr!("about-desc-2")}</p>
            </div>
        </section>
    }
}

#[component]
pub(crate) fn Community() -> impl IntoView {
    view! {
        <section class="content-section" id="community">
            <div class="section-header"><h2>{move_tr!("community-title")}</h2><p>{move_tr!("community-subtitle")}</p></div>
            <div class="community-grid">
                <a href="https://github.com/Kopuz-org/kopuz/issues" target="_blank"><i class="fa-solid fa-bug"></i><div><h3>{move_tr!("community-issues-title")}</h3><p>{move_tr!("community-issues-desc")}</p></div></a>
                <a href="https://github.com/Kopuz-org/kopuz/discussions" target="_blank"><i class="fa-regular fa-comments"></i><div><h3>{move_tr!("community-discussions-title")}</h3><p>{move_tr!("community-discussions-desc")}</p></div></a>
                <a href="https://discord.gg/K6Bmzw2E4M" target="_blank"><i class="fa-brands fa-discord"></i><div><h3>"Discord"</h3><p>{move_tr!("community-discord-desc")}</p></div></a>
                <a href="https://github.com/Kopuz-org/kopuz" target="_blank"><i class="fa-solid fa-code-branch"></i><div><h3>{move_tr!("community-contribute-title")}</h3><p>{move_tr!("community-contribute-desc")}</p></div></a>
            </div>
        </section>
    }
}

#[component]
pub(crate) fn Platforms() -> impl IntoView {
    view! {
        <section class="platforms" id="downloads">
            <div class="section-header">
                <h1>{move_tr!("platforms-title")}</h1>
                <p>{move_tr!("platforms-subtitle")}</p>
            </div>
            <div class="platform-grid">
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-windows platform-os-icon"></i>
                        <span class="platform-name">{move_tr!("platforms-windows")}</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".exe"</span>
                    </div>
                    <span class="platform-dl">{move_tr!("platforms-download")}</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-apple platform-os-icon"></i>
                        <span class="platform-name">{move_tr!("platforms-macos")}</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".dmg"</span>
                    </div>
                    <span class="platform-note">{move_tr!("platforms-macos-note")}</span>
                    <span class="platform-dl">{move_tr!("platforms-download")}</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-linux platform-os-icon"></i>
                        <span class="platform-name">{move_tr!("platforms-linux")}</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".AppImage"</span>
                        <span class="platform-fmt">".deb"</span>
                        <span class="platform-fmt">".rpm"</span>
                        <span class="platform-fmt">"Flatpak"</span>
                        <span class="platform-fmt">"Nix"</span>
                    </div>
                    <span class="platform-dl">{move_tr!("platforms-download")}</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-android platform-os-icon"></i>
                        <span class="platform-name">{move_tr!("platforms-android")}</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".apk"</span>
                    </div>
                    <span class="platform-note">{move_tr!("platforms-android-note")}</span>
                    <span class="platform-dl">{move_tr!("platforms-apk")}</span>
                </a>
            </div>
        </section>
    }
}

#[component]
pub(crate) fn Support() -> impl IntoView {
    view! {
        <section class="support" id="support">
            <div class="section-header">
                <h1>{move_tr!("support-title")}</h1>
                <p>{move_tr!("support-subtitle")}</p>
            </div>
            <DonationBanner/>
            <div class="support-links">
                <a href="https://buymeacoffee.com/temidaradev" target="_blank" class="support-btn support-bmc">
                    <i class="fa-solid fa-mug-hot"></i>
                    {move_tr!("support-bmc")}
                </a>
            </div>
            <details class="crypto-details">
                <summary class="donate-divider">{move_tr!("support-crypto-divider")}</summary>
            <div class="donate-grid">
                <div class="donate-item">
                    <span class="donate-coin">"SOL"</span>
                    <code>"2fapJYRztnTRLpJbmyEUnsuZ36AzLK2JrMmmLEfDqKpN"</code>
                </div>
                <div class="donate-item">
                    <span class="donate-coin">"BTC"</span>
                    <code>"bc1qz94yz9xvufa6hxlvjzaajgd2zyfu86arn68hu4"</code>
                </div>
                <div class="donate-item">
                    <span class="donate-coin">"XMR"</span>
                    <code>"86mz3HxTrKyYpuvx78m6pufbXdwAnoyoZBztz6HyYrnM1XP5YVrMy9jTVRY5vzgGtkizACLpFwHEdafKTMoj6y8mAVgvWMz"</code>
                </div>
                <div class="donate-item">
                    <span class="donate-coin">"ETH"</span>
                    <code>"0xa490D50470cdFf837B6663F7f6cBe50B157224e5"</code>
                </div>
                <div class="donate-item">
                    <span class="donate-coin">"USDT"</span>
                    <code>"GYmnAcrA5MbF6cUxT2m5d5cwdfr14qSY9WFYRwXxaibW"</code>
                    <span class="donate-note">{move_tr!("support-usdt-note")}</span>
                </div>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn Sponsors() -> impl IntoView {
    let sponsors_list = Resource::new(|| (), |_| async move { fetch_sponsors_list().await });

    view! {
        <section class="sponsors" id="sponsors">
            <div class="section-header">
                <h2>{move_tr!("sponsors-title")}</h2>
                <p>{move_tr!("sponsors-subtitle")}</p>
            </div>
            <Suspense fallback=|| view! { <div class="sponsors-grid"></div> }>
            {move || {
                let sponsors = sponsors_list
                    .get()
                    .unwrap_or_else(SponsorsList::fallback);
                let special_sponsors = SponsorsList::special_sponsors().map(|username| {
                    let profile = format!("https://github.com/{username}");
                    let avatar = format!("https://github.com/{username}.png?size=80");
                    let alt = username.to_string();
                    let name = username.to_string();
                    view! {
                        <a href=profile target="_blank" class="sponsor-card sponsor-special">
                            <img src=avatar alt=alt/>
                            <span>{name}</span>
                        </a>
                    }
                });

                view! {
                    <div class="sponsors-tier sponsors-special">
                        <h3 class="sponsors-section-title">"Special Sponsors"</h3>
                        <div class="sponsors-grid">
                            {special_sponsors.collect_view()}
                        </div>
                    </div>
                    <div class="sponsors-tier sponsors-monthly">
                        <h3 class="sponsors-section-title">{format!("Monthly Sponsors ({})", sponsors.current.len())}</h3>
                        <div class="sponsors-grid">
                            {sponsors.current.iter().map(|username| {
                                let profile = format!("https://github.com/{username}");
                                let avatar = format!("https://github.com/{username}.png?size=80");
                                let alt = username.clone();
                                let name = username.clone();
                                view! {
                                    <a href=profile target="_blank" class="sponsor-card sponsor-monthly">
                                        <img src=avatar alt=alt/>
                                        <span>{name}</span>
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                    <div class="sponsors-tier sponsors-one-time">
                        <h3 class="sponsors-section-title">{format!("One-time Sponsors ({})", sponsors.regular_one_time().count())}</h3>
                        <div class="sponsors-grid">
                            {sponsors.regular_one_time().map(|username| {
                                let profile = format!("https://github.com/{username}");
                                let avatar = format!("https://github.com/{username}.png?size=80");
                                let alt = username.clone();
                                let name = username.clone();
                                view! {
                                    <a href=profile target="_blank" class="sponsor-card sponsor-one-time">
                                        <img src=avatar alt=alt/>
                                        <span>{name}</span>
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                }
            }}
            </Suspense>
            <div class="sponsors-cta">
                <a href="https://github.com/sponsors/temidaradev" target="_blank" class="btn-secondary">{move_tr!("sponsors-cta")}</a>
            </div>
        </section>
    }
}

#[component]
pub(crate) fn WebButton() -> impl IntoView {
    // Classic 88x31 "link back" button — the old-web tradition. The SVG and PNG
    // live in /public and are served from a stable, absolute URL so anyone can
    // embed the button on their own site from anywhere.
    const SITE: &str = "https://kopuz.moe";
    let home_href = internal_href("/");

    let html_embed = format!(
        "<a href=\"{SITE}\"><img src=\"{SITE}/88x31.svg\" width=\"88\" height=\"31\" alt=\"Kopuz\"></a>"
    );
    let markdown_embed = format!("[![Kopuz]({SITE}/88x31.svg)]({SITE})");
    let bbcode_embed = format!("[url={SITE}][img]{SITE}/88x31.svg[/img][/url]");

    view! {
        <section class="webbutton disclosure-section" id="button">
            <details>
                <summary class="disclosure-summary">
                    <h2 class="disclosure-copy">
                        <span class="disclosure-title">"Put Kopuz on your site"</span>
                        <span class="disclosure-description">"A classic 88" {"\u{00d7}"} "31 link-back button."</span>
                    </h2>
                    <i class="fa-solid fa-chevron-down disclosure-icon"></i>
                </summary>
                <div class="webbutton-body disclosure-body">
                <div class="webbutton-preview">
                    <a href=home_href aria-label="Kopuz home">
                        <img src="/88x31.svg" width="88" height="31" alt="Kopuz 88x31 button" class="webbutton-img"/>
                    </a>
                    <span class="webbutton-note">
                        "SVG " <a href="/88x31.svg" target="_blank">"/88x31.svg"</a>
                        " " {"\u{2022}"} " PNG " <a href="/88x31.png" target="_blank">"/88x31.png"</a>
                    </span>
                </div>
                <div class="webbutton-codes">
                    <div class="webbutton-code">
                        <span class="webbutton-code-label">"HTML"</span>
                        <pre><code>{html_embed}</code></pre>
                    </div>
                    <div class="webbutton-code">
                        <span class="webbutton-code-label">"Markdown"</span>
                        <pre><code>{markdown_embed}</code></pre>
                    </div>
                    <div class="webbutton-code">
                        <span class="webbutton-code-label">"BBCode"</span>
                        <pre><code>{bbcode_embed}</code></pre>
                    </div>
                </div>
                </div>
            </details>
        </section>
    }
}

#[component]
pub(crate) fn Footer() -> impl IntoView {
    let privacy_href = internal_href("/privacy");
    let button_href = internal_href("/support#button");

    view! {
        <footer class="footer">
            <div class="footer-left">
                <span class="footer-logo">"Kopuz"</span>
                <span>{move_tr!("footer-license")}</span>
            </div>
            <div class="footer-links">
                <a href=privacy_href>{move_tr!("footer-privacy")}</a>
                <a href=button_href>"88" {"\u{00d7}"} "31 Button"</a>
                <a href="https://github.com/Kopuz-org/kopuz" target="_blank">{move_tr!("footer-github")}</a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank">{move_tr!("footer-releases")}</a>
                <a href="https://github.com/Kopuz-org/kopuz/issues" target="_blank">{move_tr!("footer-issues")}</a>
                <a href="https://discord.gg/K6Bmzw2E4M" target="_blank">{move_tr!("footer-discord")}</a>
            </div>
        </footer>
    }
}
