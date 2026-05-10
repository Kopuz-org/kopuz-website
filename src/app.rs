use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
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
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="site">
            <Nav/>
            <Hero/>
            <Features/>
            <Performance/>
            <Install/>
            <Downloads/>
            <Support/>
            <Sponsors/>
            <Footer/>
        </div>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav class="nav">
            <span class="nav-logo">"Kopuz"</span>
            <div class="nav-links">
                <a href="#features">"Features"</a>
                <a href="#install">"Install"</a>
                <a href="#downloads">"Download"</a>
                <a href="#sponsors">"Sponsors"</a>
                <a href="https://github.com/Kopuz-org/kopuz" target="_blank">"GitHub"</a>
            </div>
        </nav>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-badge">"Formerly Rusic"</div>
            <h1>"Your music."<br/>"Your way."</h1>
            <p>
                "Kopuz is a modern, lightweight music player built with Rust and Dioxus. "
                "Scan local folders, stream from Jellyfin or Navidrome, and browse your library the way you want."
            </p>
            <div class="hero-ctas">
                <a href="#downloads" class="btn-primary">"Download"</a>
                <a href="https://github.com/Kopuz-org/kopuz" target="_blank" class="btn-secondary">"View on GitHub"</a>
            </div>
            <div class="hero-stats">
                <div class="stat">
                    <span class="stat-num">"Rust"</span>
                    <span class="stat-label">"Built with"</span>
                </div>
                <div class="stat-sep"></div>
                <div class="stat">
                    <span class="stat-num">"MIT"</span>
                    <span class="stat-label">"License"</span>
                </div>
                <div class="stat-sep"></div>
                <div class="stat">
                    <span class="stat-num">"3 Platforms"</span>
                    <span class="stat-label">"Win / Mac / Linux"</span>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Features() -> impl IntoView {
    view! {
        <section class="features" id="features">
            <div class="section-header">
                <h2>"Everything you need."</h2>
                <p>"No subscriptions. No tracking. Just your music."</p>
            </div>
            <div class="features-grid">
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
                    <p>"UI available in English and Russian. More languages easy to add via Fluent."</p>
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
                    <h3>"macOS"</h3>
                    <p>"Download the " <code>".dmg"</code> " from GitHub Releases. If macOS blocks it, clear the quarantine flag:"</p>
                    <pre><code>"xattr -d com.apple.quarantine /Applications/Kopuz.app"</code></pre>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Downloads() -> impl IntoView {
    view! {
        <section class="downloads" id="downloads">
            <div class="section-header">
                <h2>"Download Kopuz"</h2>
                <p>"Free and open source. All releases on GitHub."</p>
            </div>
            <div class="download-grid">
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="download-card">
                    <i class="dl-icon fa-brands fa-windows"></i>
                    <span class="dl-name">"Windows"</span>
                    <span class="dl-ext">".exe / .msi"</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="download-card">
                    <i class="dl-icon fa-brands fa-apple"></i>
                    <span class="dl-name">"macOS"</span>
                    <span class="dl-ext">".dmg"</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="download-card">
                    <i class="dl-icon fa-brands fa-linux"></i>
                    <span class="dl-name">"Linux"</span>
                    <span class="dl-ext">".AppImage / .deb"</span>
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
