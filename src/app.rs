use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/kopuz-website.css"/>
        <Title text="Kopuz — Music Player"/>
        <Meta name="description" content="Kopuz is a modern, lightweight music player built with Rust and Dioxus. Stream from Jellyfin or Navidrome, browse local files, synced lyrics, equalizer, themes, and more."/>
        <Meta name="keywords" content="Kopuz, music player, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, lyrics"/>
        <Meta name="author" content="temidaradev"/>
        <Meta name="robots" content="index, follow"/>
        <Meta name="theme-color" content="#32302f"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content="Kopuz — Music Player"/>
        <Meta property="og:description" content="Modern, lightweight music player built with Rust. Local files, Jellyfin, Navidrome, synced lyrics, equalizer, Discord RPC, and more. Free and open source."/>
        <Meta property="og:url" content="https://kopuz.temidara.rocks"/>
        <Meta property="og:image" content="https://kopuz.temidara.rocks/banner.png"/>
        <Meta property="og:image:alt" content="Kopuz music player"/>
        <Meta property="og:site_name" content="Kopuz"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content="Kopuz — Music Player"/>
        <Meta name="twitter:description" content="Modern, lightweight music player built with Rust. Free and open source."/>
        <Meta name="twitter:image" content="https://kopuz.temidara.rocks/banner.png"/>
        <Link rel="canonical" href="https://kopuz.temidara.rocks"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("gallery") view=GalleryPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="site">
            <Nav active="home"/>
            <Hero/>
            <Features/>
            <Performance/>
            <Install/>
            <Platforms/>
            <Support/>
            <Sponsors/>
            <Footer/>
        </div>
    }
}

#[component]
fn GalleryPage() -> impl IntoView {
    view! {
        <Title text="Gallery — Kopuz Music Player"/>
        <Meta name="description" content="Screenshots of Kopuz in action — home, library, playlist, fullscreen player, lyrics, theme editor, and more."/>
        <Meta property="og:title" content="Gallery — Kopuz Music Player"/>
        <Meta property="og:description" content="Screenshots of Kopuz in action — home, library, playlist, fullscreen player, lyrics, theme editor, and more."/>
        <Meta property="og:url" content="https://kopuz.temidara.rocks/gallery"/>
        <Link rel="canonical" href="https://kopuz.temidara.rocks/gallery"/>
        <div class="site">
            <Nav active="gallery"/>
            <GalleryContent/>
            <Footer/>
        </div>
    }
}

#[component]
fn Nav(#[prop(into)] active: String) -> impl IntoView {
    let is_gallery = active == "gallery";
    view! {
        <nav class="nav">
            <div class="nav-row">
                <a href="/" class="nav-logo">"Kopuz"</a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="nav-announce">
                    <span class="nav-new">"New"</span>
                    " Modern UI, Vaxry theme, yt-dlp downloader, 20+ languages"
                </a>
                <div class="nav-tabs">
                    <a href="/#features" class="nav-tab">"Features"</a>
                    <a href="/#install" class="nav-tab">"Install"</a>
                    <a href="/#downloads" class="nav-tab">"Download"</a>
                    <a href="/#sponsors" class="nav-tab">"Sponsors"</a>
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" class="nav-tab">"GitHub ↗"</a>
                    <a href="/gallery" class=move || if is_gallery { "nav-tab nav-tab-active" } else { "nav-tab" }>"Gallery"</a>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-left">
<h1>"Your music."<br/>"Your way."</h1>
                <p>
                    "Kopuz is a modern, lightweight music player built with Rust and Dioxus. "
                    "Scan local folders, stream from Jellyfin or Navidrome, and browse your library the way you want."
                </p>
                <div class="hero-ctas">
                    <a href="#downloads" class="btn-primary">"Download"</a>
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" class="btn-secondary">"View on GitHub"</a>
                </div>
            </div>
            <div class="hero-right">
                <img src="/normal-home.png" alt="Kopuz — home view" class="hero-screenshot"/>
            </div>
        </section>
    }
}

#[component]
fn Features() -> impl IntoView {
    view! {
        <section class="features" id="features">
            <div class="features-grid">
                <div class="feature-header-cell">
                    <h2>"Everything you need."</h2>
                    <span class="features-chip">"No subscriptions. No tracking. Just your music."</span>
                </div>
                <div class="features-sources-bar">
                    <span class="sources-label">"Works with"</span>
                    <div class="sources-list">
                        <span class="source-tag"><i class="fa-solid fa-folder-open"></i>" Local files"</span>
                        <span class="source-tag"><i class="fa-solid fa-server"></i>" Jellyfin"</span>
                        <span class="source-tag"><i class="fa-solid fa-server"></i>" Navidrome"</span>
                        <span class="source-tag"><i class="fa-solid fa-satellite-dish"></i>" Subsonic API"</span>
                    </div>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-music"></i>
                    <h3>"Local + Streaming"</h3>
                    <p>"Point at a local folder or connect to Jellyfin / Subsonic (Navidrome). Mix and match as you like."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-palette"></i>
                    <h3>"Theming"</h3>
                    <p>"Dynamic theming with full color variable control. Build your own theme from scratch or pick a preset."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-display"></i>
                    <h3>"Native Integration"</h3>
                    <p>"MPRIS on Linux, Now Playing on macOS, System Media Transport on Windows. Fully wired up."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-align-left"></i>
                    <h3>"Synced Lyrics"</h3>
                    <p>"Real-time scrolling lyrics, synced or plain, auto-scrolling to follow along with your music."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-sliders"></i>
                    <h3>"Equalizer"</h3>
                    <p>"5-band equalizer with built-in presets and full custom control over your sound."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-star"></i>
                    <h3>"Favorites & Playlists"</h3>
                    <p>"Star tracks locally or sync favorites with your server. Create playlists, add whole albums at once."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-tower-broadcast"></i>
                    <h3>"Scrobbling"</h3>
                    <p>"ListenBrainz scrobbling built in. Jellyfin users can also use the listenbrainz plugin."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-brands fa-discord"></i>
                    <h3>"Discord RPC"</h3>
                    <p>"Show friends what you're listening to with embedded Discord Rich Presence. No setup needed."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-magnifying-glass"></i>
                    <h3>"Search"</h3>
                    <p>"Real-time search across artists, albums, and tracks. Instant results as you type."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-tags"></i>
                    <h3>"Genre Browsing"</h3>
                    <p>"Browse your entire library by genre — works for both local files and server music."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-clock"></i>
                    <h3>"Listening Logs"</h3>
                    <p>"Play counts tracked locally. See what you actually listen to most over time."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-globe"></i>
                    <h3>"i18n"</h3>
                    <p>"English, Russian, German, French, Spanish, Turkish, Ukrainian, Polish, Arabic, Greek, Hebrew, Hungarian, Indonesian, Japanese, Korean, Romanian, Portuguese, Toki Pona, Chinese, and more."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-brands fa-youtube"></i>
                    <h3>"yt-dlp Downloader"</h3>
                    <p>"Download audio directly from YouTube. Output as MP3, FLAC, WAV, MP4, or best quality. SponsorBlock, chapter splitting, and rate limiting included."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-shuffle"></i>
                    <h3>"Crossfade & Transitions"</h3>
                    <p>"Blend track transitions for smoother playback. Crossfade support on native desktop builds."</p>
                </div>
                <div class="feature-card">
                    <i class="feature-icon fa-solid fa-headphones"></i>
                    <h3>"Channel Modes"</h3>
                    <p>"Stereo, Mono, Left-only, Right-only, and L/R swap. Fine-grained audio channel control."</p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Performance() -> impl IntoView {
    view! {
        <section class="perf" id="performance">
            <div class="section-header">
                <h2>"Built to be fast."</h2>
                <p>"Large libraries. Instant startup. No freezes."</p>
            </div>
            <div class="perf-grid">
                <div class="perf-item">
                    <span class="perf-label">"Skip already indexed"</span>
                    <p>"Rescans only process new files. 10k tracks + 5 new = only 5 read."</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">"Parallel startup"</span>
                    <p>"Library, config, playlists, and favorites all load in parallel with " <code>"tokio::join!"</code> ". Near-instant open."</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">"Album art caching"</span>
                    <p>"Covers extracted once, saved to disk. Never re-decoded on repeat views."</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">"Lazy image loading"</span>
                    <p>"Hundreds of album covers in search results — none load until they're in view."</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">"Non-blocking I/O"</span>
                    <p>"Heavy work runs on " <code>"spawn_blocking"</code> " threads. UI stays responsive during full library scans."</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">"HTTP art caching"</span>
                    <p>"Custom " <code>"artwork://"</code> " protocol serves covers with 1-year cache headers. Webview never re-fetches."</p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Install() -> impl IntoView {
    view! {
        <section class="install" id="install">
            <div class="section-header">
                <h2>"Installation"</h2>
            </div>
            <div class="install-grid">
                <div class="install-card">
                    <h3>"Nix / NixOS"</h3>
                    <p>"Run without installing:"</p>
                    <pre><code>"nix run github:temidaradev/kopuz"</code></pre>
                    <p>"Or add to your profile:"</p>
                    <pre><code>"nix profile add github:temidaradev/kopuz"</code></pre>
                    <p class="install-note">"NixOS flake supported with Cachix binary cache."</p>
                </div>
                <div class="install-card">
                    <h3>"Flatpak"</h3>
                    <p>"Install from source manifest:"</p>
                    <pre><code>"git clone https://github.com/temidaradev/kopuz
cd kopuz
flatpak-builder --user --install --force-clean \\
  build-dir com.temidaradev.kopuz.json
flatpak run com.temidaradev.kopuz"</code></pre>
                    <p class="install-note">"Flathub listing coming soon."</p>
                </div>
                <div class="install-card">
                    <h3>"AppImage"</h3>
                    <p>"Download from GitHub Releases and run directly. Requires " <code>"webkit2gtk-4.1"</code> " and " <code>"gtk3"</code> " on your system."</p>
                    <p class="install-note">"Arch users: if it crashes with a WebKitNetworkProcess error, run with " <code>"LD_LIBRARY_PATH=/usr/lib"</code> " prefix."</p>
                </div>
                <div class="install-card">
                    <h3>"macOS " <span class="install-chip">"Apple Silicon"</span></h3>
                    <p>"Download the " <code>".dmg"</code> " from GitHub Releases. If macOS blocks it, clear the quarantine flag:"</p>
                    <pre><code>"xattr -d com.apple.quarantine /Applications/Kopuz.app"</code></pre>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Platforms() -> impl IntoView {
    view! {
        <section class="platforms" id="downloads">
            <div class="section-header">
                <h2>"Download Kopuz"</h2>
                <p>"Free and open source. All releases on GitHub."</p>
            </div>
            <div class="platform-grid">
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-windows platform-os-icon"></i>
                        <span class="platform-name">"Windows"</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".exe"</span>
                    </div>
                    <span class="platform-dl">"Download →"</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-apple platform-os-icon"></i>
                        <span class="platform-name">"macOS"</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".dmg"</span>
                    </div>
                    <span class="platform-note">"Apple Silicon only"</span>
                    <span class="platform-dl">"Download →"</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-linux platform-os-icon"></i>
                        <span class="platform-name">"Linux"</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".AppImage"</span>
                        <span class="platform-fmt">".deb"</span>
                        <span class="platform-fmt">".rpm"</span>
                        <span class="platform-fmt">"Flatpak"</span>
                        <span class="platform-fmt">"Nix"</span>
                    </div>
                    <span class="platform-dl">"Download →"</span>
                </a>
            </div>
        </section>
    }
}

#[component]
fn Support() -> impl IntoView {
    view! {
        <section class="support" id="support">
            <div class="section-header">
                <h2>"Support Kopuz"</h2>
                <p>"Kopuz is free and open source. Support keeps it alive."</p>
            </div>
            <div class="support-links">
                <a href="https://github.com/sponsors/temidaradev" target="_blank" class="support-btn support-gh">
                    <i class="fa-solid fa-heart"></i>
                    "GitHub Sponsors"
                </a>
                <a href="https://buymeacoffee.com/temidaradev" target="_blank" class="support-btn support-bmc">
                    <i class="fa-solid fa-mug-hot"></i>
                    "Buy Me a Coffee"
                </a>
            </div>
            <div class="donate-divider">"— or send crypto —"</div>
            <div class="donate-grid">
                <div class="donate-item">
                    <span class="donate-coin">"SOL"</span>
                    <code>"BK84dVEMnGBP5Tya2mEaB1BQgcSBjngf1NBmRCqefxGg"</code>
                </div>
                <div class="donate-item">
                    <span class="donate-coin">"BTC"</span>
                    <code>"bc1qz94yz9xvufa6hxlvjzaajgd2zyfu86arn68hu4"</code>
                </div>
                <div class="donate-item">
                    <span class="donate-coin">"XMR"</span>
                    <code>"86mz3HxTrKyYpuvx78m6pufbXdwAnoyoZBztz6HyYrnM1XP5YVrMy9jTVRY5vzgGtkizACLpFwHEdafKTMoj6y8mAVgvWMz"</code>
                </div>
                <div class="donate-item">
                    <span class="donate-coin">"ETH"</span>
                    <code>"0xa490D50470cdFf837B6663F7f6cBe50B157224e5"</code>
                </div>
                <div class="donate-item">
                    <span class="donate-coin">"USDT"</span>
                    <code>"GYmnAcrA5MbF6cUxT2m5d5cwdfr14qSY9WFYRwXxaibW"</code>
                    <span class="donate-note">"(Solana chain)"</span>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Sponsors() -> impl IntoView {
    view! {
        <section class="sponsors" id="sponsors">
            <div class="section-header">
                <h2>"Sponsors"</h2>
                <p>"People keeping Kopuz going."</p>
            </div>
            <div class="sponsors-grid">
                <a href="https://github.com/shytzedaka" target="_blank" class="sponsor-card">
                    <img src="https://github.com/shytzedaka.png?size=80" alt="shytzedaka"/>
                    <span>"shytzedaka"</span>
                </a>
            </div>
            <div class="sponsors-cta">
                <a href="https://github.com/sponsors/temidaradev" target="_blank" class="btn-secondary">"Become a Sponsor"</a>
            </div>
        </section>
    }
}

static GALLERY_IMAGES: &[(&str, &str)] = &[
    ("/normal-home.png",     "Normal — Home"),
    ("/modern-home.png",     "Modern — Home"),
    ("/normal-library.png",  "Normal — Library"),
    ("/vaxry-library.png",   "Vaxry — Library"),
    ("/normal-playlist.png", "Normal — Playlist"),
    ("/modern-playlist.png", "Modern — Playlist"),
    ("/fullscreen.png",      "Fullscreen player"),
    ("/fullscreen-lyrics.png","Fullscreen lyrics"),
    ("/search.png",          "Search"),
    ("/theme-editor.png",    "Theme editor"),
    ("/player-settings.png", "Player settings"),
    ("/downloader.png",      "Downloader"),
];

#[component]
fn GalleryContent() -> impl IntoView {
    let current: RwSignal<Option<usize>> = RwSignal::new(None);

    let go_prev = move || {
        current.update(|c| {
            if let Some(i) = c {
                *i = if *i == 0 { GALLERY_IMAGES.len() - 1 } else { *i - 1 };
            }
        });
    };
    let go_next = move || {
        current.update(|c| {
            if let Some(i) = c {
                *i = (*i + 1) % GALLERY_IMAGES.len();
            }
        });
    };

    view! {
        <section class="gallery-section">
            <div class="section-header">
                <h2>"Gallery"</h2>
                <p>"Screenshots of Kopuz — Normal and Modern styles."</p>
            </div>
            <div class="gallery-grid">
                // Home — 2 styles
                <div class="gallery-item">
                    <div class="gallery-thumbs">
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(0))>
                            <img src="/normal-home.png" alt="Normal — Home"/>
                        </div>
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(1))>
                            <img src="/modern-home.png" alt="Modern — Home"/>
                        </div>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Home"</span>
                        <span class="gallery-desc">"Normal / Modern"</span>
                    </div>
                </div>
                // Library — 2 styles
                <div class="gallery-item">
                    <div class="gallery-thumbs">
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(2))>
                            <img src="/normal-library.png" alt="Normal — Library"/>
                        </div>
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(3))>
                            <img src="/vaxry-library.png" alt="Vaxry — Library"/>
                        </div>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Library"</span>
                        <span class="gallery-desc">"Normal / Vaxry"</span>
                    </div>
                </div>
                // Playlist — 2 styles
                <div class="gallery-item">
                    <div class="gallery-thumbs">
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(4))>
                            <img src="/normal-playlist.png" alt="Normal — Playlist"/>
                        </div>
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(5))>
                            <img src="/modern-playlist.png" alt="Modern — Playlist"/>
                        </div>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Playlist"</span>
                        <span class="gallery-desc">"Normal / Modern"</span>
                    </div>
                </div>
                // Singles
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(6))>
                        <img src="/fullscreen.png" alt="Fullscreen player"/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Fullscreen player"</span>
                        <span class="gallery-desc">"Immersive full-window playback view."</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(7))>
                        <img src="/fullscreen-lyrics.png" alt="Fullscreen lyrics"/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Fullscreen lyrics"</span>
                        <span class="gallery-desc">"Synced lyrics in full-window mode."</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(8))>
                        <img src="/search.png" alt="Search"/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Search"</span>
                        <span class="gallery-desc">"Real-time search across artists, albums, and tracks."</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(9))>
                        <img src="/theme-editor.png" alt="Theme editor"/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Theme editor"</span>
                        <span class="gallery-desc">"Full color variable control. Build or pick a preset."</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(10))>
                        <img src="/player-settings.png" alt="Player settings"/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Player settings"</span>
                        <span class="gallery-desc">"Configure audio, behavior, integrations, and more."</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(11))>
                        <img src="/downloader.png" alt="Downloader"/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">"Downloader"</span>
                        <span class="gallery-desc">"Download tracks directly from within Kopuz."</span>
                    </div>
                </div>
            </div>
        </section>

        <Show when=move || current.get().is_some()>
            <div class="lightbox" on:click=move |_| current.set(None)>
                <div class="lightbox-box" on:click=move |ev| ev.stop_propagation()>
                    <div class="lightbox-topbar">
                        {move || {
                            let idx = current.get().unwrap_or(0);
                            let (_src, label) = GALLERY_IMAGES[idx];
                            view! {
                                <span class="lightbox-label">{label}</span>
                            }
                        }}
                        <span class="lightbox-counter">
                            {move || {
                                let idx = current.get().unwrap_or(0);
                                format!("{} / {}", idx + 1, GALLERY_IMAGES.len())
                            }}
                        </span>
                        <button class="lightbox-close" on:click=move |ev| { ev.stop_propagation(); current.set(None); }>"×"</button>
                    </div>
                    {move || {
                        let idx = current.get().unwrap_or(0);
                        let (src, label) = GALLERY_IMAGES[idx];
                        view! { <img src=src alt=label class="lightbox-img"/> }
                    }}
                    <div class="lightbox-nav">
                        <button class="lightbox-btn" on:click=move |ev| { ev.stop_propagation(); go_prev(); }>"← Prev"</button>
                        <button class="lightbox-btn" on:click=move |ev| { ev.stop_propagation(); go_next(); }>"Next →"</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer-left">
                <span class="footer-logo">"Kopuz"</span>
                <span>"MIT License — Free & Open Source"</span>
            </div>
            <div class="footer-links">
                <a href="https://github.com/Kopuz-org/kopuz" target="_blank">"GitHub"</a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank">"Releases"</a>
                <a href="https://github.com/Kopuz-org/kopuz/issues" target="_blank">"Issues"</a>
                <a href="https://discord.gg/K6Bmzw2E4M" target="_blank">"Discord"</a>
            </div>
        </footer>
    }
}
