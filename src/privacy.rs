use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_meta::{Link, Meta, Title};

use crate::app::{internal_href, provide_site_theme, Footer, Nav, ThemeColorMeta};

const LAST_UPDATED: &str = "23 August 2026";

/// The privacy policy for both the Kopuz app and this website.
///
/// The body is deliberately not routed through Fluent. It is the authoritative
/// text of a legal document, and a machine translation that drifts from it
/// would be worse than an English original everyone can compare against. Only
/// the framing (title, the note explaining exactly that) is translated.
#[component]
pub fn PrivacyPage() -> impl IntoView {
    let theme = provide_site_theme();
    let home_href = internal_href("/");

    view! {
        <Title text="Privacy Policy | Kopuz"/>
        <Meta name="description" content="How Kopuz handles your data: what stays on your device, what leaves it, and what the project receives."/>
        <Meta name="robots" content="index, follow"/>
        <Meta property="og:title" content="Privacy Policy | Kopuz"/>
        <Meta property="og:description" content="How Kopuz handles your data: what stays on your device, what leaves it, and what the project receives."/>
        <Meta property="og:url" content="https://kopuz.moe/privacy"/>
        <Meta name="twitter:title" content="Privacy Policy | Kopuz"/>
        <Meta name="twitter:description" content="How Kopuz handles your data: what stays on your device, what leaves it, and what the project receives."/>
        <Link rel="canonical" href="https://kopuz.moe/privacy"/>
        <ThemeColorMeta/>

        <div
            class="site"
            class:light=move || !theme.dark.get() && !theme.moe
            class:dark=move || theme.dark.get() && !theme.moe
            class:moe=move || theme.moe
        >
            <Nav/>
            <main>
            <section class="legal">
                <h1>"Privacy Policy"</h1>
                <p class="legal-meta">"Last updated: " {LAST_UPDATED}</p>
                <p class="legal-note">{move_tr!("privacy-english-note")}</p>

                <h2>"The short version"</h2>
                <p>
                    "Kopuz runs on your computer or phone. The app has no Kopuz account system, and the project does not operate an app backend. "
                    "It sends no analytics, telemetry, ads, or automatic crash reports. Library data, listening history, and credentials are stored on your device."
                </p>
                <p>
                    "Kopuz connects directly to the services described below, including GitHub for update checks. These requests do not pass through a Kopuz server."
                </p>
                <h2>"Who is responsible"</h2>
                <p>
                    "Kopuz is an MIT-licensed open source project maintained by temidaradev and its contributors. Compare this policy with the source code at "
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" rel="noopener noreferrer">"github.com/Kopuz-org/kopuz"</a>
                    "."
                </p>
                <p>
                    "Each section below states whether it applies to the Kopuz application or the website."
                </p>

                <h2>"1. What Kopuz stores on your device"</h2>
                <p>"Kopuz stores local data in files on your device. Their locations are listed below."</p>
                <ul>
                    <li><strong>"Settings"</strong>" (" <code>"settings.toml"</code> ") holds your preferences: theme, audio setup, which sources you added, and which features you turned on."</li>
                    <li><strong>"The library database"</strong>" (" <code>"kopuz.db"</code> ") holds your scanned tracks, albums, playlists, favorites, play counts, and the queue you left playing."</li>
                    <li><strong>"Cached album art"</strong>" downloaded from your sources, so the same cover is not fetched twice."</li>
                    <li><strong>"Offline tracks"</strong>" you explicitly downloaded from a source that allows it."</li>
                    <li><strong>"Logs and crash reports"</strong>", covered in section 4."</li>
                    <li>
                        <strong>"Browser profiles"</strong>
                        " are used for sources that require browser sign-in. They store the resulting session cookies in a Kopuz data directory."
                    </li>
                </ul>
                <p>"Where they live:"</p>
                <ul class="legal-paths">
                    <li>
                        <strong>"Linux"</strong>": " <code>"~/.config/kopuz/"</code> " and " <code>"~/.cache/kopuz/"</code>
                    </li>
                    <li>
                        <strong>"macOS"</strong>": " <code>"~/Library/Application Support/moe.kopuz.kopuz/"</code> " and " <code>"~/Library/Caches/moe.kopuz.kopuz/"</code>
                    </li>
                    <li>
                        <strong>"Windows"</strong>": " <code>"%APPDATA%\\kopuz\\kopuz\\config\\"</code> " and " <code>"%LOCALAPPDATA%\\kopuz\\kopuz\\cache\\"</code>
                    </li>
                    <li><strong>"Android"</strong>": the app's private storage, removed when you uninstall."</li>
                </ul>

                <h3>"Credentials"</h3>
                <p>
                    "Credentials are stored unencrypted in the local database and rely on your operating system's file permissions. "
                    "Anyone who can read or copy the database can access them. Credentials are excluded from the settings file and exported settings."
                </p>

                <h2>"2. What leaves your device"</h2>
                <p>
                    "Most connections start after you add a source, connect an account, or enable a feature. Update checks and Discord Rich Presence are enabled by default and can be disabled in settings. "
                    "Requests go directly to the relevant service, whose privacy policy applies."
                </p>

                <h3>"Music sources you add"</h3>
                <ul>
                    <li>
                        <strong>"Your own servers"</strong>": Jellyfin, Subsonic and compatible servers such as Navidrome, Nextcloud over WebDAV, or a custom endpoint. "
                        "Kopuz sends credentials directly to the configured server. The Kopuz project does not proxy the request."
                    </li>
                    <li>
                        <strong>"Streaming services"</strong>": YouTube Music, SoundCloud, Apple Music, and Spotify. Your activity is subject to the provider's terms and privacy policy. "
                        "Kopuz requests metadata, artwork, and audio directly from provider endpoints and content delivery hosts."
                    </li>
                    <li>
                        <strong>"Internet radio"</strong>": when you play a station, your device connects directly to its stream. The optional station directory is a static file fetched from GitHub."
                    </li>
                </ul>
                <p>
                    "Apple Music playback requires a Widevine content decryption module. Kopuz first looks for one installed by a browser and can otherwise fetch it from Mozilla's plugin update service. "
                    "Kopuz does not bundle the module. Requests to Mozilla have no Kopuz account or installation ID."
                </p>

                <h3>"Lyrics"</h3>
                <p>
                    "When a track has no embedded lyrics, Kopuz requests them from LRCLIB or, when enabled, Musixmatch. "
                    "The request includes the track's artist, title, album, and duration. Kopuz adds no account or installation ID. Enabling \"prefer local lyrics\" disables online lookups."
                </p>

                <h3>"Artwork and metadata"</h3>
                <p>
                    "Automatic cover fetching is off by default. When you turn it on, Kopuz queries MusicBrainz, Cover Art Archive, the iTunes search API, or Last.fm "
                    "with an artist and album name to find missing artwork."
                </p>

                <h3>"Scrobbling"</h3>
                <p>
                    "If you connect Last.fm, Libre.fm, or ListenBrainz, Kopuz submits the track, artist, album, and timestamp for each play. "
                    "Kopuz submits plays only while the account is connected. Disconnect the account to stop submissions."
                </p>

                <h3>"Discord Rich Presence"</h3>
                <p>
                    "Discord Rich Presence is enabled by default and can be disabled in settings. When enabled and Discord is running, Kopuz sends the track title, artist, album, and cover art to the local Discord client. "
                    "Discord may display that information according to your profile visibility."
                </p>

                <h3>"Update check"</h3>
                <p>
                    "On launch, Kopuz asks GitHub for the latest release tag. GitHub receives your IP address and a user agent naming the Kopuz version. "
                    "You can disable the check in settings."
                </p>
                <p class="legal-emph">
                    "Kopuz does not create an install ID, device ID, or account identifier."
                </p>

                <h2>"3. What the Kopuz project receives"</h2>
                <p>
                    "The app does not report to a Kopuz service. The project can access public download counts from GitHub and package repositories, "
                    "plus anything you submit to an issue, pull request, or the Discord server."
                </p>

                <h2>"4. Logs and crash reports"</h2>
                <p>
                    "Kopuz keeps the last ten session logs on your device and writes a crash report if it panics. These files may include library paths, track and album names, "
                    "server addresses, the Kopuz version, and the operating system version. Credentials embedded in server addresses are removed before logging."
                </p>
                <p>
                    "Logs and crash reports are not uploaded automatically. Review a log for file paths and library names before attaching it to a bug report."
                </p>

                <h2>"5. This website"</h2>
                <p>"kopuz.moe has no analytics, tracking pixels, advertising, or third-party trackers."</p>
                <ul>
                    <li>
                        <strong>"Cookies"</strong>": two first-party preference cookies. "
                        <code>"lf-lang"</code> " stores the selected language, and " <code>"kopuz-theme"</code> " stores the selected theme. Neither contains an account or installation ID."
                    </li>
                    <li>
                        <strong>"Server logs"</strong>": the web server processes the usual request data, including your IP address, in order to serve the page and keep the site running. It is not used to build a profile of you."
                    </li>
                    <li>
                        <strong>"Third party assets"</strong>": icons load from cdnjs, fonts from Google Fonts, and sponsor avatars from GitHub. Those hosts see your IP address and user agent when your browser fetches the file, "
                        "under their own privacy policies."
                    </li>
                    <li>
                        <strong>"Sponsors"</strong>": public GitHub sponsors appear on this site. Private sponsors do not."
                    </li>
                    <li>
                        <strong>"The " <code>"/j"</code> " handoff link"</strong>
                        ": a \"listen on Kopuz\" link stores its queue after the " <code>"#"</code> " in the URL. Browsers do not send that fragment to the server. "
                        "The browser passes it directly to the app."
                    </li>
                </ul>

                <h2>"6. Children"</h2>
                <p>
                    "Kopuz is not directed at children. Connected services set their own age requirements."
                </p>

                <h2>"7. Your choices"</h2>
                <p>
                    "Remove a source, disconnect an account, or disable a feature to stop its requests. A local-folder setup makes no network requests except the update check, which can also be disabled."
                </p>
                <p>
                    "To remove local Kopuz data, delete the directories in section 1 and uninstall the app. Data held by a connected service must be deleted through that service."
                </p>

                <h2>"8. Changes to this policy"</h2>
                <p>
                    "Kopuz revises this policy and its update date when a feature is added or changed in a way that affects data handling. The website's public Git history records each revision."
                </p>

                <h2>"9. Contact"</h2>
                <p>
                    "Report a discrepancy through "
                    <a href="https://github.com/Kopuz-org/kopuz/issues" target="_blank" rel="noopener noreferrer">"the issue tracker"</a>
                    " or the "
                    <a href="https://discord.gg/K6Bmzw2E4M" target="_blank" rel="noopener noreferrer">"Discord server"</a>
                    "."
                </p>

                <p class="legal-back"><a href=home_href>{move_tr!("privacy-back")}</a></p>
            </section>
            </main>
            <Footer/>
        </div>
    }
}
