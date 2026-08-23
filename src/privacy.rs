use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_meta::{Link, Meta, Title};

use crate::app::{provide_moe_theme, Footer, Nav};

const LAST_UPDATED: &str = "23 August 2026";

/// The privacy policy for both the Kopuz app and this website.
///
/// The body is deliberately not routed through Fluent. It is the authoritative
/// text of a legal document, and a machine translation that drifts from it
/// would be worse than an English original everyone can compare against. Only
/// the framing (title, the note explaining exactly that) is translated.
#[component]
pub fn PrivacyPage() -> impl IntoView {
    let moe = provide_moe_theme();

    view! {
        <Title text="Privacy Policy | Kopuz"/>
        <Meta name="description" content="How Kopuz handles your data: what stays on your device, what leaves it, and what the project receives."/>
        <Meta name="robots" content="index, follow"/>
        <Link rel="canonical" href="https://kopuz.moe/privacy"/>

        <div class="site" class:moe=move || moe.get()>
            <Nav/>
            <section class="legal">
                <h1>"Privacy Policy"</h1>
                <p class="legal-meta">"Last updated: " {LAST_UPDATED}</p>
                <p class="legal-note">{move_tr!("privacy-english-note")}</p>

                <h2>"The short version"</h2>
                <p>
                    "Kopuz is a music player that runs on your own computer or phone. It has no accounts, no sign-up, and no backend of ours. "
                    "The project runs no server that receives anything from the app, and the app contains no analytics, telemetry, advertising, or automatic crash reporting. "
                    "Your library, your listening history, and your credentials stay in files on your device."
                </p>
                <p>
                    "Kopuz does connect to the internet, because a music player that streams has to. Every one of those connections goes to a service "
                    "you chose to use, and it goes there directly, never through us. This page lists all of them."
                </p>

                <h2>"Who is responsible"</h2>
                <p>
                    "Kopuz is a free and open source project (MIT licensed) maintained by temidaradev and its contributors. "
                    "The source code is public at "
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" rel="noopener noreferrer">"github.com/Kopuz-org/kopuz"</a>
                    ", so every claim on this page can be checked against the code rather than taken on trust."
                </p>
                <p>
                    "This policy covers two things: the Kopuz application, and this website at kopuz.moe. They are treated separately below, "
                    "because they behave very differently."
                </p>

                <h2>"1. What Kopuz stores on your device"</h2>
                <p>"Kopuz keeps everything it knows in ordinary files that you own and can delete at any time."</p>
                <ul>
                    <li><strong>"Settings"</strong>" (" <code>"settings.toml"</code> ") holds your preferences: theme, audio setup, which sources you added, and which features you turned on."</li>
                    <li><strong>"The library database"</strong>" (" <code>"kopuz.db"</code> ") holds your scanned tracks, albums, playlists, favorites, play counts, and the queue you left playing."</li>
                    <li><strong>"Cached album art"</strong>" downloaded from your sources, so the same cover is not fetched twice."</li>
                    <li><strong>"Offline tracks"</strong>" you explicitly downloaded from a source that allows it."</li>
                    <li><strong>"Logs and crash reports"</strong>", covered in section 4."</li>
                    <li>
                        <strong>"Browser profiles"</strong>
                        ", for the sources that require signing in through a browser window. These hold the session cookies that sign-in produced, in a directory belonging to Kopuz."
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
                    "When you connect a service, the token or password you provide is stored in the local database so the app can reconnect without asking again. "
                    "It is stored in plain text, not encrypted, because Kopuz has no password of yours to encrypt it with and a key kept next to the data protects nobody. "
                    "It is protected by your operating system's file permissions and nothing else. This matters if you share a machine, sync your home directory, "
                    "or hand someone a backup: treat that database as a secret. Credentials are deliberately kept out of the settings file and out of exported settings, "
                    "so sharing your configuration does not share your accounts."
                </p>

                <h2>"2. What leaves your device"</h2>
                <p>
                    "Nothing below happens on a fresh install with a local music folder. Each connection starts only when you add the source, "
                    "connect the account, or turn on the feature it belongs to. When it does happen, your device talks to that service directly, and that service's own "
                    "privacy policy applies to what it does with the request."
                </p>

                <h3>"Music sources you add"</h3>
                <ul>
                    <li>
                        <strong>"Your own servers"</strong>": Jellyfin, Subsonic and compatible servers such as Navidrome, Nextcloud over WebDAV, or a custom endpoint. "
                        "Kopuz sends the address and credentials you entered to the address you entered. Nobody else is involved, including us."
                    </li>
                    <li>
                        <strong>"Streaming services"</strong>": YouTube Music, SoundCloud, Apple Music, and Spotify. Using one of these means your listening happens under your account "
                        "with that provider, subject to their terms and their privacy policy. Kopuz talks to their public endpoints and their content delivery hosts to fetch metadata, artwork, and audio."
                    </li>
                    <li>
                        <strong>"Internet radio"</strong>": the station you play receives a connection from you like any other stream. The optional station directory is a static file fetched from GitHub."
                    </li>
                </ul>
                <p>
                    "For Apple Music playback specifically, Kopuz needs a Widevine content decryption module. It looks for one already installed by a browser on your machine, "
                    "and can otherwise fetch it from Mozilla's plugin update service. Kopuz does not ship that module and does not send anything about you when locating it."
                </p>

                <h3>"Lyrics"</h3>
                <p>
                    "When lyrics are shown for a track that has none embedded, Kopuz asks a lyrics provider for them: LRCLIB, a synced lyrics service, "
                    "or Musixmatch when you enable that fallback. The request contains the track's artist, title, album, and duration, which is what the provider needs to find a match. "
                    "It contains nothing about you. Turning on \"prefer local lyrics\" stops online lookups entirely."
                </p>

                <h3>"Artwork and metadata"</h3>
                <p>
                    "Automatic cover fetching is off by default. When you turn it on, Kopuz queries MusicBrainz, Cover Art Archive, the iTunes search API, or Last.fm "
                    "with an artist and album name to find missing artwork."
                </p>

                <h3>"Scrobbling"</h3>
                <p>
                    "If you connect a Last.fm, Libre.fm, or ListenBrainz account, Kopuz submits what you play to that service: track, artist, album, and a timestamp. "
                    "That is the entire point of scrobbling, and it happens only for accounts you connected yourself. Disconnecting stops it immediately."
                </p>

                <h3>"Discord Rich Presence"</h3>
                <p>
                    "This is on by default and can be turned off in settings. When Discord is running on the same machine, Kopuz tells the local Discord client "
                    "what you are playing, and Discord shows it on your profile to whoever can see your profile. The track title, artist, album, and cover art are included. "
                    "Kopuz talks to the Discord app on your own machine, not to Discord's servers, but Discord then publishes it."
                </p>

                <h3>"Update check"</h3>
                <p>
                    "On launch, Kopuz asks GitHub for the latest release tag to tell you when a new version exists. The request carries no identifier: "
                    "GitHub sees an IP address and a user agent naming the Kopuz version, as it would for any download. You can turn the check off in settings."
                </p>
                <p class="legal-emph">
                    "No request Kopuz makes carries an identifier the project assigned to you, because the project never assigns one. There is no install ID, no device ID, and no account."
                </p>

                <h2>"3. What the Kopuz project receives"</h2>
                <p>
                    "Nothing from the app. We operate no service the app reports to, so there is no dataset of Kopuz users to leak, subpoena, sell, or lose. "
                    "What the project can see is what any open source project sees: public download counts from GitHub and the package repositories that distribute Kopuz, "
                    "and whatever you choose to write in an issue, a pull request, or the Discord server."
                </p>

                <h2>"4. Logs and crash reports"</h2>
                <p>
                    "Kopuz writes a log of each session to your device, keeps the last ten, and writes a crash report if it panics. These are diagnostic files, "
                    "and they exist so that a bug can be explained. They can contain file paths from your library, track and album names, server addresses, and the version of Kopuz and your operating system. "
                    "Credentials embedded in a server address are stripped before anything is written."
                </p>
                <p>
                    "They are never uploaded. They stay on disk until you delete them or they rotate out. If you attach an exported log to a bug report, that is a deliberate act of sharing, "
                    "and it is worth reading the file first, because your library's folder names are your business and nobody else's."
                </p>

                <h2>"5. This website"</h2>
                <p>"kopuz.moe is a static presentation of the project. It has no analytics, no tracking pixels, no advertising, and no third party trackers."</p>
                <ul>
                    <li>
                        <strong>"Cookies"</strong>": two, both first party, both holding a preference rather than an identifier. "
                        <code>"lf-lang"</code> " remembers the language you picked; " <code>"kopuz-moe"</code> " remembers which theme you picked. Neither identifies you, and clearing them costs you nothing but the preference."
                    </li>
                    <li>
                        <strong>"Server logs"</strong>": the web server processes the usual request data, including your IP address, in order to serve the page and keep the site running. It is not used to build a profile of you."
                    </li>
                    <li>
                        <strong>"Third party assets"</strong>": icons load from cdnjs, fonts from Google Fonts, and sponsor avatars from GitHub. Those hosts see your IP address and user agent when your browser fetches the file, "
                        "under their own privacy policies."
                    </li>
                    <li>
                        <strong>"Sponsors"</strong>": the sponsor list is built from the publicly visible sponsor page on GitHub. If you sponsor publicly you appear there, and here. "
                        "Sponsor privately, or ask us, and you do not."
                    </li>
                    <li>
                        <strong>"The " <code>"/j"</code> " handoff link"</strong>
                        ": a \"listen on Kopuz\" link carries its queue in the part of the URL after the " <code>"#"</code> ", which browsers never transmit to a server. "
                        "It is handed straight to the app on your machine. Even though the site is server rendered, what you are listening to never reaches the server or its logs. That was the point of building it that way."
                    </li>
                </ul>

                <h2>"6. Children"</h2>
                <p>
                    "Kopuz is not directed at children and collects nothing from anyone, of any age. Connected third party services set their own age requirements, "
                    "and those apply as they normally would."
                </p>

                <h2>"7. Your choices"</h2>
                <p>
                    "Every network feature described above has an off switch: remove the source, disconnect the account, or turn off the toggle. "
                    "Kopuz with a local music folder and nothing else configured makes no network requests except the update check, which is also a toggle."
                </p>
                <p>
                    "To erase everything Kopuz holds, delete the directories in section 1 and uninstall the app. There is no request to send us, no form to fill in, and no waiting period, "
                    "because there is no copy of it anywhere else. If a connected service holds data about your listening, such as a scrobbling service, that data is theirs to delete and you would ask them."
                </p>

                <h2>"8. Changes to this policy"</h2>
                <p>
                    "If Kopuz gains a feature that touches your data, this page changes with it, and the date at the top changes too. "
                    "The history of this page is in the website's public git repository, so you can see exactly what changed and when."
                </p>

                <h2>"9. Contact"</h2>
                <p>
                    "Questions about this policy, or something here that does not match what the code does, belong in an issue at "
                    <a href="https://github.com/Kopuz-org/kopuz/issues" target="_blank" rel="noopener noreferrer">"the issue tracker"</a>
                    " or in the "
                    <a href="https://discord.gg/K6Bmzw2E4M" target="_blank" rel="noopener noreferrer">"Discord server"</a>
                    ". A privacy report is a bug report, and it will be treated as one."
                </p>

                <p class="legal-back"><a href="/">{move_tr!("privacy-back")}</a></p>
            </section>
            <Footer/>
        </div>
    }
}
