use leptos::prelude::*;
use leptos_fluent::move_tr;

use crate::icons::Icon;
use crate::releases::{use_latest_release, ReleaseAsset};

const RELEASES: &str = "https://github.com/Kopuz-org/kopuz/releases";

/// Row order on the download page and on the home page shortlist.
const ORDER: [&str; 4] = ["linux", "macos", "windows", "android"];

pub(crate) struct Platform {
    pub(crate) id: &'static str,
    /// Lucide icon name. Desktop targets share the monitor glyph.
    pub(crate) icon: &'static str,
    pub(crate) name_key: &'static str,
    /// Release asset extensions, in button order. The first one is what the
    /// hero button downloads, unless the platform has a store.
    pub(crate) extensions: &'static [&'static str],
    pub(crate) note_key: Option<&'static str>,
    /// Store name and listing URL, shown after the files.
    pub(crate) store: Option<(&'static str, &'static str)>,
}

pub(crate) const PLATFORMS: &[Platform] = &[
    Platform {
        id: "windows",
        icon: "monitor",
        name_key: "platforms-windows",
        extensions: &["exe", "msi"],
        note_key: None,
        store: None,
    },
    Platform {
        id: "macos",
        icon: "monitor",
        name_key: "platforms-macos",
        extensions: &["dmg"],
        note_key: Some("platforms-macos-note"),
        store: None,
    },
    Platform {
        id: "linux",
        icon: "monitor",
        name_key: "platforms-linux",
        extensions: &["appimage", "deb", "rpm"],
        note_key: None,
        store: Some(("Flathub", "https://flathub.org/apps/moe.kopuz.kopuz")),
    },
    Platform {
        id: "android",
        icon: "smartphone",
        name_key: "platforms-android",
        extensions: &["apk"],
        note_key: Some("platforms-android-note"),
        store: Some((
            "Google Play",
            "https://play.google.com/store/apps/details?id=moe.kopuz.kopuz",
        )),
    },
];

fn extension(name: &str) -> String {
    name.rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
}

/// The platform's release files, in the order its extensions are listed.
fn files_for<'a>(entry: &Platform, assets: &'a [ReleaseAsset]) -> Vec<&'a ReleaseAsset> {
    entry
        .extensions
        .iter()
        .flat_map(|ext| {
            assets
                .iter()
                .filter(move |asset| extension(&asset.name) == *ext)
        })
        .collect()
}

fn file_label(ext: &str) -> Signal<String> {
    match ext {
        "exe" => move_tr!("dl-file-exe"),
        "msi" => move_tr!("dl-file-msi"),
        "dmg" => move_tr!("dl-file-dmg"),
        "appimage" => move_tr!("dl-file-appimage"),
        "deb" => move_tr!("dl-file-deb"),
        "rpm" => move_tr!("dl-file-rpm"),
        _ => move_tr!("dl-file-apk"),
    }
}

/// The hero's download button opens this instead of leaving the page: every
/// installer for the latest release, the visitor's platform first.
#[component]
pub(crate) fn DownloadDialog() -> impl IntoView {
    let release = use_latest_release();

    view! {
        <dialog class="dl-dialog" aria-labelledby="dl-dialog-title" tabindex="-1" data-download-dialog>
            <div class="dl-dialog-head">
                <h2 id="dl-dialog-title">{move_tr!("platforms-title")}</h2>
                <button
                    type="button"
                    class="btn btn-ghost btn-icon"
                    aria-label=move_tr!("dl-dialog-close")
                    data-close-dialog
                >
                    <Icon name="x" size=16/>
                </button>
            </div>
            <Transition fallback=|| ()>
                {move || {
                    release
                        .get()
                        .map(|release| release.tag_name)
                        .filter(|version| !version.is_empty())
                        .map(|version| view! {
                            <p class="dl-dialog-status">
                                {move_tr!("dl-dialog-ready", { "version" => version.clone() })}
                            </p>
                        })
                }}
            </Transition>
            <Installers/>
            <a class="text-link dl-dialog-release" href=RELEASES target="_blank" rel="noopener noreferrer">
                {move_tr!("dl-dialog-release")}
                <Icon name="arrow-up-right" size=14/>
            </a>
        </dialog>
    }
}

/// One row per platform with a button per installer file, the visitor's own
/// platform first. Without the release (GitHub down or rate limited) each row
/// falls back to the releases page.
#[component]
pub(crate) fn Installers() -> impl IntoView {
    let release = use_latest_release();

    view! {
        <Transition fallback=|| view! { <InstallerRows assets=Vec::new()/> }>
            {move || release.get().map(|release| view! { <InstallerRows assets=release.assets/> })}
        </Transition>
    }
}

#[component]
fn InstallerRows(assets: Vec<ReleaseAsset>) -> impl IntoView {
    view! {
        <ul class="dl-list">
            {ORDER.into_iter().map(|id| {
                let entry = platform(id);
                let name = platform_text(entry.name_key);
                let note = entry.note_key.map(platform_text);
                let files = files_for(entry, &assets)
                    .into_iter()
                    .map(|asset| {
                        let label = file_label(&extension(&asset.name));
                        view! {
                            <a class="dl-file" href=asset.url.clone() title=asset.name.clone()>
                                <Icon name="download" size=14/>
                                {label}
                            </a>
                        }
                    })
                    .collect::<Vec<_>>();
                let fallback = files.is_empty().then(|| view! {
                    <a class="dl-file" href=RELEASES target="_blank" rel="noopener noreferrer">
                        {move_tr!("dl-release-page")}
                        <Icon name="arrow-up-right" size=14/>
                    </a>
                });
                let store = entry.store.map(|(store, href)| view! {
                    <a class="dl-file dl-store" href=href target="_blank" rel="noopener noreferrer">
                        {store}
                        <Icon name="arrow-up-right" size=14/>
                    </a>
                });
                view! {
                    <li class="dl-row" data-os=id>
                        <Icon name=entry.icon/>
                        <div class="dl-row-copy">
                            <span class="dl-name">
                                {name}
                                <span class="dl-this">{move_tr!("dl-this-device")}</span>
                            </span>
                            {note.map(|note| view! { <span class="dl-note">{note}</span> })}
                        </div>
                        <div class="dl-files">
                            {files}
                            {fallback}
                            {store}
                        </div>
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}

pub(crate) fn platform(id: &str) -> &'static Platform {
    PLATFORMS
        .iter()
        .find(|entry| entry.id == id)
        .expect("platform id is a compile-time constant from PLATFORMS")
}

pub(crate) fn platform_text(key: &'static str) -> Signal<String> {
    match key {
        "platforms-windows" => move_tr!("platforms-windows"),
        "platforms-macos" => move_tr!("platforms-macos"),
        "platforms-macos-note" => move_tr!("platforms-macos-note"),
        "platforms-linux" => move_tr!("platforms-linux"),
        "platforms-android" => move_tr!("platforms-android"),
        "platforms-android-note" => move_tr!("platforms-android-note"),
        _ => Signal::derive(String::new),
    }
}

#[component]
pub(crate) fn Platforms() -> impl IntoView {
    let release = use_latest_release();
    let release_label = move_tr!("hero-release");
    let release_line = move || {
        let version = release
            .get()
            .map(|release| release.version().to_string())
            .unwrap_or_default();
        if version.is_empty() {
            String::new()
        } else {
            format!("{} v{version}", release_label.get())
        }
    };

    view! {
        <section class="sec dl-head" id="downloads" data-title="Download">
            <div class="wrap">
                <h1>{move_tr!("platforms-title")}</h1>
                <Transition fallback=|| ()>
                    {move || {
                        let line = release_line();
                        (!line.is_empty())
                            .then(|| view! { <p class="dl-version tabular">{line}</p> })
                    }}
                </Transition>
                <Installers/>
            </div>
        </section>
    }
}

#[component]
fn Snippet(code: &'static str) -> impl IntoView {
    view! {
        <div class="snip">
            <pre><code>{code}</code></pre>
            <button
                type="button"
                class="snip-copy"
                data-copy=code
                data-copied-label=move_tr!("copied")
            >{move_tr!("copy")}</button>
        </div>
    }
}

#[component]
pub(crate) fn Install() -> impl IntoView {
    view! {
        <section class="sec install-sec reveal-sec" id="install" data-title="Install">
            <div class="wrap">
                <h2>{move_tr!("install-other-title")}</h2>
                <div class="install-list">
                    <div class="install-block">
                        <h3>{move_tr!("install-cargo-title")}</h3>
                        <p class="prose">{move_tr!("install-cargo-desc")}</p>
                        <Snippet code="cargo install --locked kopuz"/>
                    </div>
                    <div class="install-block">
                        <h3>{move_tr!("install-nix-title")}</h3>
                        <p class="prose">{move_tr!("install-nix-run")}</p>
                        <Snippet code="nix run github:temidaradev/kopuz"/>
                        <p class="prose">{move_tr!("install-nix-profile")}</p>
                        <Snippet code="nix profile add github:temidaradev/kopuz"/>
                        <p class="install-note">{move_tr!("install-nix-note")}</p>
                    </div>
                    <div class="install-block">
                        <h3>{move_tr!("install-aur-title")}</h3>
                        <p class="prose">{move_tr!("install-aur-desc")}</p>
                        <Snippet code="yay -S kopuz-bin\n# or\nparu -S kopuz-bin"/>
                    </div>
                    <div class="install-block">
                        <h3>{move_tr!("install-flatpak-title")}</h3>
                        <p class="prose">{move_tr!("install-flatpak-desc")}</p>
                        <Snippet code="flatpak install flathub moe.kopuz.kopuz"/>
                        <p class="install-note">{move_tr!("install-flatpak-note")}</p>
                    </div>
                    <div class="install-block">
                        <h3>{move_tr!("install-macos-title")}</h3>
                        <p class="prose">{move_tr!("install-macos-homebrew")}</p>
                        <Snippet code="brew install --cask --no-quarantine kopuz-org/tap/kopuz"/>
                        <p class="prose">
                            {move_tr!("install-macos-desc-1")}" "<code>".dmg"</code>
                            {move_tr!("install-macos-desc-2")}
                        </p>
                        <Snippet code="xattr -d com.apple.quarantine /Applications/Kopuz.app"/>
                    </div>
                    <div class="install-block">
                        <h3>{move_tr!("install-android-title")}</h3>
                        <p class="prose">{move_tr!("install-android-desc")}</p>
                        <a
                            class="text-link"
                            href="https://play.google.com/store/apps/details?id=moe.kopuz.kopuz"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            {move_tr!("install-android-play")}
                            <Icon name="arrow-up-right" size=14/>
                        </a>
                        <p class="install-note">{move_tr!("install-android-note")}</p>
                    </div>
                    <div class="install-block">
                        <h3>{move_tr!("install-appimage-title")}</h3>
                        <p class="prose">
                            {move_tr!("install-appimage-desc-1")}" "<code>"webkit2gtk-4.1"</code>
                            {move_tr!("install-appimage-desc-2")}" "<code>"gtk3"</code>
                            {move_tr!("install-appimage-desc-3")}
                        </p>
                        <p class="install-note">
                            {move_tr!("install-appimage-note-1")}" "
                            <code>"LD_LIBRARY_PATH=/usr/lib"</code>
                            {move_tr!("install-appimage-note-2")}
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
pub(crate) fn Requirements() -> impl IntoView {
    view! {
        <section class="sec req-sec reveal-sec" id="requirements" data-title="Requirements">
            <div class="wrap">
                <h2>{move_tr!("requirements-title")}</h2>
                <ul class="req-list">
                    <li><strong>"Spotify"</strong>" "{move_tr!("req-spotify")}</li>
                    <li><strong>"Apple Music"</strong>" "{move_tr!("req-applemusic")}</li>
                    <li><strong>"Nextcloud"</strong>" "{move_tr!("req-nextcloud")}</li>
                    <li><strong>"Android"</strong>" "{move_tr!("req-android")}</li>
                    <li><strong>"AppImage"</strong>" "{move_tr!("req-appimage")}</li>
                    <li><strong>"YouTube Music"</strong>" "{move_tr!("req-ytmusic")}</li>
                    <li><strong>"Crossfade"</strong>" "{move_tr!("req-crossfade")}</li>
                    <li><strong>{move_tr!("req-spotify-limits-label")}</strong>" "{move_tr!("req-spotify-limits")}</li>
                </ul>
            </div>
        </section>
    }
}
