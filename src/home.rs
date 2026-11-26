use leptos::context::Provider;
use leptos::prelude::*;
use leptos_fluent::{move_tr, I18n};
use leptos_meta::{Link, Meta, Title};

use crate::app::AboutName;
use crate::download::{DownloadDialog, Installers};
use crate::features::{feature_desc, feature_title};
use crate::icons::Icon;
use crate::releases::use_latest_release;
use crate::shell::{
    internal_href, provide_site_theme, simple_mode, Footer, Nav, RobotsMeta, ThemeColorMeta,
};
use crate::support::{fetch_sponsors_list, sponsor_avatar};

const GITHUB: &str = "https://github.com/Kopuz-org/kopuz";

/// Kept in step with the page: free, no paid tier, the four platforms the
/// download page ships.
const STRUCTURED_DATA: &str = r#"{"@context":"https://schema.org","@graph":[{"@type":"WebSite","@id":"https://kopuz.moe/#website","url":"https://kopuz.moe/","name":"Kopuz","inLanguage":"en"},{"@type":"SoftwareApplication","name":"Kopuz","url":"https://kopuz.moe/","description":"A free, open-source music player for desktop and Android that plays local files, NAS shares, self-hosted servers, and streaming services in one library.","applicationCategory":"MultimediaApplication","operatingSystem":"Windows, macOS, Linux, Android","downloadUrl":"https://kopuz.moe/download","image":"https://kopuz.moe/banner.png","license":"https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12","isAccessibleForFree":true,"offers":{"@type":"Offer","price":"0","priceCurrency":"USD"},"sameAs":["https://github.com/Kopuz-org/kopuz","https://flathub.org/apps/moe.kopuz.kopuz","https://play.google.com/store/apps/details?id=moe.kopuz.kopuz"]}]}"#;

/// Screenshots keep their paths across redesigns, so returning visitors would
/// otherwise see cached old captures next to new copy.
fn asset(path: &str) -> String {
    format!("{path}?v={}", crate::app::css_cache_bust())
}

#[component]
pub fn HomePage(i18n: I18n) -> impl IntoView {
    let theme = provide_site_theme();
    let simple = simple_mode();

    view! {
        <Provider value=i18n>
            <Title text=move_tr!(i18n, "home-title")/>
            <Meta name="description" content=move_tr!(i18n, "home-meta-desc")/>
            <Meta name="keywords" content=move_tr!(i18n, "home-meta-keywords")/>
            <RobotsMeta/>
            <Meta property="og:title" content=move_tr!(i18n, "og-title")/>
            <Meta property="og:description" content=move_tr!(i18n, "og-desc")/>
            <Meta property="og:url" content="https://kopuz.moe/"/>
            <Meta name="twitter:title" content=move_tr!(i18n, "twitter-title")/>
            <Meta name="twitter:description" content=move_tr!(i18n, "twitter-desc")/>
            <Link rel="canonical" href="https://kopuz.moe/"/>
            <ThemeColorMeta/>
            <script type="application/ld+json" inner_html=STRUCTURED_DATA></script>
            <div
                class="site"
                class:simple=simple
                class:light=move || !theme.dark.get() && !theme.moe
                class:dark=move || theme.dark.get() && !theme.moe
                class:moe=move || theme.moe
            >
                <Nav/>
                <main>
                    <Hero/>
                    <Sources/>
                    <Gallery/>
                    <GetKopuz/>
                    <HomeSponsors/>
                    <AboutName/>
                </main>
                <Footer/>
            </div>
        </Provider>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let simple = simple_mode();
    let download_href = internal_href("/download");
    let release = use_latest_release();
    let version = move || {
        release
            .get()
            .map(|release| release.version().to_string())
            .filter(|version| !version.is_empty())
            .map(|version| view! { <span class="tabular">"v"{version}</span> })
    };

    view! {
        <section class="sec hero-sec" data-title=move_tr!("chapter-home")>
            <div class="wrap">
                <div class="hero">
                    <div class="hero-copy">
                        <h1 class="rise rise-1">
                            {move_tr!("hero-title-1")}" "<em>{move_tr!("hero-title-2")}</em>
                        </h1>
                        <p class="lede rise rise-2">{move_tr!("hero-desc")}</p>
                        {if simple {
                            view! {
                                <p>
                                    <a href=download_href.clone()>{move_tr!("nav-download")}</a>
                                </p>
                            }.into_any()
                        } else {
                            view! {
                                <div class="hero-actions rise rise-3">
                                    <a
                                        class="btn btn-primary"
                                        href=download_href.clone()
                                        aria-haspopup="dialog"
                                        data-open-download
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
                                    <a
                                        class="btn btn-ghost"
                                        href=GITHUB
                                        target="_blank"
                                        rel="noopener noreferrer"
                                    >
                                        <Icon name="github" size=15/>
                                        "GitHub"
                                    </a>
                                </div>
                                <DownloadDialog/>
                            }.into_any()
                        }}
                        <p class="hero-platforms rise rise-4">
                            <Transition fallback=|| ()>{version}</Transition>
                            <span>"Linux · macOS · Windows · Android"</span>
                            <span>"EUPL 1.2"</span>
                        </p>
                    </div>
                    {(!simple).then(|| view! {
                        <div class="hero-art rise rise-2">
                            <canvas class="hero-spectrum" data-spectrum aria-hidden="true"></canvas>
                            <HeroShot/>
                        </div>
                    })}
                </div>
            </div>
        </section>
    }
}

struct Source {
    name_key: &'static str,
    anchor: &'static str,
}

struct SourceGroup {
    icon: &'static str,
    title_key: &'static str,
    sources: &'static [Source],
}

/// Grouped the way the app's source picker groups them.
const SOURCE_GROUPS: &[SourceGroup] = &[
    SourceGroup {
        icon: "hard-drive",
        title_key: "home-sources-local",
        sources: &[
            Source {
                name_key: "source-local",
                anchor: "src-local",
            },
            Source {
                name_key: "source-smb",
                anchor: "src-smb",
            },
            Source {
                name_key: "source-radio",
                anchor: "features",
            },
        ],
    },
    SourceGroup {
        icon: "server",
        title_key: "home-sources-server",
        sources: &[
            Source {
                name_key: "features-source-jellyfin",
                anchor: "src-jellyfin",
            },
            Source {
                name_key: "source-navidrome",
                anchor: "src-navidrome",
            },
            Source {
                name_key: "features-source-nextcloud",
                anchor: "src-nextcloud",
            },
        ],
    },
    SourceGroup {
        icon: "cloud",
        title_key: "home-sources-streaming",
        sources: &[
            Source {
                name_key: "features-source-ytmusic",
                anchor: "src-ytmusic",
            },
            Source {
                name_key: "features-source-spotify",
                anchor: "src-spotify",
            },
            Source {
                name_key: "features-source-applemusic",
                anchor: "src-applemusic",
            },
            Source {
                name_key: "features-source-soundcloud",
                anchor: "src-soundcloud",
            },
            Source {
                name_key: "source-clippsly",
                anchor: "src-clippsly",
            },
        ],
    },
];

fn source_name(key: &'static str) -> Signal<String> {
    match key {
        "source-local" => move_tr!("source-local"),
        "source-navidrome" => move_tr!("source-navidrome"),
        "source-smb" => move_tr!("source-smb"),
        "source-clippsly" => move_tr!("source-clippsly"),
        "source-radio" => move_tr!("source-radio"),
        "features-source-jellyfin" => move_tr!("features-source-jellyfin"),
        "features-source-nextcloud" => move_tr!("features-source-nextcloud"),
        "features-source-ytmusic" => move_tr!("features-source-ytmusic"),
        "features-source-applemusic" => move_tr!("features-source-applemusic"),
        "features-source-soundcloud" => move_tr!("features-source-soundcloud"),
        "features-source-spotify" => move_tr!("features-source-spotify"),
        "home-sources-local" => move_tr!("home-sources-local"),
        "home-sources-server" => move_tr!("home-sources-server"),
        "home-sources-streaming" => move_tr!("home-sources-streaming"),
        _ => Signal::derive(String::new),
    }
}

#[component]
fn Sources() -> impl IntoView {
    let simple = simple_mode();

    view! {
        <section class="sec sources-sec reveal-sec" data-title=move_tr!("chapter-sources")>
            <div class="wrap">
                <h2>{move_tr!("home-sources-title")}</h2>
                <p class="prose sources-note">{move_tr!("home-sources-plugin-note")}</p>
                <div class="source-groups">
                    {SOURCE_GROUPS.iter().map(|group| view! {
                        <div class="source-group">
                            <h3>
                                {(!simple).then(|| view! { <Icon name=group.icon size=18/> })}
                                {source_name(group.title_key)}
                            </h3>
                            <ul>
                                {group.sources.iter().map(|source| {
                                    let href = internal_href(&format!("/features#{}", source.anchor));
                                    view! {
                                        <li>
                                            <a href=href>
                                                <span>{source_name(source.name_key)}</span>
                                            </a>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        </div>
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn HeroShot() -> impl IntoView {
    view! {
        <img
            class="hero-shot"
            src=asset("/kopuz-home.png")
            alt=move_tr!("hero-screenshot-alt")
            width="2000"
            height="1091"
            fetchpriority="high"
            decoding="async"
        />
    }
}

struct Shot {
    image: &'static str,
    title_key: &'static str,
    desc_key: &'static str,
}

/// Captures of the 1.0 app, all 2000x1091.
const SHOTS: &[Shot] = &[
    Shot {
        image: "/kopuz-lyrics.png",
        title_key: "feat-lyrics-title",
        desc_key: "feat-lyrics-desc",
    },
    Shot {
        image: "/kopuz-library.png",
        title_key: "feat-local-title",
        desc_key: "feat-local-desc",
    },
    Shot {
        image: "/kopuz-playlist.png",
        title_key: "feat-fav-title",
        desc_key: "feat-fav-desc",
    },
    Shot {
        image: "/kopuz-search.png",
        title_key: "feat-search-title",
        desc_key: "feat-search-desc",
    },
];

#[component]
fn Gallery() -> impl IntoView {
    let simple = simple_mode();
    let features_href = internal_href("/features");

    view! {
        <section class="sec gallery-sec reveal-sec">
            <div class="wrap">
                <h2>{move_tr!("features-title")}</h2>
                <ul class="gallery">
                    {SHOTS.iter().map(|shot| {
                        let title = feature_title(shot.title_key);
                        view! {
                            <li class="gallery-item">
                                {(!simple).then(|| view! {
                                    <a class="shot-link" href=asset(shot.image) data-zoom>
                                        <img
                                            class="shot"
                                            src=asset(shot.image)
                                            alt=title
                                            width="2000"
                                            height="1091"
                                            loading="lazy"
                                            decoding="async"
                                        />
                                    </a>
                                })}
                                <h3>{title}</h3>
                                <p>{feature_desc(shot.desc_key)}</p>
                            </li>
                        }
                    }).collect_view()}
                </ul>
                {(!simple).then(|| view! {
                    <dialog class="zoom" tabindex="-1" data-zoom-dialog>
                        <img alt="" width="2000" height="1091"/>
                        <button
                            type="button"
                            class="btn btn-ghost btn-icon zoom-close"
                            aria-label=move_tr!("dl-dialog-close")
                            data-close-dialog
                        >
                            <Icon name="x" size=16/>
                        </button>
                    </dialog>
                })}
                <a class="text-link" href=features_href>
                    {move_tr!("home-see-all-features")}
                    <Icon name="arrow-up-right" size=14/>
                </a>
            </div>
        </section>
    }
}

#[component]
fn GetKopuz() -> impl IntoView {
    let download_href = internal_href("/download");

    view! {
        <section class="sec get-sec reveal-sec" data-title=move_tr!("chapter-download")>
            <div class="wrap">
                <h2>{move_tr!("home-get-title")}</h2>
                <Installers group="dl-os"/>
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
        <section class="sec home-sponsors reveal-sec" data-title=move_tr!("chapter-sponsors")>
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
        for source in SOURCE_GROUPS.iter().flat_map(|group| group.sources) {
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
