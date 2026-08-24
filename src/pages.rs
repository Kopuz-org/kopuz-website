use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_meta::{Link, Meta, Title};

use crate::app::{
    provide_site_theme, AppleMusicGuide, Community, Features, Footer, Install, Nav, NextcloudGuide,
    Performance, Platforms, Privacy, Requirements, SoundCloud, Sponsors, SpotifyGuide, Support,
    ThemeColorMeta, WebButton, YtMusic,
};

#[component]
fn PageMeta(
    title: &'static str,
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
        </div>
    }
}

#[component]
pub fn FeaturesPage() -> impl IntoView {
    view! {
        <PageMeta
            title="Features | Kopuz"
            description="Kopuz features for local music, connected services, lyrics, playlists, themes, and playback controls."
            canonical="https://kopuz.moe/features"
        />
        <PageChrome>
            <Features/>
            <Privacy/>
            <Performance/>
        </PageChrome>
    }
}

#[component]
pub fn DownloadPage() -> impl IntoView {
    view! {
        <PageMeta
            title="Download | Kopuz"
            description="Current Kopuz downloads for Windows, macOS, Linux, and Android, with installation notes for each package format."
            canonical="https://kopuz.moe/download"
        />
        <PageChrome>
            <Platforms/>
            <Install/>
            <Requirements/>
        </PageChrome>
    }
}

#[component]
pub fn GuidesPage() -> impl IntoView {
    view! {
        <PageMeta
            title="Guides | Kopuz"
            description="Setup instructions for Nextcloud, YouTube Music, Apple Music, SoundCloud, and Spotify in Kopuz."
            canonical="https://kopuz.moe/guides"
        />
        <PageChrome>
            <header class="page-intro">
                <h1>{move_tr!("guides-title")}</h1>
                <p>{move_tr!("guides-subtitle")}</p>
            </header>
            <NextcloudGuide/>
            <YtMusic/>
            <AppleMusicGuide/>
            <SoundCloud/>
            <SpotifyGuide/>
        </PageChrome>
    }
}

#[component]
pub fn SupportPage() -> impl IntoView {
    view! {
        <PageMeta
            title="Support | Kopuz"
            description="Kopuz donation links, sponsors, community links, and web buttons."
            canonical="https://kopuz.moe/support"
        />
        <PageChrome>
            <Support/>
            <Sponsors/>
            <Community/>
            <WebButton/>
        </PageChrome>
    }
}
