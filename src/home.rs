use leptos::context::Provider;
use leptos::prelude::*;
use leptos_fluent::{move_tr, I18n};
use leptos_meta::{Link, Meta, Title};

use crate::app::AboutName;
use crate::download::{platform, platform_text};
use crate::features::feature_desc;
use crate::icons::Icon;
use crate::releases::{use_latest_release, WhatsNew};
use crate::shell::{internal_href, provide_site_theme, Footer, Nav, PlayerBar, Shelf, ThemeColorMeta};
use crate::support::{fetch_sponsors_list, sponsor_avatar};

const GITHUB: &str = "https://github.com/Kopuz-org/kopuz";

/* The theme wipe reveals a screenshot and its desk in the same frame, so every
   one of them has to be decoded before the card reaches the viewport. */
const THEME_PRELOADS: &[&str] = &[
    "/themes/default.png",
    "/themes/amoled.png",
    "/themes/sunset.png",
    "/themes/lake.png",
    "/wallpapers/sunset.jpg",
    "/wallpapers/lake.jpg",
];

/// Screenshots keep their paths across redesigns, so returning visitors would
/// otherwise see cached old captures next to new copy.
fn asset(path: &str) -> String {
    format!("{path}?v={}", crate::app::css_cache_bust())
}

#[component]
pub fn HomePage(i18n: I18n) -> impl IntoView {
    let theme = provide_site_theme();

    view! {
        <Provider value=i18n>
            <Title text=move_tr!(i18n, "home-title")/>
            <Meta name="description" content=move_tr!(i18n, "home-meta-desc")/>
            <Meta name="keywords" content=move_tr!(i18n, "home-meta-keywords")/>
            <Meta name="robots" content="index, follow"/>
            <Meta property="og:title" content=move_tr!(i18n, "og-title")/>
            <Meta property="og:description" content=move_tr!(i18n, "og-desc")/>
            <Meta property="og:url" content="https://kopuz.moe"/>
            <Meta name="twitter:title" content=move_tr!(i18n, "twitter-title")/>
            <Meta name="twitter:description" content=move_tr!(i18n, "twitter-desc")/>
            <Link rel="canonical" href="https://kopuz.moe"/>
            {THEME_PRELOADS
                .iter()
                .map(|href| view! { <Link rel="preload" as_="image" href=asset(href)/> })
                .collect_view()}
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
                    <Sources/>
                    <Moments/>
                    <GetKopuz/>
                    <HomeSponsors/>
                    <WhatsNew/>
                    <AboutName/>
                </main>
                <Footer/>
                <Shelf/>
            </div>
            <PlayerBar/>
        </Provider>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let download_href = internal_href("/download");
    let release = use_latest_release();
    let release_label = move_tr!("hero-release");
    let release_tag = move || {
        let label = release_label.get();
        let version = release
            .get()
            .map(|release| release.version().to_string())
            .unwrap_or_default();
        if version.is_empty() {
            label
        } else {
            format!("{label} · v{version}")
        }
    };

    view! {
        <section class="sec hero-sec" data-title="Home">
            <div class="wrap">
                <div class="card hero">
                    <div class="hero-wash" aria-hidden="true"></div>
                    <div class="hero-copy">
                        <p class="hero-release rise rise-1">
                            <Icon name="star" size=12 class="hero-star"/>
                            <Transition fallback=|| view! { <span>{move_tr!("hero-release")}</span> }>
                                {move || view! { <span>{release_tag()}</span> }}
                            </Transition>
                        </p>
                        <h1 class="rise rise-2">
                            {move_tr!("hero-title-1")}<br/>{move_tr!("hero-title-2")}
                        </h1>
                        <p class="lede rise rise-3">{move_tr!("hero-desc")}</p>
                        <div class="hero-actions rise rise-4">
                            <a
                                class="btn btn-primary"
                                href=download_href.clone()
                                data-os-cta
                            >
                                <Icon name="download" size=15/>
                                <span class="os-generic">{move_tr!("hero-cta-download")}</span>
                                {["linux", "macos", "windows", "android"].into_iter().map(|os| {
                                    let name = match os {
                                        "linux" => "Linux",
                                        "macos" => "macOS",
                                        "windows" => "Windows",
                                        _ => "Android",
                                    };
                                    view! {
                                        <span class="os-label" data-os=os>
                                            {move_tr!("hero-cta-download-for")}" "{name}
                                        </span>
                                    }
                                }).collect_view()}
                            </a>
                            <a class="btn btn-ghost" href=download_href>{move_tr!("hero-cta-all")}</a>
                            <a
                                class="btn btn-ghost btn-icon"
                                href=GITHUB
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label=move_tr!("hero-cta-github")
                            >
                                <Icon name="github" size=16/>
                            </a>
                        </div>
                        <p class="hero-platforms rise rise-4">
                            <span><Icon name="monitor" size=13/>"Linux · macOS · Windows"</span>
                            <span><Icon name="smartphone" size=13/>"Android 7+"</span>
                        </p>
                    </div>
                    <div class="hero-art">
                        <img
                            src=asset("/app-home.jpg")
                            alt=move_tr!("hero-screenshot-alt")
                            width="1800"
                            height="987"
                            loading="eager"
                            fetchpriority="high"
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}

struct Source {
    icon: &'static str,
    name_key: &'static str,
    anchor: &'static str,
}

const SOURCES: &[Source] = &[
    Source {
        icon: "folder",
        name_key: "source-local",
        anchor: "src-local",
    },
    Source {
        icon: "jellyfin",
        name_key: "features-source-jellyfin",
        anchor: "src-jellyfin",
    },
    Source {
        icon: "server",
        name_key: "source-navidrome",
        anchor: "src-navidrome",
    },
    Source {
        icon: "nextcloud",
        name_key: "features-source-nextcloud",
        anchor: "src-nextcloud",
    },
    Source {
        icon: "youtube-music",
        name_key: "features-source-ytmusic",
        anchor: "src-ytmusic",
    },
    Source {
        icon: "apple-music",
        name_key: "features-source-applemusic",
        anchor: "src-applemusic",
    },
    Source {
        icon: "soundcloud",
        name_key: "features-source-soundcloud",
        anchor: "src-soundcloud",
    },
    Source {
        icon: "spotify",
        name_key: "features-source-spotify",
        anchor: "src-spotify",
    },
    Source {
        icon: "radio",
        name_key: "source-radio",
        anchor: "features",
    },
];

fn source_name(key: &'static str) -> Signal<String> {
    match key {
        "source-local" => move_tr!("source-local"),
        "source-navidrome" => move_tr!("source-navidrome"),
        "source-radio" => move_tr!("source-radio"),
        "features-source-jellyfin" => move_tr!("features-source-jellyfin"),
        "features-source-nextcloud" => move_tr!("features-source-nextcloud"),
        "features-source-ytmusic" => move_tr!("features-source-ytmusic"),
        "features-source-applemusic" => move_tr!("features-source-applemusic"),
        "features-source-soundcloud" => move_tr!("features-source-soundcloud"),
        "features-source-spotify" => move_tr!("features-source-spotify"),
        _ => Signal::derive(String::new),
    }
}

#[component]
fn Sources() -> impl IntoView {
    view! {
        <section class="sec sources-sec" data-title="Plays from">
            <div class="wrap">
                <h2>{move_tr!("home-sources-title")}</h2>
                <ul class="sources-inline">
                    {SOURCES.iter().map(|source| {
                        let href = internal_href(&format!("/features#{}", source.anchor));
                        let name = source_name(source.name_key);
                        view! {
                            <li>
                                <a href=href>
                                    <Icon name=source.icon/>
                                    <span>{name}</span>
                                </a>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </div>
        </section>
    }
}

#[component]
fn Moments() -> impl IntoView {
    view! {
        <section class="sec moments" data-title="In use">
            <div class="wrap moments-list" data-tilt-group>
                <Moment
                    image="/fullscreen-lyrics.png"
                    width=1920
                    height=1080
                    alt_key="moment-lyrics-alt"
                    title_key="moment-lyrics-title"
                    desc_key="feat-lyrics-desc"
                />
                <Moment
                    image="/normal-library.png"
                    width=1920
                    height=1080
                    alt_key="moment-library-alt"
                    title_key="moment-library-title"
                    desc_key="feat-local-desc"
                />
                <Moment
                    image="/normal-library.png"
                    width=1920
                    height=1080
                    alt_key="moment-themes-alt"
                    title_key="moment-themes-title"
                    desc_key="feat-theming-desc"
                    extra_key="moment-themes-extra"
                    see_all=true
                    themes=true
                />
            </div>
        </section>
    }
}

fn moment_text(key: &'static str) -> Signal<String> {
    match key {
        "moment-lyrics-alt" => move_tr!("moment-lyrics-alt"),
        "moment-library-alt" => move_tr!("moment-library-alt"),
        "moment-themes-alt" => move_tr!("moment-themes-alt"),
        "moment-lyrics-title" => move_tr!("moment-lyrics-title"),
        "moment-library-title" => move_tr!("moment-library-title"),
        "moment-themes-title" => move_tr!("moment-themes-title"),
        "moment-themes-extra" => move_tr!("moment-themes-extra"),
        _ => Signal::derive(String::new),
    }
}

/* Example wallpapers for the live theme. CC0 / public domain, sources listed
   in public/wallpapers/SOURCES.txt. */
struct Wallpaper {
    slug: &'static str,
    name_key: &'static str,
}

const WALLPAPERS: &[Wallpaper] = &[
    Wallpaper {
        slug: "sunset",
        name_key: "wallpaper-sunset",
    },
    Wallpaper {
        slug: "lake",
        name_key: "wallpaper-lake",
    },
];

fn wallpaper_name(key: &'static str) -> Signal<String> {
    match key {
        "wallpaper-sunset" => move_tr!("wallpaper-sunset"),
        _ => move_tr!("wallpaper-lake"),
    }
}

#[component]
fn Moment(
    image: &'static str,
    width: u32,
    height: u32,
    alt_key: &'static str,
    title_key: &'static str,
    desc_key: &'static str,
    #[prop(optional)] extra_key: Option<&'static str>,
    #[prop(optional)] see_all: bool,
    #[prop(optional)] themes: bool,
) -> impl IntoView {
    let features_href = internal_href("/features");

    view! {
        <article class="moment" class:moment-themes=themes data-tilt>
            <div class="moment-copy">
                <div class="moment-copy-inner">
                    <h2>{moment_text(title_key)}</h2>
                    <p class="prose">{feature_desc(desc_key)}</p>
                    {extra_key.map(|key| view! { <p class="prose">{moment_text(key)}</p> })}
                    {see_all.then(|| view! {
                        <a class="text-link" href=features_href>
                            {move_tr!("home-see-all-features")}
                            <Icon name="arrow-up-right" size=14/>
                        </a>
                    })}
                </div>
            </div>
            <div class="moment-stage">
                <div class="moment-stage-inner">
                    <div class="moment-art">
                        {if themes {
                            view! { <ThemeStage alt_key=alt_key/> }.into_any()
                        } else {
                            view! {
                                <figure class="frame reveal">
                                    <img
                                        src=asset(image)
                                        alt=moment_text(alt_key)
                                        width=width
                                        height=height
                                        loading="lazy"
                                    />
                                </figure>
                            }.into_any()
                        }}
                    </div>
                </div>
            </div>
        </article>
    }
}

/// One plain screenshot per theme, stacked, each with the desk it sits on. The
/// scroll wipes them in from the start edge, so every element it drives carries
/// the index of the theme it belongs to.
#[component]
fn ThemeStage(alt_key: &'static str) -> impl IntoView {
    let wallpaper_label = move_tr!("theme-wallpaper");

    view! {
        <div class="theme-scene" data-theme-stage>
            <div class="theme-desk" data-theme-index="0"></div>
            <div class="theme-desk" data-theme-index="1"></div>
            {WALLPAPERS.iter().enumerate().map(|(index, wall)| {
                let desk = format!("background-image: url({})", asset(&format!("/wallpapers/{}.jpg", wall.slug)));
                view! {
                    <div class="theme-desk" data-theme-index=(index + 2).to_string() style=desk></div>
                }
            }).collect_view()}
            <figure class="frame reveal theme-frame">
                <div class="theme-layer" data-theme-index="0" data-theme-label=move_tr!("theme-default")>
                    <img
                        src=asset("/themes/default.png")
                        alt=moment_text(alt_key)
                        width="1920"
                        height="1080"
                        loading="eager"
                        decoding="async"
                    />
                </div>
                <div class="theme-layer" data-theme-index="1" data-theme-label=move_tr!("theme-amoled")>
                    <img
                        src=asset("/themes/amoled.png")
                        alt=""
                        width="1920"
                        height="1080"
                        loading="eager"
                        decoding="async"
                    />
                </div>
                {WALLPAPERS.iter().enumerate().map(|(index, wall)| {
                    let name = wallpaper_name(wall.name_key);
                    let label = Signal::derive(move || format!("{} \u{b7} {}", wallpaper_label.get(), name.get()));
                    let src = asset(&format!("/themes/{}.png", wall.slug));
                    view! {
                        <div
                            class="theme-layer"
                            data-theme-index=(index + 2).to_string()
                            data-theme-label=label
                        >
                            <img
                                src=src
                                alt=""
                                width="1920"
                                height="1080"
                                loading="eager"
                                decoding="async"
                            />
                        </div>
                    }
                }).collect_view()}
            </figure>
            <div class="theme-line-mask" aria-hidden="true">
                <i class="theme-line"></i>
            </div>
            <p class="theme-caption"><span data-theme-caption>{move_tr!("theme-default")}</span></p>
        </div>
    }
}

#[component]
fn GetKopuz() -> impl IntoView {
    let download_href = internal_href("/download");
    let order = ["linux", "macos", "windows", "android"];

    view! {
        <section class="sec get-sec" data-title="Get Kopuz">
            <div class="wrap">
                <h2>{move_tr!("home-get-title")}</h2>
                <ul class="get-list">
                    {order.into_iter().map(|id| {
                        let entry = platform(id);
                        let name = platform_text(entry.name_key);
                        let formats = entry.formats.join(" · ");
                        view! {
                            <li class="get-row">
                                <Icon name=entry.icon/>
                                <span class="get-name">{name}</span>
                                <span class="get-formats">{formats}</span>
                                <a class="text-link get-dl" href=entry.href target="_blank" rel="noopener noreferrer">
                                    {move_tr!("home-get-download")}
                                    <Icon name="arrow-up-right" size=14/>
                                </a>
                            </li>
                        }
                    }).collect_view()}
                </ul>
                <p class="get-also">
                    <a href=download_href>{move_tr!("home-also-on")}</a>
                </p>
            </div>
        </section>
    }
}

#[component]
fn HomeSponsors() -> impl IntoView {
    let sponsors_list = Resource::new(|| (), |_| async move { fetch_sponsors_list().await });

    view! {
        <section class="sec home-sponsors" data-title="Sponsors">
            <div class="wrap">
                <h2>{move_tr!("home-sponsors-title")}</h2>
                <Suspense fallback=|| ()>
                    {move || sponsors_list.get().map(|sponsors| {
                        let tiers = sponsors.tiers();
                        let monthly = counted(move_tr!("sponsors-tier-monthly"), tiers.monthly.len());
                        let one_time = counted(
                            move_tr!("sponsors-tier-one-time"),
                            tiers.one_time.len(),
                        );

                        view! {
                            <div class="sponsor-tiers">
                                <SponsorTierRow
                                    label=move_tr!("sponsors-tier-special")
                                    logins=tiers.special
                                    size=56
                                />
                                <SponsorTierRow label=monthly logins=tiers.monthly size=36/>
                                <SponsorTierRow label=one_time logins=tiers.one_time size=36/>
                            </div>
                        }
                    })}
                </Suspense>
                <a
                    class="btn btn-ghost"
                    href="https://github.com/sponsors/temidaradev"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <Icon name="heart" size=15/>
                    {move_tr!("home-sponsor-cta")}
                </a>
            </div>
        </section>
    }
}

fn counted(label: Signal<String>, count: usize) -> Signal<String> {
    Signal::derive(move || format!("{} \u{b7} {count}", label.get()))
}

#[component]
fn SponsorTierRow(label: Signal<String>, logins: Vec<String>, size: u32) -> impl IntoView {
    let avatars_class = if size > 36 {
        "sponsor-avatars sponsor-avatars-lg"
    } else {
        "sponsor-avatars"
    };

    (!logins.is_empty()).then(move || {
        view! {
            <div class="sponsor-tier">
                <span class="sponsor-tier-label">{label}</span>
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

#[cfg(test)]
mod home_tests {
    use super::*;
    use crate::features::SOURCE_ANCHORS;
    use crate::support::SponsorsList;

    #[test]
    fn every_source_anchor_exists_on_the_features_page() {
        for source in SOURCES {
            assert!(
                source.anchor == "features"
                    || SOURCE_ANCHORS
                        .iter()
                        .any(|(anchor, _)| *anchor == source.anchor),
                "unknown features anchor: {}",
                source.anchor
            );
        }
    }

    #[test]
    fn sponsors_fallback_is_not_empty() {
        assert!(!SponsorsList::fallback().current.is_empty());
    }
}
