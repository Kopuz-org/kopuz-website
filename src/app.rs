use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr, I18n};
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

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

    leptos_fluent! {
        translations: [TRANSLATIONS],
        languages: "./locales/languages.json",
        locales: "./locales",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lf-lang",
        cookie_attrs: "SameSite=Strict; Path=/",
        initial_language_from_cookie: true,
        set_language_to_cookie: true,
        initial_language_from_url_param: true,
        url_param: "lang",
        initial_language_from_accept_language_header: true,
        initial_language_from_navigator: true,
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/kopuz-website.css"/>
        <Title text=move_tr!("home-title")/>
        <Meta name="description" content=move_tr!("home-meta-desc")/>
        <Meta name="keywords" content=move_tr!("home-meta-keywords")/>
        <Meta name="author" content="temidaradev"/>
        <Meta name="robots" content="index, follow"/>
        <Meta name="theme-color" content="#32302f"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content=move_tr!("og-title")/>
        <Meta property="og:description" content=move_tr!("og-desc")/>
        <Meta property="og:url" content="https://kopuz.temidara.rocks"/>
        <Meta property="og:image" content="https://kopuz.temidara.rocks/banner.png"/>
        <Meta property="og:image:alt" content=move_tr!("og-image-alt")/>
        <Meta property="og:site_name" content="Kopuz"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=move_tr!("twitter-title")/>
        <Meta name="twitter:description" content=move_tr!("twitter-desc")/>
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
            <YtMusic/>
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
        <Title text=move_tr!("gallery-page-title")/>
        <Meta name="description" content=move_tr!("gallery-page-desc")/>
        <Meta property="og:title" content=move_tr!("gallery-page-title")/>
        <Meta property="og:description" content=move_tr!("gallery-page-desc")/>
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
fn LanguageSwitcher() -> impl IntoView {
    let i18n = expect_context::<I18n>();
    let on_change = move |ev: leptos::ev::Event| {
        let v = event_target_value(&ev);
        if let Some(lang) = i18n.languages.iter().find(|l| l.id.to_string() == v) {
            i18n.language.set(lang);
        }
    };

    view! {
        <select
            class="nav-lang"
            aria-label=move_tr!("nav-lang-label")
            on:change=on_change
        >
            {i18n.languages.iter().map(|lang| {
                let lang_id = lang.id.to_string();
                let lang_id_cmp = lang_id.clone();
                let name = lang.name;
                view! {
                    <option
                        value=lang_id
                        selected=move || i18n.language.get().id.to_string() == lang_id_cmp
                    >
                        {name}
                    </option>
                }
            }).collect_view()}
        </select>
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
                    <span class="nav-new">{move_tr!("nav-new")}</span>
                    " "
                    {move_tr!("nav-announce")}
                </a>
                <div class="nav-tabs">
                    <a href="/#features" class="nav-tab">{move_tr!("nav-features")}</a>
                    <a href="/#install" class="nav-tab">{move_tr!("nav-install")}</a>
                    <a href="/#downloads" class="nav-tab">{move_tr!("nav-download")}</a>
                    <a href="/#sponsors" class="nav-tab">{move_tr!("nav-sponsors")}</a>
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" class="nav-tab">{move_tr!("nav-github")}</a>
                    <a href="/gallery" class=move || if is_gallery { "nav-tab nav-tab-active" } else { "nav-tab" }>{move_tr!("nav-gallery")}</a>
                    <LanguageSwitcher/>
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
                <h1>{move_tr!("hero-title-1")}<br/>{move_tr!("hero-title-2")}</h1>
                <p>{move_tr!("hero-desc")}</p>
                <div class="hero-ctas">
                    <a href="#downloads" class="btn-primary">{move_tr!("hero-cta-download")}</a>
                    <a href="https://github.com/Kopuz-org/kopuz" target="_blank" class="btn-secondary">{move_tr!("hero-cta-github")}</a>
                </div>
            </div>
            <div class="hero-right">
                <img src="/normal-home.png" alt=move_tr!("hero-screenshot-alt") class="hero-screenshot"/>
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
                    <h2>{move_tr!("features-title")}</h2>
                    <span class="features-chip">{move_tr!("features-chip")}</span>
                </div>
                <div class="features-sources-bar">
                    <span class="sources-label">{move_tr!("features-works-with")}</span>
                    <div class="sources-list">
                        <span class="source-tag"><i class="fa-solid fa-folder-open"></i>" "{move_tr!("features-source-local")}</span>
                        <span class="source-tag"><i class="fa-solid fa-server"></i>" "{move_tr!("features-source-jellyfin")}</span>
                        <span class="source-tag"><i class="fa-solid fa-server"></i>" "{move_tr!("features-source-navidrome")}</span>
                        <span class="source-tag"><i class="fa-solid fa-satellite-dish"></i>" "{move_tr!("features-source-subsonic")}</span>
                        <span class="source-tag"><i class="fa-brands fa-youtube"></i>" "{move_tr!("features-source-ytmusic")}</span>
                    </div>
                </div>
                <FeatureCard icon="fa-solid fa-music" title_key="feat-local-title" desc_key="feat-local-desc"/>
                <FeatureCard icon="fa-brands fa-youtube" title_key="feat-youtube-title" desc_key="feat-youtube-desc"/>
                <FeatureCard icon="fa-solid fa-palette" title_key="feat-theming-title" desc_key="feat-theming-desc"/>
                <FeatureCard icon="fa-solid fa-display" title_key="feat-native-title" desc_key="feat-native-desc"/>
                <FeatureCard icon="fa-solid fa-align-left" title_key="feat-lyrics-title" desc_key="feat-lyrics-desc"/>
                <FeatureCard icon="fa-solid fa-sliders" title_key="feat-eq-title" desc_key="feat-eq-desc"/>
                <FeatureCard icon="fa-solid fa-star" title_key="feat-fav-title" desc_key="feat-fav-desc"/>
                <FeatureCard icon="fa-solid fa-tower-broadcast" title_key="feat-scrobble-title" desc_key="feat-scrobble-desc"/>
                <FeatureCard icon="fa-brands fa-discord" title_key="feat-discord-title" desc_key="feat-discord-desc"/>
                <FeatureCard icon="fa-solid fa-magnifying-glass" title_key="feat-search-title" desc_key="feat-search-desc"/>
                <FeatureCard icon="fa-solid fa-tags" title_key="feat-genre-title" desc_key="feat-genre-desc"/>
                <FeatureCard icon="fa-solid fa-clock" title_key="feat-logs-title" desc_key="feat-logs-desc"/>
                <FeatureCard icon="fa-solid fa-globe" title_key="feat-i18n-title" desc_key="feat-i18n-desc"/>
                <FeatureCard icon="fa-brands fa-youtube" title_key="feat-ytdlp-title" desc_key="feat-ytdlp-desc"/>
                <FeatureCard icon="fa-solid fa-shuffle" title_key="feat-crossfade-title" desc_key="feat-crossfade-desc"/>
                <FeatureCard icon="fa-solid fa-headphones" title_key="feat-channels-title" desc_key="feat-channels-desc"/>
                <FeatureCard icon="fa-solid fa-image" title_key="feat-metadata-title" desc_key="feat-metadata-desc"/>
                <FeatureCard icon="fa-solid fa-file-lines" title_key="feat-debug-title" desc_key="feat-debug-desc"/>
                <FeatureCard icon="fa-solid fa-broom" title_key="feat-cleanup-title" desc_key="feat-cleanup-desc"/>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    #[prop(into)] icon: String,
    #[prop(into)] title_key: &'static str,
    #[prop(into)] desc_key: &'static str,
) -> impl IntoView {
    let title = feature_title(title_key);
    let desc = feature_desc(desc_key);
    view! {
        <div class="feature-card">
            <i class=format!("feature-icon {icon}")></i>
            <h3>{title}</h3>
            <p>{desc}</p>
        </div>
    }
}

fn feature_title(key: &'static str) -> Signal<String> {
    match key {
        "feat-local-title" => move_tr!("feat-local-title"),
        "feat-theming-title" => move_tr!("feat-theming-title"),
        "feat-native-title" => move_tr!("feat-native-title"),
        "feat-lyrics-title" => move_tr!("feat-lyrics-title"),
        "feat-eq-title" => move_tr!("feat-eq-title"),
        "feat-fav-title" => move_tr!("feat-fav-title"),
        "feat-scrobble-title" => move_tr!("feat-scrobble-title"),
        "feat-discord-title" => move_tr!("feat-discord-title"),
        "feat-search-title" => move_tr!("feat-search-title"),
        "feat-genre-title" => move_tr!("feat-genre-title"),
        "feat-logs-title" => move_tr!("feat-logs-title"),
        "feat-i18n-title" => move_tr!("feat-i18n-title"),
        "feat-ytdlp-title" => move_tr!("feat-ytdlp-title"),
        "feat-crossfade-title" => move_tr!("feat-crossfade-title"),
        "feat-channels-title" => move_tr!("feat-channels-title"),
        "feat-youtube-title" => move_tr!("feat-youtube-title"),
        "feat-metadata-title" => move_tr!("feat-metadata-title"),
        "feat-debug-title" => move_tr!("feat-debug-title"),
        "feat-cleanup-title" => move_tr!("feat-cleanup-title"),
        _ => Signal::derive(|| String::new()),
    }
}

fn feature_desc(key: &'static str) -> Signal<String> {
    match key {
        "feat-local-desc" => move_tr!("feat-local-desc"),
        "feat-theming-desc" => move_tr!("feat-theming-desc"),
        "feat-native-desc" => move_tr!("feat-native-desc"),
        "feat-lyrics-desc" => move_tr!("feat-lyrics-desc"),
        "feat-eq-desc" => move_tr!("feat-eq-desc"),
        "feat-fav-desc" => move_tr!("feat-fav-desc"),
        "feat-scrobble-desc" => move_tr!("feat-scrobble-desc"),
        "feat-discord-desc" => move_tr!("feat-discord-desc"),
        "feat-search-desc" => move_tr!("feat-search-desc"),
        "feat-genre-desc" => move_tr!("feat-genre-desc"),
        "feat-logs-desc" => move_tr!("feat-logs-desc"),
        "feat-i18n-desc" => move_tr!("feat-i18n-desc"),
        "feat-ytdlp-desc" => move_tr!("feat-ytdlp-desc"),
        "feat-crossfade-desc" => move_tr!("feat-crossfade-desc"),
        "feat-channels-desc" => move_tr!("feat-channels-desc"),
        "feat-youtube-desc" => move_tr!("feat-youtube-desc"),
        "feat-metadata-desc" => move_tr!("feat-metadata-desc"),
        "feat-debug-desc" => move_tr!("feat-debug-desc"),
        "feat-cleanup-desc" => move_tr!("feat-cleanup-desc"),
        _ => Signal::derive(|| String::new()),
    }
}

#[component]
fn Performance() -> impl IntoView {
    view! {
        <section class="perf" id="performance">
            <div class="section-header">
                <h2>{move_tr!("perf-title")}</h2>
                <p>{move_tr!("perf-subtitle")}</p>
            </div>
            <div class="perf-grid">
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-skip-label")}</span>
                    <p>{move_tr!("perf-skip-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-parallel-label")}</span>
                    <p>{move_tr!("perf-parallel-desc-1")}" "<code>"tokio::join!"</code>{move_tr!("perf-parallel-desc-2")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-art-label")}</span>
                    <p>{move_tr!("perf-art-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-lazy-label")}</span>
                    <p>{move_tr!("perf-lazy-desc")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-io-label")}</span>
                    <p>{move_tr!("perf-io-desc-1")}" "<code>"spawn_blocking"</code>{move_tr!("perf-io-desc-2")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-http-label")}</span>
                    <p>{move_tr!("perf-http-desc-1")}" "<code>"artwork://"</code>{move_tr!("perf-http-desc-2")}</p>
                </div>
                <div class="perf-item">
                    <span class="perf-label">{move_tr!("perf-sort-label")}</span>
                    <p>{move_tr!("perf-sort-desc-1")}" "<code>"sort_by_cached_key"</code>{move_tr!("perf-sort-desc-2")}</p>
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
                <h2>{move_tr!("install-title")}</h2>
            </div>
            <div class="install-grid">
                <div class="install-card">
                    <h3>{move_tr!("install-nix-title")}</h3>
                    <p>{move_tr!("install-nix-run")}</p>
                    <pre><code>"nix run github:temidaradev/kopuz"</code></pre>
                    <p>{move_tr!("install-nix-profile")}</p>
                    <pre><code>"nix profile add github:temidaradev/kopuz"</code></pre>
                    <p class="install-note">{move_tr!("install-nix-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-aur-title")}</h3>
                    <p>{move_tr!("install-aur-desc")}</p>
                    <pre><code>"yay -S kopuz
# or
paru -S kopuz"</code></pre>
                    <p class="install-note">{move_tr!("install-aur-note-1")}" "<code>"dioxus-cli"</code>{move_tr!("install-aur-note-2")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-flatpak-title")}</h3>
                    <p>{move_tr!("install-flatpak-desc")}</p>
                    <pre><code>"git clone https://github.com/temidaradev/kopuz
cd kopuz
flatpak-builder --user --install --force-clean \\
  build-dir packaging/flatpak/com.temidaradev.kopuz.json
flatpak run com.temidaradev.kopuz"</code></pre>
                    <p class="install-note">{move_tr!("install-flatpak-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-appimage-title")}</h3>
                    <p>{move_tr!("install-appimage-desc-1")}" "<code>"webkit2gtk-4.1"</code>{move_tr!("install-appimage-desc-2")}" "<code>"gtk3"</code>{move_tr!("install-appimage-desc-3")}</p>
                    <p class="install-note">{move_tr!("install-appimage-note-1")}" "<code>"LD_LIBRARY_PATH=/usr/lib"</code>{move_tr!("install-appimage-note-2")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("install-macos-title")}" "<span class="install-chip">{move_tr!("install-macos-chip")}</span></h3>
                    <p>{move_tr!("install-macos-desc-1")}" "<code>".dmg"</code>{move_tr!("install-macos-desc-2")}</p>
                    <pre><code>"xattr -d com.apple.quarantine /Applications/Kopuz.app"</code></pre>
                </div>
            </div>
        </section>
    }
}

#[component]
fn YtMusic() -> impl IntoView {
    view! {
        <section class="install" id="ytmusic">
            <div class="section-header">
                <h2>{move_tr!("ytmusic-title")}</h2>
                <p>{move_tr!("ytmusic-subtitle")}</p>
            </div>
            <div class="install-grid">
                <div class="install-card">
                    <h3>{move_tr!("ytmusic-prereq-title")}</h3>
                    <p>{move_tr!("ytmusic-prereq-desc-1")}" "<code>"rustypipe-botguard"</code>{move_tr!("ytmusic-prereq-desc-2")}</p>
                    <pre><code>"cargo install rustypipe-botguard --version 0.1.2"</code></pre>
                    <p class="install-note">{move_tr!("ytmusic-prereq-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("ytmusic-signin-title")}</h3>
                    <p>{move_tr!("ytmusic-signin-desc")}</p>
                    <p class="install-note">{move_tr!("ytmusic-signin-note")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("ytmusic-anon-title")}</h3>
                    <p>{move_tr!("ytmusic-anon-desc")}</p>
                </div>
                <div class="install-card">
                    <h3>{move_tr!("ytmusic-premium-title")}</h3>
                    <p>{move_tr!("ytmusic-premium-desc-1")}" "<code>"yt-dlp"</code>{move_tr!("ytmusic-premium-desc-2")}</p>
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
                <h2>{move_tr!("platforms-title")}</h2>
                <p>{move_tr!("platforms-subtitle")}</p>
            </div>
            <div class="platform-grid">
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-windows platform-os-icon"></i>
                        <span class="platform-name">{move_tr!("platforms-windows")}</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".exe"</span>
                    </div>
                    <span class="platform-dl">{move_tr!("platforms-download")}</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-apple platform-os-icon"></i>
                        <span class="platform-name">{move_tr!("platforms-macos")}</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".dmg"</span>
                    </div>
                    <span class="platform-note">{move_tr!("platforms-macos-note")}</span>
                    <span class="platform-dl">{move_tr!("platforms-download")}</span>
                </a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank" class="platform-card">
                    <div class="platform-header">
                        <i class="fa-brands fa-linux platform-os-icon"></i>
                        <span class="platform-name">{move_tr!("platforms-linux")}</span>
                    </div>
                    <div class="platform-formats">
                        <span class="platform-fmt">".AppImage"</span>
                        <span class="platform-fmt">".deb"</span>
                        <span class="platform-fmt">".rpm"</span>
                        <span class="platform-fmt">"Flatpak"</span>
                        <span class="platform-fmt">"Nix"</span>
                    </div>
                    <span class="platform-dl">{move_tr!("platforms-download")}</span>
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
                <h2>{move_tr!("support-title")}</h2>
                <p>{move_tr!("support-subtitle")}</p>
            </div>
            <div class="support-links">
                <a href="https://github.com/sponsors/temidaradev" target="_blank" class="support-btn support-gh">
                    <i class="fa-solid fa-heart"></i>
                    {move_tr!("support-gh")}
                </a>
                <a href="https://buymeacoffee.com/temidaradev" target="_blank" class="support-btn support-bmc">
                    <i class="fa-solid fa-mug-hot"></i>
                    {move_tr!("support-bmc")}
                </a>
            </div>
            <div class="donate-divider">{move_tr!("support-crypto-divider")}</div>
            <div class="donate-grid">
                <div class="donate-item">
                    <span class="donate-coin">"SOL"</span>
                    <code>"2fapJYRztnTRLpJbmyEUnsuZ36AzLK2JrMmmLEfDqKpN"</code>
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
                    <span class="donate-note">{move_tr!("support-usdt-note")}</span>
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
                <h2>{move_tr!("sponsors-title")}</h2>
                <p>{move_tr!("sponsors-subtitle")}</p>
            </div>
            <div class="sponsors-grid">
                <a href="https://github.com/shytzedaka" target="_blank" class="sponsor-card">
                    <img src="https://github.com/shytzedaka.png?size=80" alt="shytzedaka"/>
                    <span>"shytzedaka"</span>
                </a>
                <a href="https://github.com/m110" target="_blank" class="sponsor-card">
                    <img src="https://github.com/m110.png?size=80" alt="m110"/>
                    <span>"m110"</span>
                </a>
                <a href="https://github.com/bulakemun" target="_blank" class="sponsor-card">
                    <img src="https://github.com/bulakemun.png?size=80" alt="bulakemun"/>
                    <span>"bulakemun"</span>
                </a>
                <a href="https://github.com/Iamknownasfesal" target="_blank" class="sponsor-card">
                    <img src="https://github.com/Iamknownasfesal.png?size=80" alt="fesal"/>
                    <span>"fesal"</span>
                </a>
                <a href="https://github.com/arda2k3" target="_blank" class="sponsor-card">
                    <img src="https://github.com/arda2k3.png?size=80" alt="arda2k3"/>
                    <span>"arda2k3"</span>
                </a>
            </div>
            <div class="sponsors-cta">
                <a href="https://github.com/sponsors/temidaradev" target="_blank" class="btn-secondary">{move_tr!("sponsors-cta")}</a>
            </div>
        </section>
    }
}

static GALLERY_SRCS: &[&str] = &[
    "/normal-home.png",
    "/modern-home.png",
    "/normal-library.png",
    "/vaxry-library.png",
    "/normal-playlist.png",
    "/modern-playlist.png",
    "/fullscreen.png",
    "/fullscreen-lyrics.png",
    "/search.png",
    "/theme-editor.png",
    "/player-settings.png",
    "/downloader.png",
];

fn gallery_label(idx: usize) -> Signal<String> {
    match idx {
        0 => move_tr!("gallery-label-normal-home"),
        1 => move_tr!("gallery-label-modern-home"),
        2 => move_tr!("gallery-label-normal-library"),
        3 => move_tr!("gallery-label-vaxry-library"),
        4 => move_tr!("gallery-label-normal-playlist"),
        5 => move_tr!("gallery-label-modern-playlist"),
        6 => move_tr!("gallery-label-fullscreen"),
        7 => move_tr!("gallery-label-fullscreen-lyrics"),
        8 => move_tr!("gallery-label-search"),
        9 => move_tr!("gallery-label-theme-editor"),
        10 => move_tr!("gallery-label-player-settings"),
        11 => move_tr!("gallery-label-downloader"),
        _ => Signal::derive(|| String::new()),
    }
}

#[component]
fn GalleryContent() -> impl IntoView {
    let current: RwSignal<Option<usize>> = RwSignal::new(None);

    let go_prev = move || {
        current.update(|c| {
            if let Some(i) = c {
                *i = if *i == 0 { GALLERY_SRCS.len() - 1 } else { *i - 1 };
            }
        });
    };
    let go_next = move || {
        current.update(|c| {
            if let Some(i) = c {
                *i = (*i + 1) % GALLERY_SRCS.len();
            }
        });
    };

    view! {
        <section class="gallery-section">
            <div class="section-header">
                <h2>{move_tr!("gallery-title")}</h2>
                <p>{move_tr!("gallery-subtitle")}</p>
            </div>
            <div class="gallery-grid">
                <div class="gallery-item">
                    <div class="gallery-thumbs">
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(0))>
                            <img src="/normal-home.png" alt=gallery_label(0)/>
                        </div>
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(1))>
                            <img src="/modern-home.png" alt=gallery_label(1)/>
                        </div>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-home")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-home-styles")}</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumbs">
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(2))>
                            <img src="/normal-library.png" alt=gallery_label(2)/>
                        </div>
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(3))>
                            <img src="/vaxry-library.png" alt=gallery_label(3)/>
                        </div>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-library")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-library-styles")}</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumbs">
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(4))>
                            <img src="/normal-playlist.png" alt=gallery_label(4)/>
                        </div>
                        <div class="gallery-thumb" on:click=move |_| current.set(Some(5))>
                            <img src="/modern-playlist.png" alt=gallery_label(5)/>
                        </div>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-playlist")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-playlist-styles")}</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(6))>
                        <img src="/fullscreen.png" alt=gallery_label(6)/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-fullscreen-title")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-fullscreen-desc")}</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(7))>
                        <img src="/fullscreen-lyrics.png" alt=gallery_label(7)/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-fullscreen-lyrics-title")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-fullscreen-lyrics-desc")}</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(8))>
                        <img src="/search.png" alt=gallery_label(8)/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-search-title")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-search-desc")}</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(9))>
                        <img src="/theme-editor.png" alt=gallery_label(9)/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-theme-title")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-theme-desc")}</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(10))>
                        <img src="/player-settings.png" alt=gallery_label(10)/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-settings-title")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-settings-desc")}</span>
                    </div>
                </div>
                <div class="gallery-item">
                    <div class="gallery-thumb gallery-thumb-single" on:click=move |_| current.set(Some(11))>
                        <img src="/downloader.png" alt=gallery_label(11)/>
                    </div>
                    <div class="gallery-caption">
                        <span class="gallery-title">{move_tr!("gallery-downloader-title")}</span>
                        <span class="gallery-desc">{move_tr!("gallery-downloader-desc")}</span>
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
                            view! {
                                <span class="lightbox-label">{gallery_label(idx)}</span>
                            }
                        }}
                        <span class="lightbox-counter">
                            {move || {
                                let idx = current.get().unwrap_or(0);
                                format!("{} / {}", idx + 1, GALLERY_SRCS.len())
                            }}
                        </span>
                        <button class="lightbox-close" on:click=move |ev| { ev.stop_propagation(); current.set(None); }>"×"</button>
                    </div>
                    {move || {
                        let idx = current.get().unwrap_or(0);
                        let src = GALLERY_SRCS[idx];
                        view! { <img src=src alt=gallery_label(idx) class="lightbox-img"/> }
                    }}
                    <div class="lightbox-nav">
                        <button class="lightbox-btn" on:click=move |ev| { ev.stop_propagation(); go_prev(); }>{move_tr!("gallery-prev")}</button>
                        <button class="lightbox-btn" on:click=move |ev| { ev.stop_propagation(); go_next(); }>{move_tr!("gallery-next")}</button>
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
                <span>{move_tr!("footer-license")}</span>
            </div>
            <div class="footer-links">
                <a href="https://github.com/Kopuz-org/kopuz" target="_blank">{move_tr!("footer-github")}</a>
                <a href="https://github.com/Kopuz-org/kopuz/releases" target="_blank">{move_tr!("footer-releases")}</a>
                <a href="https://github.com/Kopuz-org/kopuz/issues" target="_blank">{move_tr!("footer-issues")}</a>
                <a href="https://discord.gg/K6Bmzw2E4M" target="_blank">{move_tr!("footer-discord")}</a>
            </div>
        </footer>
    }
}
