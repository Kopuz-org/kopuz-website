use leptos::prelude::*;
use leptos_fluent::{move_tr, I18n};
use leptos_meta::Meta;
use leptos_router::hooks::{use_location, use_query_map};

use crate::icons::Icon;
use crate::releases::{provide_latest_release, use_latest_release};

const GITHUB: &str = "https://github.com/Kopuz-org/kopuz";
const BUTTON_SNIPPET: &str =
    "<a href=\"https://kopuz.moe\"><img src=\"https://kopuz.moe/88x31.svg\" alt=\"Kopuz\"></a>";

/// Runs before first paint so a saved theme never flashes the other palette.
/// The colors here must match `--ground` in style/_tokens.scss.
pub const THEME_BOOT_SCRIPT: &str = r##"
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
        ? "#121110"
        : theme === "moe"
            ? "#ffbfe6"
            : "#e7e2d6";

    const ua = `${navigator.userAgent} ${navigator.platform || ""}`.toLowerCase();
    const os = ua.includes("android") ? "android"
        : /iphone|ipad|ipod/.test(ua) ? null
        : ua.includes("mac") ? "macos"
        : ua.includes("win") ? "windows"
        : ua.includes("linux") || ua.includes("x11") ? "linux"
        : null;
    if (os) root.dataset.os = os;
})();
"##;

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
        "dark" => ("dark", "#121110"),
        "moe" => ("light", "#ffbfe6"),
        _ => ("light", "#e7e2d6"),
    };

    if let Some(root) = browser_document().and_then(|document| document.document_element()) {
        let _ = root.set_attribute("data-theme", theme);
        let _ = root.set_attribute(
            "style",
            &format!("color-scheme: {color_scheme}; background-color: {background};"),
        );
        // public/site.js waits on this before it writes into any text node the
        // framework owns; doing it earlier corrupts the hydration walk.
        let _ = root.set_attribute("data-hydrated", "1");
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
///
/// Every page component calls this first, so it is also where the shared
/// release resource is seeded: the shelf and the hero both read it, and they
/// are siblings with no common ancestor of their own.
pub(crate) fn provide_site_theme() -> SiteTheme {
    let query = use_query_map();
    let moe = query.with_untracked(|q| q.get("moe").is_some());
    let dark: RwSignal<bool> = RwSignal::new(false);
    let theme = SiteTheme { dark, moe };
    provide_context(theme);
    provide_context(MoeQuery(moe));
    provide_latest_release();

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
                "#121110"
            } else {
                "#e7e2d6"
            }
        />
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let theme = expect_context::<SiteTheme>();
    let leave_moe = move_tr!("theme-leave-moe");
    let use_light = move_tr!("theme-use-light");
    let use_dark = move_tr!("theme-use-dark");
    let label = move || {
        if theme.moe {
            leave_moe.get()
        } else if theme.dark.get() {
            use_light.get()
        } else {
            use_dark.get()
        }
    };

    view! {
        <button
            type="button"
            class="btn btn-ghost btn-icon theme-toggle"
            aria-label=label
            aria-pressed=move || (theme.dark.get() && !theme.moe).to_string()
            title=label
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
            {move || {
                let name = if theme.moe {
                    "x"
                } else if theme.dark.get() {
                    "sun"
                } else {
                    "moon"
                };
                view! { <Icon name=name/> }
            }}
        </button>
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
        <span class="nav-util lang-switch">
            <Icon name="languages" size=13/>
            <span class="lang-switch-value">{move || i18n.language.get().name}</span>
            <Icon name="chevron-down" size=13/>
            <select
                class="lang-select"
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
        </span>
    }
}

#[component]
pub(crate) fn Nav() -> impl IntoView {
    let pathname = use_location().pathname;
    let open = RwSignal::new(false);

    let close_menu = move_tr!("nav-menu-close");
    let open_menu = move_tr!("nav-menu-open");

    let home_href = internal_href("/");
    let features_href = internal_href("/features");
    let download_href = internal_href("/download");
    let guides_href = internal_href("/guides");
    let support_href = internal_href("/support");

    let link = move |href: String, path: &'static str, label: Signal<String>| {
        let active = move || {
            let current = pathname.get();
            current == path || (path != "/" && current.starts_with(&format!("{path}/")))
        };
        view! {
            <a
                href=href
                class="nav-link"
                class:nav-link-active=active
                aria-current=move || active().then_some("page")
                on:click=move |_| open.set(false)
            >{label}</a>
        }
    };

    view! {
        <nav class="nav" aria-label=move_tr!("nav-primary-aria")>
            <div class="wrap nav-inner">
                <a
                    href=home_href.clone()
                    class="nav-brand"
                    aria-current=move || (pathname.get() == "/").then_some("page")
                >
                    <img class="nav-mark" src="/logo.svg" alt="" width="24" height="24"/>
                    <span>"Kopuz"</span>
                </a>
                <div class="nav-links" class:open=move || open.get() id="nav-menu">
                    {link(home_href, "/", move_tr!("nav-home"))}
                    {link(features_href, "/features", move_tr!("nav-features"))}
                    {link(download_href, "/download", move_tr!("nav-download"))}
                    {link(guides_href, "/guides", move_tr!("guides-title"))}
                    {link(support_href, "/support", move_tr!("support-title"))}
                </div>
                <div class="nav-right">
                    <LanguageSwitcher/>
                    <ThemeToggle/>
                    <a class="nav-util nav-gh" href=GITHUB target="_blank" rel="noopener noreferrer">
                        "GitHub"
                        <Icon name="arrow-up-right" size=13/>
                    </a>
                    <button
                        type="button"
                        class="btn btn-ghost btn-icon nav-menu-btn"
                        aria-controls="nav-menu"
                        aria-expanded=move || open.get().to_string()
                        aria-label=move || if open.get() {
                            close_menu.get()
                        } else {
                            open_menu.get()
                        }
                        on:click=move |_| open.update(|value| *value = !*value)
                    >
                        {move || {
                            let name = if open.get() { "x" } else { "menu" };
                            view! { <Icon name=name size=18/> }
                        }}
                    </button>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub(crate) fn Footer() -> impl IntoView {
    let privacy_href = internal_href("/privacy");

    view! {
        <footer class="footer">
            <div class="wrap footer-inner">
                <div class="footer-brand">
                    <span class="footer-logo">"Kopuz"</span>
                    <span class="footer-license">{move_tr!("footer-license-line")}</span>
                </div>
                <nav class="footer-links" aria-label=move_tr!("footer-nav-aria")>
                    <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank">{move_tr!("footer-releases")}</a>
                    <a href="https://github.com/Kopuz-org/kopuz/issues" target="_blank">{move_tr!("footer-issues")}</a>
                    <a href="https://discord.gg/K6Bmzw2E4M" target="_blank">{move_tr!("footer-discord")}</a>
                    <a href=privacy_href>{move_tr!("footer-privacy")}</a>
                    <a href=GITHUB target="_blank">{move_tr!("footer-github")}</a>
                </nav>
            </div>
        </footer>
    }
}

#[component]
pub(crate) fn Shelf() -> impl IntoView {
    let button_href = "/88x31.png".to_string();
    let release = use_latest_release();
    let stamp = move || {
        let version = release
            .get()
            .map(|release| release.version().to_string())
            .unwrap_or_default();
        if version.is_empty() {
            "EUPL 1.2 · Rust".to_string()
        } else {
            format!("v{version} · EUPL 1.2 · Rust")
        }
    };

    view! {
        <div class="shelf">
            <div class="wrap shelf-inner">
                <a class="shelf-button" href=button_href>
                    <img
                        src="/88x31.png"
                        alt=move_tr!("webbutton-alt")
                        width="176"
                        height="62"
                        loading="lazy"
                    />
                </a>
                <p class="shelf-caption">
                    <span>{move_tr!("shelf-link-to-us")}</span>
                    <span class="shelf-stamp">
                        <Transition fallback=|| view! { <span>"EUPL 1.2 · Rust"</span> }>
                            {move || view! { <span>{stamp()}</span> }}
                        </Transition>
                    </span>
                </p>
                <div class="shelf-snip">
                    <pre>{BUTTON_SNIPPET}</pre>
                    <button
                        type="button"
                        class="shelf-copy"
                        data-copy=BUTTON_SNIPPET
                        data-copied-label=move_tr!("copied")
                    >{move_tr!("copy")}</button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub(crate) fn PlayerBar() -> impl IntoView {
    view! {
        <div class="player">
            <div class="player-now">
                <span class="player-thumb">
                    <img src="/logo.svg" alt="" width="26" height="26"/>
                </span>
                <p class="player-title">
                    <strong>"Kopuz"</strong>
                    <span class="player-section" data-player-section></span>
                </p>
                <button type="button" class="player-like" tabindex="-1" aria-hidden="true">
                    <Icon name="heart" size=15/>
                </button>
                <button type="button" class="player-play player-play-compact" tabindex="-1" aria-hidden="true">
                    <Icon name="pause" size=14/>
                </button>
            </div>
            <div class="player-center">
                <div class="player-transport" aria-hidden="true">
                    <button type="button" tabindex="-1"><Icon name="shuffle" size=15/></button>
                    <button type="button" class="player-step" tabindex="-1"><Icon name="skip-back" size=15/></button>
                    <button type="button" class="player-play" tabindex="-1"><Icon name="pause" size=14/></button>
                    <button type="button" class="player-step" tabindex="-1"><Icon name="skip-forward" size=15/></button>
                    <button type="button" tabindex="-1"><Icon name="repeat" size=15/></button>
                </div>
                <div class="player-seek">
                    <span data-player-elapsed>"0:00"</span>
                    <span class="player-track"><i data-player-fill></i></span>
                    <span data-player-total>"0:00"</span>
                </div>
            </div>
            <div class="player-side" aria-hidden="true">
                <span class="player-volume">
                    <Icon name="volume-2" size=15/>
                    <span class="player-vol-track"></span>
                </span>
                <button type="button" tabindex="-1"><Icon name="list-music" size=15/></button>
                <button type="button" tabindex="-1"><Icon name="maximize-2" size=14/></button>
            </div>
        </div>
    }
}
