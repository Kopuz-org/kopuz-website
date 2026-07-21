use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr, I18n};
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_query_map,
    StaticSegment,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

const STYLE_SOURCE: &str = include_str!("../style/main.scss");

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

impl SponsorsList {
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
            past: ["Iamknownasfesal", "arda2k3", "bulakemun", "AniviaFlome", "SeriousPassenger", ]
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
            };
        }
    }

    fetch_sponsors_list_via_scrape().await
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
                    let current_section = slice_between(&body, "Current sponsors", "</remote-pagination>");
                    let past_section = slice_between(&body, "Past sponsors", "</remote-pagination>");

                    if let (Some(current_section), Some(past_section)) = (current_section, past_section) {
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

                    let current_sponsors = parse_uint_after_marker(&body, "Current sponsors")
                        .unwrap_or(0);

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
        <Title text=move_tr!("home-title")/>
        <Meta name="description" content=move_tr!("home-meta-desc")/>
        <Meta name="keywords" content=move_tr!("home-meta-keywords")/>
        <Meta name="author" content="temidaradev"/>
        <Meta name="robots" content="index, follow"/>
        <Meta name="theme-color" content="#17140f"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content=move_tr!("og-title")/>
        <Meta property="og:description" content=move_tr!("og-desc")/>
        <Meta property="og:url" content="https://kopuz.temidara.rocks"/>
        <Meta property="og:image" content="https://kopuz.temidara.rocks/banner.png"/>
        <Meta property="og:image:alt" content=move_tr!("og-image-alt")/>
        <Meta property="og:site_name" content="Kopuz"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=move_tr!("twitter-title")/>
        <Meta name="twitter:description" content=move_tr!("twitter-desc")/>
        <Meta name="twitter:image" content="https://kopuz.temidara.rocks/banner.png"/>
        <Link rel="canonical" href="https://kopuz.temidara.rocks"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

fn read_moe_cookie() -> Option<bool> {
    use wasm_bindgen::JsCast;
    let html_doc = web_sys::window()?
        .document()?
        .dyn_into::<web_sys::HtmlDocument>()
        .ok()?;
    let cookies = html_doc.cookie().ok()?;
    cookies
        .split(';')
        .find_map(|c| c.trim().strip_prefix("kopuz-moe=").map(|v| v == "1"))
}

fn write_moe_cookie(value: bool) {
    use wasm_bindgen::JsCast;
    if let Some(html_doc) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.dyn_into::<web_sys::HtmlDocument>().ok())
    {
        let _ = html_doc.set_cookie(&format!(
            "kopuz-moe={}; Path=/; Max-Age=31536000; SameSite=Strict",
            if value { "1" } else { "0" }
        ));
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Theme priority: `?moe` in the URL (shareable easter egg) > saved cookie
    // (explicit toggle choice) > system color scheme (dark stays modern,
    // light gets moe). The nav toggle flips it and saves the choice.
    let query = use_query_map();
    let initial_moe = query.with_untracked(|q| q.get("moe").is_some());
    let moe: RwSignal<bool> = RwSignal::new(initial_moe);
    provide_context(moe);

    // Effects only run on the client, after hydration — SSR always renders
    // dark, then the effect settles the real theme on first paint.
    Effect::new(move |_| {
        if initial_moe {
            return;
        }
        if let Some(saved) = read_moe_cookie() {
            moe.set(saved);
            return;
        }
        if let Some(win) = web_sys::window() {
            if let Ok(Some(mql)) = win.match_media("(prefers-color-scheme: light)") {
                if mql.matches() {
                    moe.set(true);
                }
            }
        }
    });

    view! {
        <div class="site" class:moe=move || moe.get()>
            <DonationBanner/>
            <Nav/>
            <Hero/>
            <Features/>
            <Performance/>
            <Install/>
            <YtMusic/>
            <SoundCloud/>
            <Platforms/>
            <Support/>
            <Sponsors/>
            <Footer/>
        </div>
    }
}

#[component]
fn DonationBanner() -> impl IntoView {
    let sponsor_stats = Resource::new(|| (), |_| async move { fetch_sponsor_stats().await });

    view! {
        <section class="donation-banner" aria-label="Support the developer">
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
        </section>
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
fn Nav() -> impl IntoView {
    view! {
        <nav class="nav">
            <div class="nav-row">
                <a href="/" class="nav-logo"><img src="/logo.svg" alt="" width="26" height="26"/>"Kopuz"</a>
                <div class="nav-tabs">
                    <a href="/#features" class="nav-tab">{move_tr!("nav-features")}</a>
                    <a href="/#install" class="nav-tab">{move_tr!("nav-install")}</a>
                    <a href="/#downloads" class="nav-tab">{move_tr!("nav-download")}</a>
                    <a href="/#sponsors" class="nav-tab">{move_tr!("nav-sponsors")}</a>
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" class="nav-tab">{move_tr!("nav-github")}</a>
                    <LanguageSwitcher/>
                    <ThemeToggle/>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let moe = expect_context::<RwSignal<bool>>();
    view! {
        <button
            type="button"
            class="theme-toggle"
            aria-label="Toggle theme"
            title="Toggle theme (or add ?moe to the URL)"
            on:click=move |_| {
                moe.update(|m| *m = !*m);
                write_moe_cookie(moe.get_untracked());
            }
        >
            <i class=move || if moe.get() { "fa-solid fa-sun" } else { "fa-solid fa-moon" }></i>
        </button>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-left">
                <h1>{move_tr!("hero-title-1")}<br/>{move_tr!("hero-title-2")}</h1>
                <p>{move_tr!("hero-desc")}</p>
                <div class="hero-ctas">
                    <a href="#downloads" class="btn-primary">{move_tr!("hero-cta-download")}</a>
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
fn Features() -> impl IntoView {
    view! {
        <section class="features" id="features">
            <div class="features-grid">
                <div class="features-sources-bar">
                    <span class="sources-label">{move_tr!("features-works-with")}</span>
                    <div class="sources-list">
                        <span class="source-tag"><i class="fa-solid fa-folder-open"></i>" "{move_tr!("features-source-local")}</span>
                        <span class="source-tag"><i class="fa-solid fa-server"></i>" "{move_tr!("features-source-jellyfin")}</span>
                        <span class="source-tag"><i class="fa-solid fa-server"></i>" "{move_tr!("features-source-navidrome")}</span>
                        <span class="source-tag"><i class="fa-solid fa-satellite-dish"></i>" "{move_tr!("features-source-subsonic")}</span>
                        <span class="source-tag"><i class="fa-brands fa-youtube"></i>" "{move_tr!("features-source-ytmusic")}</span>
                        <span class="source-tag"><i class="fa-brands fa-soundcloud"></i>" "{move_tr!("features-source-soundcloud")}</span>
                    </div>
                </div>
                <FeatureCard icon="fa-solid fa-music" title_key="feat-local-title" desc_key="feat-local-desc"/>
                <FeatureCard icon="fa-brands fa-youtube" title_key="feat-youtube-title" desc_key="feat-youtube-desc"/>
                <FeatureCard icon="fa-brands fa-soundcloud" title_key="feat-soundcloud-title" desc_key="feat-soundcloud-desc"/>
                <FeatureCard icon="fa-solid fa-palette" title_key="feat-theming-title" desc_key="feat-theming-desc"/>
                <FeatureCard icon="fa-solid fa-display" title_key="feat-native-title" desc_key="feat-native-desc"/>
                <FeatureCard icon="fa-solid fa-align-left" title_key="feat-lyrics-title" desc_key="feat-lyrics-desc"/>
                <FeatureCard icon="fa-solid fa-sliders" title_key="feat-eq-title" desc_key="feat-eq-desc"/>
                <FeatureCard icon="fa-solid fa-star" title_key="feat-fav-title" desc_key="feat-fav-desc"/>
                <FeatureCard icon="fa-solid fa-tower-broadcast" title_key="feat-scrobble-title" desc_key="feat-scrobble-desc"/>
                <FeatureCard icon="fa-brands fa-discord" title_key="feat-discord-title" desc_key="feat-discord-desc"/>
                <FeatureCard icon="fa-solid fa-tags" title_key="feat-genre-title" desc_key="feat-genre-desc"/>
                <FeatureCard icon="fa-solid fa-clock" title_key="feat-logs-title" desc_key="feat-logs-desc"/>
                <FeatureCard icon="fa-solid fa-globe" title_key="feat-i18n-title" desc_key="feat-i18n-desc"/>
                <FeatureCard icon="fa-brands fa-youtube" title_key="feat-ytdlp-title" desc_key="feat-ytdlp-desc"/>
                <FeatureCard icon="fa-solid fa-shuffle" title_key="feat-crossfade-title" desc_key="feat-crossfade-desc"/>
                <FeatureCard icon="fa-solid fa-headphones" title_key="feat-channels-title" desc_key="feat-channels-desc"/>
                <FeatureCard icon="fa-solid fa-image" title_key="feat-metadata-title" desc_key="feat-metadata-desc"/>
                <FeatureCard icon="fa-solid fa-file-lines" title_key="feat-debug-title" desc_key="feat-debug-desc"/>
                <FeatureCard icon="fa-solid fa-broom" title_key="feat-cleanup-title" desc_key="feat-cleanup-desc"/>
                <FeatureCard icon="fa-solid fa-window-minimize" title_key="feat-miniplayer-title" desc_key="feat-miniplayer-desc"/>
                <FeatureCard icon="fa-solid fa-inbox" title_key="feat-tray-title" desc_key="feat-tray-desc"/>
                <FeatureCard icon="fa-solid fa-file-audio" title_key="feat-badges-title" desc_key="feat-badges-desc"/>
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

fn feature_title(key: &'static str) -> Signal<String> {
    match key {
        "feat-local-title" => move_tr!("feat-local-title"),
        "feat-theming-title" => move_tr!("feat-theming-title"),
        "feat-native-title" => move_tr!("feat-native-title"),
        "feat-lyrics-title" => move_tr!("feat-lyrics-title"),
        "feat-eq-title" => move_tr!("feat-eq-title"),
        "feat-fav-title" => move_tr!("feat-fav-title"),
        "feat-scrobble-title" => move_tr!("feat-scrobble-title"),
        "feat-discord-title" => move_tr!("feat-discord-title"),
        "feat-search-title" => move_tr!("feat-search-title"),
        "feat-genre-title" => move_tr!("feat-genre-title"),
        "feat-logs-title" => move_tr!("feat-logs-title"),
        "feat-i18n-title" => move_tr!("feat-i18n-title"),
        "feat-ytdlp-title" => move_tr!("feat-ytdlp-title"),
        "feat-crossfade-title" => move_tr!("feat-crossfade-title"),
        "feat-channels-title" => move_tr!("feat-channels-title"),
        "feat-youtube-title" => move_tr!("feat-youtube-title"),
        "feat-metadata-title" => move_tr!("feat-metadata-title"),
        "feat-debug-title" => move_tr!("feat-debug-title"),
        "feat-cleanup-title" => move_tr!("feat-cleanup-title"),
        "feat-soundcloud-title" => move_tr!("feat-soundcloud-title"),
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
        "feat-lyrics-desc" => move_tr!("feat-lyrics-desc"),
        "feat-eq-desc" => move_tr!("feat-eq-desc"),
        "feat-fav-desc" => move_tr!("feat-fav-desc"),
        "feat-scrobble-desc" => move_tr!("feat-scrobble-desc"),
        "feat-discord-desc" => move_tr!("feat-discord-desc"),
        "feat-search-desc" => move_tr!("feat-search-desc"),
        "feat-genre-desc" => move_tr!("feat-genre-desc"),
        "feat-logs-desc" => move_tr!("feat-logs-desc"),
        "feat-i18n-desc" => move_tr!("feat-i18n-desc"),
        "feat-ytdlp-desc" => move_tr!("feat-ytdlp-desc"),
        "feat-crossfade-desc" => move_tr!("feat-crossfade-desc"),
        "feat-channels-desc" => move_tr!("feat-channels-desc"),
        "feat-youtube-desc" => move_tr!("feat-youtube-desc"),
        "feat-metadata-desc" => move_tr!("feat-metadata-desc"),
        "feat-debug-desc" => move_tr!("feat-debug-desc"),
        "feat-cleanup-desc" => move_tr!("feat-cleanup-desc"),
        "feat-soundcloud-desc" => move_tr!("feat-soundcloud-desc"),
        "feat-miniplayer-desc" => move_tr!("feat-miniplayer-desc"),
        "feat-tray-desc" => move_tr!("feat-tray-desc"),
        "feat-badges-desc" => move_tr!("feat-badges-desc"),
        _ => Signal::derive(|| String::new()),
    }
}

#[component]
fn Performance() -> impl IntoView {
    view! {
        <section class="perf" id="performance">
            <div class="section-header">
                <h2>{move_tr!("perf-title")}</h2>
                <p>{move_tr!("perf-subtitle")}</p>
            </div>
            <div class="perf-grid">
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
        </section>
    }
}

#[component]
fn Install() -> impl IntoView {
    view! {
        <section class="install" id="install">
            <div class="section-header">
                <h2>{move_tr!("install-title")}</h2>
            </div>
            <div class="install-grid">
                <div class="install-card">
                    <h3>{move_tr!("install-quick-title")}</h3>
                    <p>{move_tr!("install-quick-desc")}</p>
                    <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="btn-secondary">{move_tr!("install-quick-cta")}</a>
                    <p class="install-note">{move_tr!("install-quick-note")}</p>
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
                    <pre><code>"yay -S kopuz
# or
paru -S kopuz"</code></pre>
                    <p class="install-note">{move_tr!("install-aur-note-1")}" "<code>"dioxus-cli"</code>{move_tr!("install-aur-note-2")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-flatpak-title")}</h3>
                    <p>{move_tr!("install-flatpak-desc")}</p>
                    <pre><code>"git clone https://github.com/temidaradev/kopuz
cd kopuz
flatpak-builder --user --install --force-clean \\
  build-dir packaging/flatpak/com.temidaradev.kopuz.json
flatpak run com.temidaradev.kopuz"</code></pre>
                    <p class="install-note">{move_tr!("install-flatpak-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-appimage-title")}</h3>
                    <p>{move_tr!("install-appimage-desc-1")}" "<code>"webkit2gtk-4.1"</code>{move_tr!("install-appimage-desc-2")}" "<code>"gtk3"</code>{move_tr!("install-appimage-desc-3")}</p>
                    <p class="install-note">{move_tr!("install-appimage-note-1")}" "<code>"LD_LIBRARY_PATH=/usr/lib"</code>{move_tr!("install-appimage-note-2")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-macos-title")}" "<span class="install-chip">{move_tr!("install-macos-chip")}</span></h3>
                    <p>{move_tr!("install-macos-desc-1")}" "<code>".dmg"</code>{move_tr!("install-macos-desc-2")}</p>
                    <pre><code>"xattr -d com.apple.quarantine /Applications/Kopuz.app"</code></pre>
                </div>
            </div>
        </section>
    }
}

#[component]
fn YtMusic() -> impl IntoView {
    view! {
        <section class="install" id="ytmusic">
            <div class="section-header">
                <h2>{move_tr!("ytmusic-title")}</h2>
                <p>{move_tr!("ytmusic-subtitle")}</p>
            </div>
            <div class="install-grid">
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
        </section>
    }
}

#[component]
fn SoundCloud() -> impl IntoView {
    view! {
        <section class="install" id="soundcloud">
            <div class="section-header">
                <h2>{move_tr!("soundcloud-title")}</h2>
                <p>{move_tr!("soundcloud-subtitle")}</p>
            </div>
            <div class="install-grid">
                <div class="install-card">
                    <h3>{move_tr!("soundcloud-signin-title")}</h3>
                    <p>{move_tr!("soundcloud-signin-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("soundcloud-features-title")}</h3>
                    <p>{move_tr!("soundcloud-features-desc")}</p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Platforms() -> impl IntoView {
    view! {
        <section class="platforms" id="downloads">
            <div class="section-header">
                <h2>{move_tr!("platforms-title")}</h2>
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
            </div>
        </section>
    }
}

#[component]
fn Support() -> impl IntoView {
    view! {
        <section class="support" id="support">
            <div class="section-header">
                <h2>{move_tr!("support-title")}</h2>
                <p>{move_tr!("support-subtitle")}</p>
            </div>
            <div class="support-links">
                <a href="https://github.com/sponsors/temidaradev" target="_blank" class="support-btn support-gh">
                    <i class="fa-solid fa-heart"></i>
                    {move_tr!("support-gh")}
                </a>
                <a href="https://buymeacoffee.com/temidaradev" target="_blank" class="support-btn support-bmc">
                    <i class="fa-solid fa-mug-hot"></i>
                    {move_tr!("support-bmc")}
                </a>
            </div>
            <div class="donate-divider">{move_tr!("support-crypto-divider")}</div>
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
        </section>
    }
}

#[component]
fn Sponsors() -> impl IntoView {
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

                view! {
                    <div class="sponsors-grid sponsors-monthly">
                        <h3 class="sponsors-section-title">{format!("Monthly Sponsors ({})", sponsors.current.len())}</h3>
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
                    <div class="sponsors-grid sponsors-one-time">
                        <h3 class="sponsors-section-title">{format!("One-time Sponsors ({})", sponsors.past.len())}</h3>
                        {sponsors.past.iter().map(|username| {
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
                }
            }}
            </Suspense>
            <div class="sponsors-cta">
                <a href="https://github.com/sponsors/temidaradev" target="_blank" class="btn-secondary">{move_tr!("sponsors-cta")}</a>
            </div>
        </section>
    }
}

static GALLERY_SRCS: &[&str] = &[
    "/normal-home.png",
    "/modern-home.png",
    "/normal-library.png",
    "/vaxry-library.png",
    "/normal-playlist.png",
    "/modern-playlist.png",
    "/fullscreen.png",
    "/fullscreen-lyrics.png",
    "/theme-editor.png",
    "/player-settings.png",
    "/downloader.png",
];

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer-left">
                <span class="footer-logo">"Kopuz"</span>
                <span>{move_tr!("footer-license")}</span>
            </div>
            <div class="footer-links">
                <a href="https://github.com/Kopuz-org/kopuz" target="_blank">{move_tr!("footer-github")}</a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank">{move_tr!("footer-releases")}</a>
                <a href="https://github.com/Kopuz-org/kopuz/issues" target="_blank">{move_tr!("footer-issues")}</a>
                <a href="https://discord.gg/K6Bmzw2E4M" target="_blank">{move_tr!("footer-discord")}</a>
            </div>
        </footer>
    }
}
