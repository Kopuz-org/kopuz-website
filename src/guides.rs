use leptos::prelude::*;
use leptos_fluent::move_tr;

use crate::shell::internal_href;

/// One row of the guides index and one `/guides/:service` page.
pub struct Guide {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub canonical: &'static str,
}

pub static GUIDES: [Guide; 5] = [
    Guide {
        slug: "nextcloud",
        name: "Nextcloud",
        description: "Connect a Nextcloud server to Kopuz over WebDAV: app password, library folders, and playback.",
        canonical: "https://kopuz.moe/guides/nextcloud",
    },
    Guide {
        slug: "youtube-music",
        name: "YouTube Music",
        description: "Set up YouTube Music in Kopuz: browser sign-in, anonymous mode, and Premium track limits.",
        canonical: "https://kopuz.moe/guides/youtube-music",
    },
    Guide {
        slug: "apple-music",
        name: "Apple Music",
        description: "Set up Apple Music in Kopuz: sign-in or media-user-token, Widevine on desktop, and Android status.",
        canonical: "https://kopuz.moe/guides/apple-music",
    },
    Guide {
        slug: "soundcloud",
        name: "SoundCloud",
        description: "Set up SoundCloud in Kopuz with a one-time browser sign-in, and what the source supports.",
        canonical: "https://kopuz.moe/guides/soundcloud",
    },
    Guide {
        slug: "spotify",
        name: "Spotify",
        description: "Set up Spotify in Kopuz: create a developer app, add the Client ID, and choose a playback device.",
        canonical: "https://kopuz.moe/guides/spotify",
    },
];

pub fn guide_by_slug(slug: &str) -> Option<&'static Guide> {
    GUIDES.iter().find(|guide| guide.slug == slug)
}

fn guide_href(slug: &str) -> String {
    internal_href(&format!("/guides/{slug}"))
}

/// The guides index. The ids are the anchors the single-page guides used, so
/// links to `/guides#spotify` still land on the right row.
#[component]
pub fn GuideIndex() -> impl IntoView {
    view! {
        <ul class="guide-index">
            <li id="nextcloud">
                <a href=guide_href("nextcloud")>
                    <span class="guide-index-name">"Nextcloud"</span>
                    <span class="guide-index-desc">{move_tr!("nextcloud-subtitle")}</span>
                </a>
            </li>
            <li id="ytmusic">
                <a href=guide_href("youtube-music")>
                    <span class="guide-index-name">"YouTube Music"</span>
                    <span class="guide-index-desc">{move_tr!("ytmusic-subtitle")}</span>
                </a>
            </li>
            <li id="applemusic">
                <a href=guide_href("apple-music")>
                    <span class="guide-index-name">"Apple Music"</span>
                    <span class="guide-index-desc">{move_tr!("applemusic-subtitle")}</span>
                </a>
            </li>
            <li id="soundcloud">
                <a href=guide_href("soundcloud")>
                    <span class="guide-index-name">"SoundCloud"</span>
                    <span class="guide-index-desc">{move_tr!("soundcloud-subtitle")}</span>
                </a>
            </li>
            <li id="spotify">
                <a href=guide_href("spotify")>
                    <span class="guide-index-name">"Spotify"</span>
                    <span class="guide-index-desc">{move_tr!("spotify-guide-subtitle")}</span>
                </a>
            </li>
        </ul>
    }
}

#[component]
fn GuideSide(current: &'static str) -> impl IntoView {
    view! {
        <nav class="guide-side" aria-label=move_tr!("guides-title")>
            {GUIDES
                .iter()
                .map(|guide| {
                    let is_current = guide.slug == current;
                    view! {
                        <a
                            href=guide_href(guide.slug)
                            class="guide-side-link"
                            class:is-current=is_current
                            aria-current=is_current.then_some("page")
                        >
                            {guide.name}
                        </a>
                    }
                })
                .collect_view()}
        </nav>
    }
}

#[component]
pub fn GuideArticle(guide: &'static Guide) -> impl IntoView {
    view! {
        <section class="sec guide-page" data-title=guide.name>
            <div class="wrap guide-layout">
                <GuideSide current=guide.slug/>
                <div class="guide-main">
                    <h1>{guide.name}</h1>
                    {match guide.slug {
                        "nextcloud" => view! { <NextcloudGuide/> }.into_any(),
                        "youtube-music" => view! { <YtMusic/> }.into_any(),
                        "apple-music" => view! { <AppleMusicGuide/> }.into_any(),
                        "soundcloud" => view! { <SoundCloud/> }.into_any(),
                        _ => view! { <SpotifyGuide/> }.into_any(),
                    }}
                </div>
            </div>
        </section>
    }
}

#[component]
fn NextcloudGuide() -> impl IntoView {
    view! {
        <p class="lede">{move_tr!("nextcloud-subtitle")}</p>
        <div class="guide-blocks">
            <div>
                <h2>{move_tr!("nextcloud-connect-title")}</h2>
                <p class="prose">{move_tr!("nextcloud-connect-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("nextcloud-folders-title")}</h2>
                <p class="prose">{move_tr!("nextcloud-folders-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("nextcloud-webdav-title")}</h2>
                <p class="prose">{move_tr!("nextcloud-webdav-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("nextcloud-playback-title")}</h2>
                <p class="prose">{move_tr!("nextcloud-playback-desc")}</p>
            </div>
        </div>
    }
}

#[component]
fn YtMusic() -> impl IntoView {
    view! {
        <p class="lede">{move_tr!("ytmusic-subtitle")}</p>
        <div class="guide-blocks">
            <div>
                <h2>{move_tr!("ytmusic-token-title")}</h2>
                <p class="prose">{move_tr!("ytmusic-token-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("ytmusic-signin-title")}</h2>
                <p class="prose">{move_tr!("ytmusic-signin-desc")}</p>
                <p class="prose guide-aside">{move_tr!("ytmusic-signin-note")}</p>
            </div>
            <div>
                <h2>{move_tr!("ytmusic-anon-title")}</h2>
                <p class="prose">{move_tr!("ytmusic-anon-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("ytmusic-premium-title")}</h2>
                <p class="prose">{move_tr!("ytmusic-premium-desc")}</p>
            </div>
        </div>
    }
}

#[component]
fn AppleMusicGuide() -> impl IntoView {
    view! {
        <p class="lede">{move_tr!("applemusic-subtitle")}</p>
        <div class="guide-blocks">
            <div>
                <h2>{move_tr!("applemusic-signin-title")}</h2>
                <p class="prose">{move_tr!("applemusic-signin-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("applemusic-playback-title")}</h2>
                <p class="prose">{move_tr!("applemusic-playback-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("applemusic-features-title")}</h2>
                <p class="prose">{move_tr!("applemusic-features-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("applemusic-android-title")}</h2>
                <p class="prose">{move_tr!("applemusic-android-desc")}</p>
            </div>
        </div>
    }
}

#[component]
fn SoundCloud() -> impl IntoView {
    view! {
        <p class="lede">{move_tr!("soundcloud-subtitle")}</p>
        <div class="guide-blocks">
            <div>
                <h2>{move_tr!("soundcloud-signin-title")}</h2>
                <p class="prose">{move_tr!("soundcloud-signin-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("soundcloud-features-title")}</h2>
                <p class="prose">{move_tr!("soundcloud-features-desc")}</p>
            </div>
        </div>
    }
}

#[component]
fn SpotifyGuide() -> impl IntoView {
    view! {
        <p class="lede">{move_tr!("spotify-guide-subtitle")}</p>
        <ol class="guide-steps">
            <li>
                <span class="guide-step-lead">{move_tr!("spotify-step-1-title")}</span>
                {move_tr!("spotify-step-1-desc")}
                <code class="guide-value">"http://127.0.0.1:8898/callback"</code>
            </li>
            <li>
                <span class="guide-step-lead">{move_tr!("spotify-step-2-title")}</span>
                {move_tr!("spotify-step-2-desc")}
            </li>
            <li>
                <span class="guide-step-lead">{move_tr!("spotify-step-3-title")}</span>
                {move_tr!("spotify-step-3-desc")}
            </li>
        </ol>
        <p class="prose guide-after">{move_tr!("spotify-requirement")}</p>
        <a
            class="text-link guide-link"
            href="https://github.com/Kopuz-org/kopuz#spotify-setup"
            target="_blank"
            rel="noopener noreferrer"
        >
            {move_tr!("spotify-full-guide")}
        </a>
    }
}
