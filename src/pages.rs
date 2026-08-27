use leptos::context::Provider;
use leptos::prelude::*;
use leptos_fluent::{move_tr, I18n};
use leptos_meta::{Link, Meta, Title};
use leptos_router::hooks::use_params_map;

use crate::app::NotFoundBody;
use crate::download::{Install, Platforms, Requirements};
use crate::features::Features;
use crate::guides::{guide_by_slug, GuideArticle, GuideIndex};
use crate::shell::{provide_site_theme, Footer, Nav, PlayerBar, Shelf, ThemeColorMeta};
use crate::support::{Community, Sponsors, Support};

#[component]
fn PageMeta(
    title: Signal<String>,
    description: &'static str,
    canonical: &'static str,
) -> impl IntoView {
    view! {
        <Title text=title/>
        <Meta name="description" content=description/>
        <Meta name="robots" content="index, follow"/>
        <Meta property="og:title" content=title/>
        <Meta property="og:description" content=description/>
        <Meta property="og:url" content=canonical/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Link rel="canonical" href=canonical/>
    }
}

#[component]
fn PageChrome(children: Children) -> impl IntoView {
    let theme = provide_site_theme();

    view! {
        <ThemeColorMeta/>
        <div
            class="site page"
            class:light=move || !theme.dark.get() && !theme.moe
            class:dark=move || theme.dark.get() && !theme.moe
            class:moe=move || theme.moe
        >
            <Nav/>
            <main>{children()}</main>
            <Footer/>
            <Shelf/>
        </div>
        <PlayerBar/>
    }
}

#[component]
pub fn FeaturesPage(i18n: I18n) -> impl IntoView {
    view! {
        <Provider value=i18n>
            <PageMeta
                title=move_tr!(i18n, "features-page-title")
                description="Kopuz features for local music, connected services, lyrics, playlists, themes, and playback controls."
                canonical="https://kopuz.moe/features"
            />
            <PageChrome>
                <Features/>
            </PageChrome>
        </Provider>
    }
}

#[component]
pub fn DownloadPage(i18n: I18n) -> impl IntoView {
    view! {
        <Provider value=i18n>
            <PageMeta
                title=move_tr!(i18n, "download-page-title")
                description="Current Kopuz downloads for Windows, macOS, Linux, and Android, with installation notes for each package format."
                canonical="https://kopuz.moe/download"
            />
            <PageChrome>
                <Platforms/>
                <Install/>
                <Requirements/>
            </PageChrome>
        </Provider>
    }
}

#[component]
pub fn GuidesPage(i18n: I18n) -> impl IntoView {
    view! {
        <Provider value=i18n>
            <PageMeta
                title=move_tr!(i18n, "guides-page-title")
                description="Setup instructions for Nextcloud, YouTube Music, Apple Music, SoundCloud, and Spotify in Kopuz."
                canonical="https://kopuz.moe/guides"
            />
            <PageChrome>
                <section class="sec" data-title="Guides">
                    <div class="wrap">
                        <div class="sec-head">
                            <h1>{move_tr!(i18n, "guides-title")}</h1>
                            <p class="lede">{move_tr!(i18n, "guides-subtitle")}</p>
                        </div>
                        <GuideIndex/>
                    </div>
                </section>
            </PageChrome>
        </Provider>
    }
}

#[component]
pub fn GuidePage(i18n: I18n) -> impl IntoView {
    let params = use_params_map();

    view! {
        <Provider value=i18n>
            <PageChrome>
                {move || {
                    let slug = params.read().get("service").unwrap_or_default();
                    match guide_by_slug(&slug) {
                        Some(guide) => {
                            let name = guide.name;
                            let suffix = move_tr!(i18n, "guide-title-suffix");
                            let title = Signal::derive(move || format!("{name} {}", suffix.get()));

                            view! {
                                <PageMeta
                                    title=title
                                    description=guide.description
                                    canonical=guide.canonical
                                />
                                <GuideArticle guide/>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <Title text=move_tr!(i18n, "notfound-page-title")/>
                                <Meta name="robots" content="noindex, follow"/>
                                <NotFoundBody/>
                            }
                                .into_any()
                        }
                    }
                }}
            </PageChrome>
        </Provider>
    }
}

#[component]
pub fn SupportPage(i18n: I18n) -> impl IntoView {
    view! {
        <Provider value=i18n>
            <PageMeta
                title=move_tr!(i18n, "support-page-title")
                description="Kopuz donation links, sponsors, community links, and web buttons."
                canonical="https://kopuz.moe/support"
            />
            <PageChrome>
                <Support/>
                <Sponsors/>
                <Community/>
            </PageChrome>
        </Provider>
    }
}
