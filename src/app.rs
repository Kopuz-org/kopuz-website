use fluent_templates::static_loader;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr, I18n};
use leptos_meta::{
    provide_meta_context, Html, Link, Meta, MetaTags, Script, Style, Stylesheet, Title,
};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::home::HomePage;
use crate::shell::{
    internal_href, provide_site_theme, simple_mode, Footer, Nav, PlayerBar, Shelf, ThemeColorMeta,
    SIMPLE_CSS, THEME_BOOT_SCRIPT,
};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

/// Every partial has to be listed here: the hash below is the CSS cache key,
/// and main.scss on its own is only a list of `@use` lines that never changes.
const STYLE_SOURCES: &[&str] = &[
    include_str!("../style/main.scss"),
    include_str!("../style/_tokens.scss"),
    include_str!("../style/_base.scss"),
    include_str!("../style/_shell.scss"),
    include_str!("../style/_home.scss"),
    include_str!("../style/_features.scss"),
    include_str!("../style/_download.scss"),
    include_str!("../style/_guides.scss"),
    include_str!("../style/_support.scss"),
    include_str!("../style/_privacy.scss"),
    include_str!("../style/_moe.scss"),
];

pub(crate) fn css_cache_bust() -> u64 {
    // FNV-1a hash of the SCSS source so cache key changes whenever styles change.
    STYLE_SOURCES
        .iter()
        .flat_map(|source| source.bytes())
        .fold(1469598103934665603u64, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(1099511628211)
        })
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script id="theme-init" inner_html=THEME_BOOT_SCRIPT></script>
                <link
                    rel="preload"
                    r#as="font"
                    type="font/woff2"
                    href="/fonts/jetbrains-mono-latin-400-normal.woff2"
                    crossorigin=""
                />
                <link
                    rel="preload"
                    r#as="font"
                    type="font/woff2"
                    href="/fonts/jetbrains-mono-latin-700-normal.woff2"
                    crossorigin=""
                />
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

    // Root metadata runs under an owner that does not inherit App's context
    // during client-side navigation, so every page takes I18n explicitly and
    // re-provides it over its own tree.
    let i18n = expect_context::<I18n>();

    view! {
        <Link rel="icon" href="/favicon.ico"/>
        <Meta name="author" content="temidaradev"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:image" content="https://kopuz.moe/banner.png"/>
        <Meta property="og:image:alt" content=move_tr!(i18n, "og-image-alt")/>
        <Meta property="og:site_name" content="Kopuz"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:image" content="https://kopuz.moe/banner.png"/>
        <Router>
            <RootDocument i18n=i18n/>
            <Routes fallback=move || view! { <NotFoundPage i18n=i18n/> }.into_view()>
                <Route
                    path=StaticSegment("")
                    view=move || view! { <HomePage i18n=i18n/> }
                />
                <Route
                    path=StaticSegment("features")
                    view=move || view! { <crate::pages::FeaturesPage i18n=i18n/> }
                />
                <Route
                    path=StaticSegment("download")
                    view=move || view! { <crate::pages::DownloadPage i18n=i18n/> }
                />
                <Route
                    path=StaticSegment("guides")
                    view=move || view! { <crate::pages::GuidesPage i18n=i18n/> }
                />
                <Route
                    path=(StaticSegment("guides"), ParamSegment("service"))
                    view=move || view! { <crate::pages::GuidePage i18n=i18n/> }
                />
                <Route
                    path=StaticSegment("support")
                    view=move || view! { <crate::pages::SupportPage i18n=i18n/> }
                />
                <Route path=StaticSegment("j") view=move || view! { <JoinPage i18n=i18n/> }/>
                <Route
                    path=StaticSegment("privacy")
                    view=move || view! { <crate::privacy::PrivacyPage i18n=i18n/> }
                />
            </Routes>
        </Router>
    }
}

/// The `<html>` attributes, the stylesheet and the page scripts.
///
/// This sits inside the router because simple mode is a query parameter and
/// `shell()` renders before there is a router to read it from. Simple mode
/// takes its own inline stylesheet and none of the site one.
#[component]
fn RootDocument(i18n: I18n) -> impl IntoView {
    let simple = simple_mode();
    let css_version = css_cache_bust();

    view! {
        <Html
            attr:lang=move || i18n.language.get().id.to_string()
            attr:dir=move || i18n.language.get().dir.as_str()
            attr:data-simple=simple.then_some("1")
        />
        {if simple {
            view! { <Style>{SIMPLE_CSS}</Style> }.into_any()
        } else {
            view! {
                <Stylesheet id="leptos" href=format!("/pkg/kopuz-website.css?v={css_version}")/>
                <Script src="/site.js" defer=""/>
            }.into_any()
        }}
    }
}

/// Bounce a "Listen on Kopuz" link into the desktop app.
///
/// Discord only accepts http/https in a Rich Presence button, so the button
/// points here and this hands off to `kopuz://`. The queue rides in the URL
/// **fragment**, which browsers never send to the server. Despite this being an
/// SSR app, nobody's queue ever reaches the box or its logs. Everything below
/// runs client-side for the same reason: there is nothing to render on the
/// server, because the server cannot see the payload.
#[component]
fn JoinPage(i18n: I18n) -> impl IntoView {
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
        // error to catch. Assume failure after a beat and offer the download
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
        <Provider value=i18n>
            <Title text=move_tr!(i18n, "join-title")/>
            // A one-shot handoff link is worthless in an index.
            <Meta name="robots" content="noindex, nofollow"/>
            <main>
            <section class="sec join">
                <div class="wrap">
                    <h1>{move_tr!(i18n, "join-opening")}</h1>
                    <Show when=move || took_too_long.get()>
                        <p class="prose">
                            {move || if payload.get().is_some() {
                                move_tr!(i18n, "join-fallback")
                            } else {
                                move_tr!(i18n, "join-no-payload")
                            }}
                        </p>
                        <a href=internal_href("/download") class="btn btn-primary">{move_tr!(i18n, "join-download")}</a>
                    </Show>
                </div>
            </section>
            </main>
        </Provider>
    }
}

/// The 404 body on its own, so `/guides/:service` can serve it for a slug that
/// has no guide without swapping out the page chrome.
#[component]
pub fn NotFoundBody() -> impl IntoView {
    let home_href = internal_href("/");

    view! {
        <section class="sec not-found" data-title="Not found">
            <div class="wrap">
                <h1>{move_tr!("notfound-title")}</h1>
                <p class="prose">{move_tr!("notfound-desc")}</p>
                <a class="btn btn-ghost" href=home_href>{move_tr!("notfound-home")}</a>
            </div>
        </section>
    }
}

#[component]
fn NotFoundPage(i18n: I18n) -> impl IntoView {
    let theme = provide_site_theme();
    let simple = simple_mode();

    view! {
        <Provider value=i18n>
            <Title text=move_tr!(i18n, "notfound-page-title")/>
            <Meta name="robots" content="noindex, follow"/>
            <ThemeColorMeta/>
            <div
                class="site page"
                class:simple=simple
                class:light=move || !theme.dark.get() && !theme.moe
                class:dark=move || theme.dark.get() && !theme.moe
                class:moe=move || theme.moe
            >
                <Nav/>
                <main>
                    <NotFoundBody/>
                </main>
                <Footer/>
                <Shelf/>
            </div>
            <PlayerBar/>
        </Provider>
    }
}

#[component]
pub(crate) fn AboutName() -> impl IntoView {
    view! {
        <section class="sec about-name" id="about-name" data-title="The name">
            <div class="wrap">
                <h2>{move_tr!("about-title")}</h2>
                <p class="prose">{move_tr!("about-desc-1")}</p>
                <p class="prose">{move_tr!("about-desc-2")}</p>
            </div>
        </section>
    }
}
