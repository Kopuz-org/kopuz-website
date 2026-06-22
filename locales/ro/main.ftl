## Language
lang-name = Română

## Nav
nav-new = Nou
nav-announce = Streaming YouTube Music, Discover, radio mix, 20+ limbi
nav-features = Funcții
nav-install = Instalează
nav-download = Descarcă
nav-sponsors = Sponsori
nav-github = GitHub ↗
nav-gallery = Galerie
nav-lang-label = Limbă

## Hero
hero-title-1 = Muzica ta.
hero-title-2 = În felul tău.
hero-desc = Kopuz este un player de muzică modern și ușor, construit cu Rust și Dioxus. Scanează foldere locale, transmite de la Jellyfin sau Navidrome și răsfoiește biblioteca cum vrei.
hero-cta-download = Descarcă
hero-cta-github = Vezi pe GitHub
hero-screenshot-alt = Kopuz — vizualizare acasă

## Features
features-title = Tot ce ai nevoie.
features-chip = Fără abonamente. Fără urmărire. Doar muzica ta.
features-works-with = Funcționează cu
features-source-local = Fișiere locale
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-ytmusic = YouTube Music
features-source-soundcloud = SoundCloud

feat-local-title = Local + Streaming
feat-local-desc = Indică un folder local sau conectează-te la Jellyfin / Subsonic (Navidrome). Combină după preferință.
feat-theming-title = Teme
feat-theming-desc = Teme dinamice cu control complet al variabilelor de culoare. Construiește propria temă de la zero sau alege un preset.
feat-native-title = Integrare nativă
feat-native-desc = MPRIS pe Linux, Now Playing pe macOS, System Media Transport pe Windows. Complet conectat.
feat-lyrics-title = Versuri sincronizate
feat-lyrics-desc = Versuri derulante în timp real, sincronizate sau simple, auto-derulare pentru a urma muzica.
feat-eq-title = Egalizator
feat-eq-desc = Egalizator cu 5 benzi cu presetări încorporate și control complet al sunetului.
feat-fav-title = Favorite și liste
feat-fav-desc = Marchează piese local sau sincronizează favoritele cu serverul. Creează liste, adaugă albume întregi dintr-o dată.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobbling ListenBrainz încorporat. Utilizatorii Jellyfin pot folosi și plugin-ul listenbrainz.
feat-discord-title = Discord RPC
feat-discord-desc = Arată prietenilor ce asculți cu Discord Rich Presence încorporat. Fără setare.
feat-search-title = Căutare
feat-search-desc = Căutare în timp real în artiști, albume și piese. Rezultate instant în timp ce scrii.
feat-genre-title = Răsfoire după gen
feat-genre-desc = Răsfoiește toată biblioteca după gen — funcționează pentru fișiere locale și muzică de pe server.
feat-logs-title = Jurnale de ascultare
feat-logs-desc = Contoare de redare urmărite local. Vezi ce asculți cu adevărat mai mult de-a lungul timpului.
feat-i18n-title = i18n
feat-i18n-desc = Engleză, rusă, germană, franceză, spaniolă, turcă, ucraineană, poloneză, arabă, greacă, ebraică, maghiară, indoneziană, japoneză, coreeană, română, portugheză, chineză și altele.
feat-ytdlp-title = Descărcător yt-dlp
feat-ytdlp-desc = Descarcă audio direct de pe YouTube. Ieșire MP3, FLAC, WAV, MP4 sau cea mai bună calitate. SponsorBlock, împărțire pe capitole și limitare de rată incluse.
feat-crossfade-title = Crossfade și tranziții
feat-crossfade-desc = Combină tranzițiile între piese pentru redare mai lină. Suport crossfade pe build-uri desktop native.
feat-channels-title = Moduri canal
feat-channels-desc = Stereo, Mono, Doar-Stânga, Doar-Dreapta și schimbare L/R. Control fin al canalelor audio.

feat-youtube-title = YouTube Music
feat-youtube-desc = Backend complet de streaming cu o pagină Discover în stil Spotify, profiluri bogate de artiști și radio mix. Conectează-te pentru biblioteca ta, Liked Music și liste — sau răsfoiește anonim.
feat-metadata-title = Imagini artiști
feat-metadata-desc = Alege cum sunt obținute imaginile artiștilor: prima copertă de album (implicit) sau fotografii reale ale artiștilor preluate de pe serverul tău Jellyfin sau Subsonic, cu revenire automată.
feat-debug-title = Jurnale și rapoarte de crash
feat-debug-desc = Deschide sau exportă jurnalele direct din Setări, cu un trace de performanță opțional pe care îl poți deschide în Speedscope sau Perfetto. Crash-urile scriu automat un raport.
feat-cleanup-title = Curățare automată
feat-cleanup-desc = Piesele lipsă sau șterse sunt eliminate automat din biblioteca ta la rescanare. Fără intrări fantomă.
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = Transmite de pe SoundCloud după o conectare unică prin browser. Căutare, redare progresivă MP3 și Go+ AAC/HLS, piesele tale Liked ca favorite și liste doar pentru citire.
feat-miniplayer-title = Mini-player
feat-miniplayer-desc = O suprapunere compactă a piesei curente pe care o poți comuta din bara de jos pentru o vizualizare mai mică.
feat-tray-title = Minimizare în tray
feat-tray-desc = Închide într-o pictogramă din tray-ul de sistem în loc să închizi aplicația, astfel încât redarea continuă în fundal. Comută în Setări.
feat-badges-title = Insigne tip de fișier
feat-badges-desc = Piesele locale afișează o mică insignă de format — MP3, FLAC, WAV și altele — chiar în rândul piesei.

## Performance
perf-title = Construit să fie rapid.
perf-subtitle = Biblioteci mari. Pornire instantanee. Fără înghețuri.
perf-skip-label = Sări peste cele deja indexate
perf-skip-desc = Rescanările procesează doar fișiere noi. 10k piese + 5 noi = doar 5 citite.
perf-parallel-label = Pornire paralelă
perf-parallel-desc-1 = Biblioteca, configurația, listele și favoritele se încarcă toate în paralel cu
perf-parallel-desc-2 = . Deschidere aproape instantanee.
perf-art-label = Cache coperți album
perf-art-desc = Coperțile sunt extrase o singură dată, salvate pe disc. Nu sunt niciodată decodificate din nou la vizionări repetate.
perf-lazy-label = Încărcare leneșă imagini
perf-lazy-desc = Sute de coperți de album în rezultatele căutării — niciuna nu se încarcă până nu este vizibilă.
perf-io-label = I/O non-blocant
perf-io-desc-1 = Lucrul greu rulează pe fire
perf-io-desc-2 = { " " }. UI rămâne receptiv în timpul scanărilor complete ale bibliotecii.
perf-http-label = Cache HTTP coperți
perf-http-desc-1 = Protocolul
perf-http-desc-2 = { " " }personalizat servește coperțile cu antete cache de 1 an. Webview nu re-cere niciodată.
perf-sort-label = Sortare mai inteligentă
perf-sort-desc-1 = Vizualizările bibliotecii sortează cu
perf-sort-desc-2 = { " " }astfel încât cheile de sortare sunt calculate o singură dată, nu la fiecare comparație.

## Install
install-title = Instalare
install-nix-title = Nix / NixOS
install-nix-run = Rulează fără instalare:
install-nix-profile = Sau adaugă la profil:
install-nix-note = NixOS flake suportat cu cache binar Cachix.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Instalează cu helper-ul preferat:
install-aur-note-1 = Necesită
install-aur-note-2 = { " " }instalat în prealabil la o versiune compatibilă cu dioxus 0.7.x.
install-flatpak-title = Flatpak
install-flatpak-desc = Instalează din manifest sursă:
install-flatpak-note = Listare Flathub în curând.
install-appimage-title = AppImage
install-appimage-desc-1 = Descarcă de pe GitHub Releases și rulează direct. Necesită
install-appimage-desc-2 = { " " }și
install-appimage-desc-3 = { " " }pe sistemul tău.
install-appimage-note-1 = Utilizatori Arch: dacă crash-uiește cu eroare WebKitNetworkProcess, rulează cu prefixul
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Descarcă
install-macos-desc-2 = { " " }de pe GitHub Releases. Dacă macOS blochează, șterge flag-ul de carantină:

## Platforms
platforms-title = Descarcă Kopuz
platforms-subtitle = Gratuit și open source. Toate versiunile pe GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Doar Apple Silicon
platforms-linux = Linux
platforms-download = Descarcă →

## Support
support-title = Susține Kopuz
support-subtitle = Kopuz e gratuit și open source. Sprijinul îl ține în viață.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — sau trimite cripto —
support-usdt-note = (rețeaua Solana)

donate-title = Donații cripto
donate-subtitle = Susține dezvoltarea trimițând cripto la aceste adrese.
donate-solana = Solana: 2fapJYRztnTRLpJbmyEUnsuZ36AzLK2JrMmmLEfDqKpN
donate-bitcoin = Bitcoin: bc1qz94yz9xvufa6hxlvjzaajgd2zyfu86arn68hu4
donate-monero = Monero: 86mz3HxTrKyYpuvx78m6pufbXdwAnoyoZBztz6HyYrnM1XP5YVrMy9jTVRY5vzgGtkizACLpFwHEdafKTMoj6y8mAVgvWMz
donate-ethereum = Ethereum: 0xa490D50470cdFf837B6663F7f6cBe50B157224e5
donate-usdt-sol = USDT (Solana): GYmnAcrA5MbF6cUxT2m5d5cwdfr14qSY9WFYRwXxaibW

## YouTube Music
ytmusic-title = Configurare YouTube Music
ytmusic-subtitle = Adaugă-l din Setări → Servere media → Adaugă → YouTube Music.
ytmusic-token-title = Niciun helper necesar
ytmusic-token-desc-1 = Redarea anonimă necesită un content PO token, pe care Kopuz îl generează acum în aplicație cu un WebView ascuns ce rulează BotGuard de la YouTube. Vechiul
ytmusic-token-desc-2 = { " " }subproces a dispărut — nimic de instalat, iar funcționează în Flatpak.
ytmusic-signin-title = Conectare cu un browser
ytmusic-signin-desc = Kopuz deschide conectarea Google într-un profil de browser izolat — navigarea ta obișnuită nu este niciodată atinsă — și extrage cookie-urile de sesiune. Deblochează biblioteca ta, Liked Music, listele și artiștii urmăriți.
ytmusic-signin-note = Pe Windows, conectarea prin browser este momentan dezactivată; utilizatorii Windows primesc automat modul anonim. Conectarea funcționează pe Linux și macOS.
ytmusic-anon-title = Mod anonim
ytmusic-anon-desc = Fără conectare, fără cookie-uri. Răsfoiește, caută, deschide pagini de artiști, albume și liste, pornește radio mix și redă piese publice. Aprecierea și vizualizările bibliotecii sunt dezactivate.
ytmusic-premium-title = Piese Premium
ytmusic-premium-desc-1 = Piesele blocate cu Music Premium revin la o rezolvare locală
ytmusic-premium-desc-2 = { " " }când calea principală returnează UNPLAYABLE, deci a-l avea instalat ajută. Modul anonim nu poate reda deloc conținut exclusiv Premium.

## SoundCloud
soundcloud-title = Configurare SoundCloud
soundcloud-subtitle = Adaugă-l din Setări → Servere media → Adaugă → SoundCloud.
soundcloud-signin-title = Conectare unică prin browser
soundcloud-signin-desc = Niciun URL sau parolă de tastat. Kopuz deschide soundcloud.com/signin într-un profil de browser izolat — navigarea ta obișnuită nu este niciodată atinsă — și preia oauth_token-ul sesiunii. Alege ce browser din familia Chromium să folosești (Chrome, Chromium, Brave, Edge sau Vivaldi).
soundcloud-features-title = Ce primești
soundcloud-features-desc = Căutare, redare de piese (MP3 progresiv plus fluxuri Go+ AAC/HLS), piesele tale Liked ca favorite, liste doar pentru citire și apreciere/dezapreciere. Eliminarea sursei curăță profilul său izolat.

## Sponsors
sponsors-title = Sponsori
sponsors-subtitle = Oameni care țin Kopuz în mișcare.
sponsors-cta = Devino sponsor

## Gallery
gallery-page-title = Galerie — Kopuz Music Player
gallery-page-desc = Capturi cu Kopuz în acțiune — acasă, bibliotecă, listă, player fullscreen, versuri, editor de teme și altele.
gallery-title = Galerie
gallery-subtitle = Capturi cu Kopuz — stilurile Normal și Modern.
gallery-home = Acasă
gallery-home-styles = Normal / Modern
gallery-library = Bibliotecă
gallery-library-styles = Normal / Vaxry
gallery-playlist = Listă
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Player fullscreen
gallery-fullscreen-desc = Vizualizare imersivă de redare pe fereastră întreagă.
gallery-fullscreen-lyrics-title = Versuri fullscreen
gallery-fullscreen-lyrics-desc = Versuri sincronizate în mod fereastră întreagă.
gallery-search-title = Căutare
gallery-search-desc = Căutare în timp real în artiști, albume și piese.
gallery-theme-title = Editor teme
gallery-theme-desc = Control complet al variabilelor de culoare. Construiește sau alege un preset.
gallery-settings-title = Setări player
gallery-settings-desc = Configurează audio, comportament, integrări și altele.
gallery-downloader-title = Descărcător
gallery-downloader-desc = Descarcă piese direct din Kopuz.
gallery-prev = ← Anterior
gallery-next = Următor →
gallery-label-normal-home = Normal — Acasă
gallery-label-modern-home = Modern — Acasă
gallery-label-normal-library = Normal — Bibliotecă
gallery-label-vaxry-library = Vaxry — Bibliotecă
gallery-label-normal-playlist = Normal — Listă
gallery-label-modern-playlist = Modern — Listă
gallery-label-fullscreen = Player fullscreen
gallery-label-fullscreen-lyrics = Versuri fullscreen
gallery-label-search = Căutare
gallery-label-theme-editor = Editor teme
gallery-label-player-settings = Setări player
gallery-label-downloader = Descărcător

## Footer
footer-license = Licență MIT — Gratuit & Open Source
footer-github = GitHub
footer-releases = Versiuni
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz este un player de muzică modern și ușor, construit cu Rust și Dioxus. Transmite de la Jellyfin sau Navidrome, răsfoiește fișiere locale, versuri sincronizate, egalizator, teme și altele.
home-meta-keywords = Kopuz, player muzică, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, versuri
og-title = Kopuz — Music Player
og-desc = Player de muzică modern și ușor, construit cu Rust. Fișiere locale, Jellyfin, Navidrome, versuri sincronizate, egalizator, Discord RPC și altele. Gratuit și open source.
og-image-alt = Player de muzică Kopuz
twitter-title = Kopuz — Music Player
twitter-desc = Player de muzică modern și ușor, construit cu Rust. Gratuit și open source.

## User-friendly aliases (synced with EN updates)
perf-parallel-desc = { perf-parallel-desc-1 } { perf-parallel-desc-2 }
perf-io-desc = { perf-io-desc-1 } { perf-io-desc-2 }
perf-http-desc = { perf-http-desc-1 } { perf-http-desc-2 }
perf-sort-desc = { perf-sort-desc-1 } { perf-sort-desc-2 }
ytmusic-token-desc = { ytmusic-token-desc-1 } { ytmusic-token-desc-2 }
ytmusic-premium-desc = { ytmusic-premium-desc-1 } { ytmusic-premium-desc-2 }
install-quick-title = Quick start (most users)
install-quick-desc = Download the latest release for your platform and open it.
install-quick-cta = Open Releases
install-quick-note = If you are not sure which option to choose, start here.
