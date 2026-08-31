use leptos::prelude::*;
use leptos_fluent::move_tr;

use crate::icons::Icon;
use crate::shell::internal_href;

/// Anchors the home sources row links into. Nothing renders from it: it is the
/// contract the tests on both sides check, so the two modules cannot drift.
#[allow(dead_code)]
pub(crate) const SOURCE_ANCHORS: &[(&str, &str)] = &[
    ("src-local", "features-source-local"),
    ("src-jellyfin", "features-source-jellyfin"),
    ("src-navidrome", "features-source-navidrome"),
    ("src-subsonic", "features-source-subsonic"),
    ("src-nextcloud", "features-source-nextcloud"),
    ("src-ytmusic", "features-source-ytmusic"),
    ("src-applemusic", "features-source-applemusic"),
    ("src-soundcloud", "features-source-soundcloud"),
    ("src-spotify", "features-source-spotify"),
];

struct Row {
    /// Ids the home page links to. The first becomes the row id, the rest are
    /// bare anchors: several sources share the one row that describes them.
    anchors: &'static [&'static str],
    title_key: &'static str,
    desc_key: &'static str,
}

struct Group {
    id: &'static str,
    /// English name, also what the player bar reads out of `data-title`.
    label: &'static str,
    rows: &'static [Row],
}

const fn row(title_key: &'static str, desc_key: &'static str) -> Row {
    Row {
        anchors: &[],
        title_key,
        desc_key,
    }
}

const fn anchored(
    anchors: &'static [&'static str],
    title_key: &'static str,
    desc_key: &'static str,
) -> Row {
    Row {
        anchors,
        title_key,
        desc_key,
    }
}

const GROUPS: &[Group] = &[
    Group {
        id: "sources",
        label: "Sources",
        rows: &[
            anchored(
                &["src-local", "src-jellyfin", "src-navidrome", "src-subsonic"],
                "feat-local-title",
                "feat-local-desc",
            ),
            anchored(&["src-ytmusic"], "feat-youtube-title", "feat-youtube-desc"),
            anchored(
                &["src-applemusic"],
                "feat-applemusic-title",
                "feat-applemusic-desc",
            ),
            anchored(
                &["src-nextcloud"],
                "feat-nextcloud-title",
                "feat-nextcloud-desc",
            ),
            anchored(
                &["src-soundcloud"],
                "feat-soundcloud-title",
                "feat-soundcloud-desc",
            ),
            anchored(&["src-spotify"], "feat-spotify-title", "feat-spotify-desc"),
            row("feat-radio-title", "feat-radio-desc"),
        ],
    },
    Group {
        id: "playback",
        label: "Playback",
        rows: &[
            row("feat-lyrics-title", "feat-lyrics-desc"),
            row("feat-eq-title", "feat-eq-desc"),
            row("feat-crossfade-title", "feat-crossfade-desc"),
            row("feat-channels-title", "feat-channels-desc"),
            row("feat-offline-title", "feat-offline-desc"),
            row("feat-miniplayer-title", "feat-miniplayer-desc"),
        ],
    },
    Group {
        id: "library",
        label: "Library",
        rows: &[
            row("feat-fav-title", "feat-fav-desc"),
            row("feat-search-title", "feat-search-desc"),
            row("feat-genre-title", "feat-genre-desc"),
            row("feat-logs-title", "feat-logs-desc"),
            row("feat-cleanup-title", "feat-cleanup-desc"),
            row("feat-badges-title", "feat-badges-desc"),
            row("feat-metadata-title", "feat-metadata-desc"),
        ],
    },
    Group {
        id: "desktop",
        label: "Desktop",
        rows: &[
            row("feat-native-title", "feat-native-desc"),
            row("feat-tray-title", "feat-tray-desc"),
            row("feat-fonts-title", "feat-fonts-desc"),
            row("feat-theming-title", "feat-theming-desc"),
        ],
    },
    Group {
        id: "android",
        label: "Android",
        rows: &[row("feat-android-title", "feat-android-desc")],
    },
    Group {
        id: "integrations",
        label: "Integrations",
        rows: &[
            row("feat-scrobble-title", "feat-scrobble-desc"),
            row("feat-discord-title", "feat-discord-desc"),
            row("feat-ytdlp-title", "feat-ytdlp-desc"),
            row("feat-i18n-title", "feat-i18n-desc"),
            row("feat-debug-title", "feat-debug-desc"),
        ],
    },
    Group {
        id: "performance",
        label: "Performance",
        rows: &[
            row("perf-skip-label", "perf-skip-desc"),
            row("perf-parallel-label", "perf-parallel-desc"),
            row("perf-art-label", "perf-art-desc"),
            row("perf-lazy-label", "perf-lazy-desc"),
            row("perf-io-label", "perf-io-desc"),
            row("perf-http-label", "perf-http-desc"),
            row("perf-sort-label", "perf-sort-desc"),
        ],
    },
    Group {
        id: "privacy",
        label: "Privacy",
        rows: &[
            row("privacy-local-title", "privacy-local-desc"),
            row("privacy-accounts-title", "privacy-accounts-desc"),
            row("privacy-files-title", "privacy-files-desc"),
        ],
    },
];

fn group_label(group: &'static Group) -> Signal<String> {
    match group.id {
        "sources" => move_tr!("features-group-sources"),
        "playback" => move_tr!("features-group-playback"),
        "library" => move_tr!("features-group-library"),
        "desktop" => move_tr!("features-group-desktop"),
        "android" => move_tr!("features-group-android"),
        "integrations" => move_tr!("features-group-integrations"),
        "performance" => move_tr!("features-group-performance"),
        _ => move_tr!("features-group-privacy"),
    }
}

#[component]
pub(crate) fn Features() -> impl IntoView {
    let privacy_href = internal_href("/privacy");

    view! {
        <section class="sec features-head" id="features" data-title="Features">
            <div class="wrap">
                <div class="sec-head">
                    <h1>{move_tr!("features-title")}</h1>
                    <p class="lede">{move_tr!("features-chip")}</p>
                </div>
            </div>
        </section>
        <div class="sec features-body">
            <div class="wrap feat-layout">
                <nav class="feat-jump" aria-label=move_tr!("features-jump-aria")>
                    <ul>
                        {GROUPS.iter().map(|group| {
                            let href = internal_href(&format!("/features#{}", group.id));
                            view! { <li><a href=href>{group_label(group)}</a></li> }
                        }).collect_view()}
                    </ul>
                </nav>
                <div class="feat-groups">
                    {GROUPS.iter().map(|group| {
                        let privacy_href = privacy_href.clone();
                        view! {
                            <section class="feat-group" id=group.id data-title=group.label>
                                <h2>{group_label(group)}</h2>
                                <dl class="feat-list">
                                    {group.rows.iter().map(|row| view! {
                                        <div class="feat-row" id=row.anchors.first().copied()>
                                            <dt class="feat-name">
                                                {row.anchors.iter().skip(1).map(|anchor| view! {
                                                    <span class="feat-anchor" id=*anchor></span>
                                                }).collect_view()}
                                                {feature_title(row.title_key)}
                                            </dt>
                                            <dd class="feat-desc">{feature_desc(row.desc_key)}</dd>
                                        </div>
                                    }).collect_view()}
                                </dl>
                                {(group.id == "privacy").then(move || view! {
                                    <p class="feat-more">
                                        <a class="text-link" href=privacy_href>
                                            {move_tr!("features-privacy-link")}
                                            <Icon name="arrow-up-right" size=14/>
                                        </a>
                                    </p>
                                })}
                            </section>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

pub(crate) fn feature_title(key: &'static str) -> Signal<String> {
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
        "perf-skip-label" => move_tr!("perf-skip-label"),
        "perf-parallel-label" => move_tr!("perf-parallel-label"),
        "perf-art-label" => move_tr!("perf-art-label"),
        "perf-lazy-label" => move_tr!("perf-lazy-label"),
        "perf-io-label" => move_tr!("perf-io-label"),
        "perf-http-label" => move_tr!("perf-http-label"),
        "perf-sort-label" => move_tr!("perf-sort-label"),
        "privacy-local-title" => move_tr!("privacy-local-title"),
        "privacy-accounts-title" => move_tr!("privacy-accounts-title"),
        "privacy-files-title" => move_tr!("privacy-files-title"),
        _ => Signal::derive(String::new),
    }
}

pub(crate) fn feature_desc(key: &'static str) -> Signal<String> {
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
        "perf-skip-desc" => move_tr!("perf-skip-desc"),
        "perf-parallel-desc" => move_tr!("perf-parallel-desc"),
        "perf-art-desc" => move_tr!("perf-art-desc"),
        "perf-lazy-desc" => move_tr!("perf-lazy-desc"),
        "perf-io-desc" => move_tr!("perf-io-desc"),
        "perf-http-desc" => move_tr!("perf-http-desc"),
        "perf-sort-desc" => move_tr!("perf-sort-desc"),
        "privacy-local-desc" => move_tr!("privacy-local-desc"),
        "privacy-accounts-desc" => move_tr!("privacy-accounts-desc"),
        "privacy-files-desc" => move_tr!("privacy-files-desc"),
        _ => Signal::derive(String::new),
    }
}

#[cfg(test)]
mod features_tests {
    use super::*;

    #[test]
    fn every_source_anchor_is_rendered() {
        for (anchor, _) in SOURCE_ANCHORS {
            assert!(
                GROUPS
                    .iter()
                    .flat_map(|group| group.rows)
                    .any(|row| row.anchors.contains(anchor)),
                "no row carries the anchor: {anchor}"
            );
        }
    }
}
