## Language
lang-name = Polski

## Nav
nav-new = Nowe
nav-announce = Streaming YouTube Music, Discover, radio mix, 20+ języków
nav-features = Funkcje
nav-install = Zainstaluj
nav-download = Pobierz
nav-sponsors = Sponsorzy
nav-github = GitHub ↗
nav-gallery = Galeria
nav-lang-label = Język

## Hero
hero-title-1 = Twoja muzyka.
hero-title-2 = Po twojemu.
hero-desc = Kopuz to nowoczesny, lekki odtwarzacz muzyki zbudowany w Rust i Dioxus. Skanuj lokalne foldery, streamuj z Jellyfin lub Navidrome i przeglądaj bibliotekę jak chcesz.
hero-cta-download = Pobierz
hero-cta-github = Zobacz na GitHub
hero-screenshot-alt = Kopuz — widok główny

## Features
features-title = Wszystko czego potrzebujesz.
features-chip = Bez subskrypcji. Bez śledzenia. Tylko twoja muzyka.
features-works-with = Działa z
features-source-local = Pliki lokalne
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API

feat-local-title = Lokalnie + Streaming
feat-local-desc = Wskaż lokalny folder lub połącz z Jellyfin / Subsonic (Navidrome). Mieszaj jak chcesz.
feat-theming-title = Motywy
feat-theming-desc = Dynamiczne motywy z pełną kontrolą zmiennych kolorów. Zbuduj własny motyw od zera lub wybierz preset.
feat-native-title = Integracja natywna
feat-native-desc = MPRIS na Linux, Now Playing na macOS, System Media Transport na Windows. W pełni podpięte.
feat-lyrics-title = Zsynchronizowany tekst
feat-lyrics-desc = Przewijany tekst w czasie rzeczywistym, zsynchronizowany lub zwykły, automatyczne przewijanie za muzyką.
feat-eq-title = Equalizer
feat-eq-desc = 5-pasmowy equalizer z wbudowanymi presetami i pełną kontrolą nad dźwiękiem.
feat-fav-title = Ulubione i playlisty
feat-fav-desc = Oznaczaj utwory lokalnie lub synchronizuj ulubione z serwerem. Twórz playlisty, dodawaj całe albumy naraz.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Wbudowany scrobbling ListenBrainz. Użytkownicy Jellyfin mogą też używać wtyczki listenbrainz.
feat-discord-title = Discord RPC
feat-discord-desc = Pokaż znajomym czego słuchasz dzięki wbudowanemu Discord Rich Presence. Bez konfiguracji.
feat-search-title = Wyszukiwanie
feat-search-desc = Wyszukiwanie w czasie rzeczywistym po artystach, albumach i utworach. Natychmiastowe wyniki.
feat-genre-title = Przeglądanie po gatunkach
feat-genre-desc = Przeglądaj całą bibliotekę po gatunkach — działa dla plików lokalnych i muzyki serwerowej.
feat-logs-title = Logi odsłuchu
feat-logs-desc = Liczniki odtworzeń lokalnie. Sprawdź czego naprawdę słuchasz najczęściej.
feat-i18n-title = i18n
feat-i18n-desc = Angielski, rosyjski, niemiecki, francuski, hiszpański, turecki, ukraiński, polski, arabski, grecki, hebrajski, węgierski, indonezyjski, japoński, koreański, rumuński, portugalski, chiński i więcej.
feat-ytdlp-title = Pobieracz yt-dlp
feat-ytdlp-desc = Pobieraj audio bezpośrednio z YouTube. Wyjście jako MP3, FLAC, WAV, MP4 lub najlepsza jakość. SponsorBlock, dzielenie na rozdziały i ograniczanie prędkości.
feat-crossfade-title = Crossfade i przejścia
feat-crossfade-desc = Płynne przejścia między utworami. Wsparcie crossfade w natywnych buildach desktop.
feat-channels-title = Tryby kanałów
feat-channels-desc = Stereo, Mono, Tylko-Lewy, Tylko-Prawy i zamiana L/R. Precyzyjna kontrola kanałów audio.

## Performance
perf-title = Zbudowany dla szybkości.
perf-subtitle = Duże biblioteki. Natychmiastowy start. Zero zacięć.
perf-skip-label = Pomijanie już zindeksowanych
perf-skip-desc = Powtórne skany przetwarzają tylko nowe pliki. 10k utworów + 5 nowych = czytanych tylko 5.
perf-parallel-label = Równoległy start
perf-parallel-desc-1 = Biblioteka, konfig, playlisty i ulubione ładują się równolegle przez
perf-parallel-desc-2 = . Prawie natychmiastowe otwarcie.
perf-art-label = Cache okładek
perf-art-desc = Okładki wyciągane raz, zapisywane na dysk. Nigdy nie dekodowane ponownie.
perf-lazy-label = Leniwe ładowanie obrazów
perf-lazy-desc = Setki okładek w wynikach wyszukiwania — żadna nie ładuje się dopóki nie jest widoczna.
perf-io-label = Nieblokujący I/O
perf-io-desc-1 = Ciężka praca działa na wątkach
perf-io-desc-2 = { " " }. UI pozostaje responsywny podczas pełnych skanów biblioteki.
perf-http-label = Cache HTTP okładek
perf-http-desc-1 = Własny protokół
perf-http-desc-2 = { " " }serwuje okładki z rocznymi nagłówkami cache. Webview nigdy nie pobiera ponownie.

## Install
install-title = Instalacja
install-nix-title = Nix / NixOS
install-nix-run = Uruchom bez instalacji:
install-nix-profile = Lub dodaj do profilu:
install-nix-note = Flake NixOS wspierany z Cachix binary cache.
install-flatpak-title = Flatpak
install-flatpak-desc = Zainstaluj z manifestu źródłowego:
install-flatpak-note = Listing Flathub wkrótce.
install-appimage-title = AppImage
install-appimage-desc-1 = Pobierz z GitHub Releases i uruchom bezpośrednio. Wymaga
install-appimage-desc-2 = { " " }i
install-appimage-desc-3 = { " " }w systemie.
install-appimage-note-1 = Użytkownicy Arch: jeśli crashuje z błędem WebKitNetworkProcess, uruchom z prefiksem
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Pobierz
install-macos-desc-2 = { " " }z GitHub Releases. Jeśli macOS blokuje, usuń flagę kwarantanny:

## Platforms
platforms-title = Pobierz Kopuz
platforms-subtitle = Darmowy i open source. Wszystkie wydania na GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Tylko Apple Silicon
platforms-linux = Linux
platforms-download = Pobierz →

## Support
support-title = Wesprzyj Kopuz
support-subtitle = Kopuz jest darmowy i open source. Wsparcie trzyma go przy życiu.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — albo wyślij kryptowalutę —
support-usdt-note = (sieć Solana)

## Sponsors
sponsors-title = Sponsorzy
sponsors-subtitle = Ludzie, którzy utrzymują Kopuz przy życiu.
sponsors-cta = Zostań sponsorem

## Gallery
gallery-page-title = Galeria — Kopuz Music Player
gallery-page-desc = Zrzuty Kopuz w akcji — strona główna, biblioteka, playlista, pełnoekranowy odtwarzacz, tekst, edytor motywów i więcej.
gallery-title = Galeria
gallery-subtitle = Zrzuty Kopuz — style Normal i Modern.
gallery-home = Strona główna
gallery-home-styles = Normal / Modern
gallery-library = Biblioteka
gallery-library-styles = Normal / Vaxry
gallery-playlist = Playlista
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Pełnoekranowy odtwarzacz
gallery-fullscreen-desc = Immersyjny widok odtwarzania pełnego okna.
gallery-fullscreen-lyrics-title = Pełnoekranowy tekst
gallery-fullscreen-lyrics-desc = Zsynchronizowany tekst w trybie pełnego okna.
gallery-search-title = Wyszukiwanie
gallery-search-desc = Wyszukiwanie w czasie rzeczywistym po artystach, albumach i utworach.
gallery-theme-title = Edytor motywów
gallery-theme-desc = Pełna kontrola zmiennych kolorów. Zbuduj lub wybierz preset.
gallery-settings-title = Ustawienia odtwarzacza
gallery-settings-desc = Konfiguruj audio, zachowanie, integracje i więcej.
gallery-downloader-title = Pobieracz
gallery-downloader-desc = Pobieraj utwory bezpośrednio z Kopuz.
gallery-prev = ← Wstecz
gallery-next = Dalej →
gallery-label-normal-home = Normal — Strona główna
gallery-label-modern-home = Modern — Strona główna
gallery-label-normal-library = Normal — Biblioteka
gallery-label-vaxry-library = Vaxry — Biblioteka
gallery-label-normal-playlist = Normal — Playlista
gallery-label-modern-playlist = Modern — Playlista
gallery-label-fullscreen = Pełnoekranowy odtwarzacz
gallery-label-fullscreen-lyrics = Pełnoekranowy tekst
gallery-label-search = Wyszukiwanie
gallery-label-theme-editor = Edytor motywów
gallery-label-player-settings = Ustawienia odtwarzacza
gallery-label-downloader = Pobieracz

## Footer
footer-license = Licencja MIT — Darmowy i Open Source
footer-github = GitHub
footer-releases = Wydania
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz to nowoczesny, lekki odtwarzacz muzyki zbudowany w Rust i Dioxus. Streamuj z Jellyfin lub Navidrome, przeglądaj pliki lokalne, zsynchronizowany tekst, equalizer, motywy i więcej.
home-meta-keywords = Kopuz, odtwarzacz muzyki, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, tekst piosenek
og-title = Kopuz — Music Player
og-desc = Nowoczesny, lekki odtwarzacz muzyki zbudowany w Rust. Pliki lokalne, Jellyfin, Navidrome, zsynchronizowany tekst, equalizer, Discord RPC i więcej. Darmowy i open source.
og-image-alt = Odtwarzacz muzyki Kopuz
twitter-title = Kopuz — Music Player
twitter-desc = Nowoczesny, lekki odtwarzacz muzyki zbudowany w Rust. Darmowy i open source.
