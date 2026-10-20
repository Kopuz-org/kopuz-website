use leptos::prelude::*;
use leptos_fluent::move_tr;

use crate::icons::Icon;
use crate::shell::internal_href;

/// One row of the guides index and one `/guides/:service` page.
pub struct Guide {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub canonical: &'static str,
}

pub static GUIDES: [Guide; 7] = [
    Guide {
        slug: "nextcloud",
        name: "Nextcloud",
        description: "Connect Kopuz to your Nextcloud server over WebDAV. Create an app password, pick the folders that hold your music, and stream your library on any device.",
        canonical: "https://kopuz.moe/guides/nextcloud",
    },
    Guide {
        slug: "smb",
        name: "SMB and NAS",
        description: "Play music straight from an SMB share on your NAS in Kopuz. Enter the share URL, sign in with your NAS account, and see what works on a read-only share.",
        canonical: "https://kopuz.moe/guides/smb",
    },
    Guide {
        slug: "youtube-music",
        name: "YouTube Music",
        description: "Set up YouTube Music in Kopuz. Sign in through your browser or listen without an account, and see which tracks need Premium and how playback works.",
        canonical: "https://kopuz.moe/guides/youtube-music",
    },
    Guide {
        slug: "apple-music",
        name: "Apple Music",
        description: "Set up Apple Music in Kopuz. Sign in or paste a media-user-token, get Widevine playback working on desktop, and see what works on Android today.",
        canonical: "https://kopuz.moe/guides/apple-music",
    },
    Guide {
        slug: "soundcloud",
        name: "SoundCloud",
        description: "Set up SoundCloud in Kopuz with a one-time sign-in through your browser. Then see what the source supports: likes, playlists, search, and streaming.",
        canonical: "https://kopuz.moe/guides/soundcloud",
    },
    Guide {
        slug: "spotify",
        name: "Spotify",
        description: "Set up Spotify in Kopuz. Create a Spotify developer app, add its Client ID and redirect URL, and choose which Spotify Connect device plays your music.",
        canonical: "https://kopuz.moe/guides/spotify",
    },
    Guide {
        slug: "clippsly",
        name: "Clippsly",
        description: "Connect Clippsly to Kopuz with an app password. Stream and download the music in your Clippsly locker, and upload new tracks right from Kopuz.",
        canonical: "https://kopuz.moe/guides/clippsly",
    },
];

pub fn guide_by_slug(slug: &str) -> Option<&'static Guide> {
    GUIDES.iter().find(|guide| guide.slug == slug)
}

fn guide_href(slug: &str) -> String {
    internal_href(&format!("/guides/{slug}"))
}

fn guide_summary(slug: &str) -> Signal<String> {
    match slug {
        "nextcloud" => move_tr!("guide-nextcloud-summary"),
        "youtube-music" => move_tr!("guide-ytmusic-summary"),
        "apple-music" => move_tr!("guide-applemusic-summary"),
        "soundcloud" => move_tr!("guide-soundcloud-summary"),
        "smb" => move_tr!("guide-smb-summary"),
        "clippsly" => move_tr!("guide-clippsly-summary"),
        _ => move_tr!("guide-spotify-summary"),
    }
}

/// The guides index. The ids are the anchors the single-page guides used, so
/// links to `/guides#spotify` still land on the right row.
#[component]
pub fn GuideIndex() -> impl IntoView {
    view! {
        <ul class="guide-index">
            {GUIDES.iter().map(|guide| {
                let (anchor, icon) = match guide.slug {
                    "youtube-music" => ("ytmusic", "youtube-music"),
                    "apple-music" => ("applemusic", "apple-music"),
                    "smb" => ("smb", "hard-drive"),
                    "clippsly" => ("clippsly", "cloud"),
                    slug => (slug, slug),
                };
                view! {
                    <li id=anchor>
                        <a href=guide_href(guide.slug)>
                            <Icon name=icon size=20/>
                            <span class="guide-index-copy">
                                <span class="guide-index-name">{guide.name}</span>
                                <span class="guide-index-desc">{guide_summary(guide.slug)}</span>
                            </span>
                            <Icon name="chevron-right" size=16 class="guide-index-go"/>
                        </a>
                    </li>
                }
            }).collect_view()}
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
                        "smb" => view! { <SmbGuide/> }.into_any(),
                        "clippsly" => view! { <ClippslyGuide/> }.into_any(),
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
fn SmbGuide() -> impl IntoView {
    view! {
        <p class="lede">{move_tr!("smb-subtitle")}</p>
        <div class="guide-blocks">
            <div>
                <h2>{move_tr!("smb-share-title")}</h2>
                <p class="prose">{move_tr!("smb-share-desc")}</p>
                <code class="guide-value">"smb://nas/Music"</code>
                <p class="prose guide-aside">{move_tr!("smb-share-port")}</p>
                <code class="guide-value">"smb://nas:1445/Music"</code>
            </div>
            <div>
                <h2>{move_tr!("smb-signin-title")}</h2>
                <p class="prose">{move_tr!("smb-signin-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("smb-library-title")}</h2>
                <p class="prose">{move_tr!("smb-library-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("smb-limits-title")}</h2>
                <p class="prose">{move_tr!("smb-limits-desc")}</p>
            </div>
        </div>
    }
}

#[component]
fn ClippslyGuide() -> impl IntoView {
    view! {
        <p class="lede">{move_tr!("clippsly-subtitle")}</p>
        <div class="guide-blocks">
            <div>
                <h2>{move_tr!("clippsly-signin-title")}</h2>
                <p class="prose">{move_tr!("clippsly-signin-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("clippsly-locker-title")}</h2>
                <p class="prose">{move_tr!("clippsly-locker-desc")}</p>
            </div>
            <div>
                <h2>{move_tr!("clippsly-upload-title")}</h2>
                <p class="prose">{move_tr!("clippsly-upload-desc")}</p>
                <p class="prose guide-aside">{move_tr!("clippsly-bonus")}</p>
            </div>
            <div>
                <h2>{move_tr!("clippsly-limits-title")}</h2>
                <p class="prose">{move_tr!("clippsly-limits-desc")}</p>
            </div>
        </div>
        <a
            class="text-link guide-link"
            href="https://clippsly.app"
            target="_blank"
            rel="noopener noreferrer"
        >
            {move_tr!("clippsly-site")}
        </a>
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

#[cfg(test)]
mod tests {
    use super::GUIDES;

    #[test]
    fn sitemap_lists_every_guide() {
        let sitemap = include_str!("../public/sitemap.xml");
        for guide in &GUIDES {
            assert!(
                sitemap.contains(&format!("<loc>{}</loc>", guide.canonical)),
                "{} missing from public/sitemap.xml",
                guide.slug
            );
        }
    }
}
