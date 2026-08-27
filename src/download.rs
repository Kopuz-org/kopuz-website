use leptos::prelude::*;
use leptos_fluent::move_tr;

use crate::icons::Icon;
use crate::releases::use_latest_release;

const RELEASES: &str = "https://github.com/Kopuz-org/kopuz/releases";

/// Row order on the download page and on the home page shortlist.
const ORDER: [&str; 4] = ["linux", "macos", "windows", "android"];

pub(crate) struct Platform {
    pub(crate) id: &'static str,
    /// Lucide icon name. Desktop targets share the monitor glyph.
    pub(crate) icon: &'static str,
    pub(crate) name_key: &'static str,
    pub(crate) formats: &'static [&'static str],
    pub(crate) note_key: Option<&'static str>,
    pub(crate) href: &'static str,
}

pub(crate) const PLATFORMS: &[Platform] = &[
    Platform {
        id: "windows",
        icon: "monitor",
        name_key: "platforms-windows",
        formats: &[".exe"],
        note_key: None,
        href: RELEASES,
    },
    Platform {
        id: "macos",
        icon: "monitor",
        name_key: "platforms-macos",
        formats: &[".dmg"],
        note_key: Some("platforms-macos-note"),
        href: RELEASES,
    },
    Platform {
        id: "linux",
        icon: "monitor",
        name_key: "platforms-linux",
        formats: &[".AppImage", ".deb", ".rpm", "Flatpak", "Nix"],
        note_key: None,
        href: RELEASES,
    },
    Platform {
        id: "android",
        icon: "smartphone",
        name_key: "platforms-android",
        formats: &[".apk"],
        note_key: Some("platforms-android-note"),
        href: RELEASES,
    },
];

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

fn os_display(id: &str) -> &'static str {
    match id {
        "linux" => "Linux",
        "macos" => "macOS",
        "windows" => "Windows",
        _ => "Android",
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
                <p class="lede">{move_tr!("platforms-subtitle")}</p>
                <div class="dl-action">
                    <a
                        class="btn btn-primary"
                        href=RELEASES
                        target="_blank"
                        rel="noopener noreferrer"
                        data-os-cta
                    >
                        <Icon name="download" size=15/>
                        <span class="os-generic">{move_tr!("download-latest-cta")}</span>
                        {ORDER.into_iter().map(|id| view! {
                            <span class="os-label" data-os=id>
                                {move_tr!("hero-cta-download-for")}" "{os_display(id)}
                            </span>
                        }).collect_view()}
                    </a>
                    <Transition fallback=|| ()>
                        {move || {
                            let line = release_line();
                            (!line.is_empty())
                                .then(|| view! { <span class="dl-version tabular">{line}</span> })
                        }}
                    </Transition>
                </div>
                <ul class="dl-list">
                    {ORDER.into_iter().map(|id| {
                        let entry = platform(id);
                        let name = platform_text(entry.name_key);
                        let note = entry.note_key.map(platform_text);
                        let formats = entry.formats.join(" · ");
                        view! {
                            <li class="dl-row">
                                <Icon name=entry.icon/>
                                <div class="dl-row-copy">
                                    <span class="dl-name">{name}</span>
                                    <span class="dl-formats">{formats}</span>
                                    {note.map(|note| view! { <span class="dl-note">{note}</span> })}
                                </div>
                                <a
                                    class="text-link dl-get"
                                    href=entry.href
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    {move_tr!("home-get-download")}
                                    <Icon name="arrow-up-right" size=14/>
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
        <section class="sec install-sec" id="install" data-title="Install">
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
                        <p class="install-note">
                            {move_tr!("install-aur-note-1")}" "<code>"dioxus-cli"</code>
                            {move_tr!("install-aur-note-2")}
                        </p>
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
                            href=RELEASES
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            {move_tr!("home-get-download")}
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
        <section class="sec req-sec" id="requirements" data-title="Requirements">
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
