## Language
lang-name = Español

## Nav
nav-new = Nuevo
nav-announce = UI moderna, tema Vaxry, descargador yt-dlp, 20+ idiomas
nav-features = Funciones
nav-install = Instalar
nav-download = Descargar
nav-sponsors = Patrocinadores
nav-github = GitHub ↗
nav-gallery = Galería
nav-lang-label = Idioma

## Hero
hero-title-1 = Tu música.
hero-title-2 = A tu manera.
hero-desc = Kopuz es un reproductor de música moderno y ligero construido con Rust y Dioxus. Escanea carpetas locales, transmite desde Jellyfin o Navidrome, y navega tu biblioteca como quieras.
hero-cta-download = Descargar
hero-cta-github = Ver en GitHub
hero-screenshot-alt = Kopuz — vista inicio

## Features
features-title = Todo lo que necesitas.
features-chip = Sin suscripciones. Sin rastreo. Solo tu música.
features-works-with = Funciona con
features-source-local = Archivos locales
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = API Subsonic

feat-local-title = Local + Streaming
feat-local-desc = Apunta a una carpeta local o conéctate a Jellyfin / Subsonic (Navidrome). Mezcla como quieras.
feat-theming-title = Temas
feat-theming-desc = Temas dinámicos con control total de variables de color. Crea tu propio tema desde cero o elige un preset.
feat-native-title = Integración nativa
feat-native-desc = MPRIS en Linux, Now Playing en macOS, System Media Transport en Windows. Todo cableado.
feat-lyrics-title = Letras sincronizadas
feat-lyrics-desc = Letras en tiempo real con desplazamiento, sincronizadas o no, auto-scroll para seguir la música.
feat-eq-title = Ecualizador
feat-eq-desc = Ecualizador de 5 bandas con presets integrados y control total sobre tu sonido.
feat-fav-title = Favoritos y listas
feat-fav-desc = Marca canciones localmente o sincroniza favoritos con tu servidor. Crea listas, añade álbumes enteros de una vez.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobbling ListenBrainz integrado. Los usuarios de Jellyfin también pueden usar el plugin listenbrainz.
feat-discord-title = Discord RPC
feat-discord-desc = Muestra a tus amigos qué estás escuchando con Discord Rich Presence incrustado. Sin configuración.
feat-search-title = Búsqueda
feat-search-desc = Búsqueda en tiempo real en artistas, álbumes y pistas. Resultados instantáneos al escribir.
feat-genre-title = Navegación por género
feat-genre-desc = Navega toda tu biblioteca por género — funciona para archivos locales y música de servidor.
feat-logs-title = Registros de escucha
feat-logs-desc = Conteo de reproducciones local. Mira qué escuchas más con el tiempo.
feat-i18n-title = i18n
feat-i18n-desc = Inglés, ruso, alemán, francés, español, turco, ucraniano, polaco, árabe, griego, hebreo, húngaro, indonesio, japonés, coreano, rumano, portugués, chino y más.
feat-ytdlp-title = Descargador yt-dlp
feat-ytdlp-desc = Descarga audio directamente desde YouTube. Salida como MP3, FLAC, WAV, MP4 o mejor calidad. SponsorBlock, división por capítulos y límite de velocidad incluidos.
feat-crossfade-title = Crossfade y transiciones
feat-crossfade-desc = Mezcla transiciones de pistas para reproducción más suave. Soporte crossfade en builds desktop nativos.
feat-channels-title = Modos de canal
feat-channels-desc = Estéreo, Mono, Solo-Izquierda, Solo-Derecha, e intercambio L/R. Control fino de canales de audio.

## Performance
perf-title = Construido para ser rápido.
perf-subtitle = Bibliotecas grandes. Inicio instantáneo. Sin congelaciones.
perf-skip-label = Omitir ya indexados
perf-skip-desc = Los reescaneos solo procesan archivos nuevos. 10k pistas + 5 nuevas = solo 5 leídas.
perf-parallel-label = Inicio paralelo
perf-parallel-desc-1 = Biblioteca, config, listas y favoritos cargan todos en paralelo con
perf-parallel-desc-2 = . Apertura casi instantánea.
perf-art-label = Caché de carátulas
perf-art-desc = Carátulas extraídas una vez, guardadas en disco. Nunca redecodificadas en vistas repetidas.
perf-lazy-label = Carga perezosa de imágenes
perf-lazy-desc = Cientos de carátulas en resultados de búsqueda — ninguna carga hasta ser visible.
perf-io-label = I/O no bloqueante
perf-io-desc-1 = El trabajo pesado corre en hilos
perf-io-desc-2 = { " " }. La UI permanece responsiva durante escaneos completos.
perf-http-label = Caché HTTP de carátulas
perf-http-desc-1 = El protocolo
perf-http-desc-2 = { " " }personalizado sirve carátulas con headers de caché de 1 año. Webview nunca vuelve a buscarlas.

## Install
install-title = Instalación
install-nix-title = Nix / NixOS
install-nix-run = Ejecutar sin instalar:
install-nix-profile = O añadir a tu perfil:
install-nix-note = Flake NixOS soportado con caché binario Cachix.
install-flatpak-title = Flatpak
install-flatpak-desc = Instalar desde manifiesto fuente:
install-flatpak-note = Listado en Flathub próximamente.
install-appimage-title = AppImage
install-appimage-desc-1 = Descarga desde GitHub Releases y ejecuta directamente. Requiere
install-appimage-desc-2 = { " " }y
install-appimage-desc-3 = { " " }en tu sistema.
install-appimage-note-1 = Usuarios Arch: si crashea con error WebKitNetworkProcess, ejecuta con prefijo
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Descarga el
install-macos-desc-2 = { " " }desde GitHub Releases. Si macOS lo bloquea, quita la bandera de cuarentena:

## Platforms
platforms-title = Descargar Kopuz
platforms-subtitle = Gratis y open source. Todas las versiones en GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Solo Apple Silicon
platforms-linux = Linux
platforms-download = Descargar →

## Support
support-title = Apoyar Kopuz
support-subtitle = Kopuz es gratis y open source. El apoyo lo mantiene vivo.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — o envía cripto —
support-usdt-note = (cadena Solana)

## Sponsors
sponsors-title = Patrocinadores
sponsors-subtitle = Gente que mantiene Kopuz andando.
sponsors-cta = Hazte patrocinador

## Gallery
gallery-page-title = Galería — Kopuz Music Player
gallery-page-desc = Capturas de Kopuz en acción — inicio, biblioteca, lista, reproductor pantalla completa, letras, editor de temas y más.
gallery-title = Galería
gallery-subtitle = Capturas de Kopuz — estilos Normal y Modern.
gallery-home = Inicio
gallery-home-styles = Normal / Modern
gallery-library = Biblioteca
gallery-library-styles = Normal / Vaxry
gallery-playlist = Lista
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Reproductor pantalla completa
gallery-fullscreen-desc = Vista inmersiva de reproducción pantalla completa.
gallery-fullscreen-lyrics-title = Letras pantalla completa
gallery-fullscreen-lyrics-desc = Letras sincronizadas en modo pantalla completa.
gallery-search-title = Búsqueda
gallery-search-desc = Búsqueda en tiempo real en artistas, álbumes y pistas.
gallery-theme-title = Editor de temas
gallery-theme-desc = Control total de variables de color. Construye o elige un preset.
gallery-settings-title = Ajustes del reproductor
gallery-settings-desc = Configura audio, comportamiento, integraciones y más.
gallery-downloader-title = Descargador
gallery-downloader-desc = Descarga pistas directamente desde Kopuz.
gallery-prev = ← Atrás
gallery-next = Siguiente →
gallery-label-normal-home = Normal — Inicio
gallery-label-modern-home = Modern — Inicio
gallery-label-normal-library = Normal — Biblioteca
gallery-label-vaxry-library = Vaxry — Biblioteca
gallery-label-normal-playlist = Normal — Lista
gallery-label-modern-playlist = Modern — Lista
gallery-label-fullscreen = Reproductor pantalla completa
gallery-label-fullscreen-lyrics = Letras pantalla completa
gallery-label-search = Búsqueda
gallery-label-theme-editor = Editor de temas
gallery-label-player-settings = Ajustes del reproductor
gallery-label-downloader = Descargador

## Footer
footer-license = Licencia MIT — Gratis y Open Source
footer-github = GitHub
footer-releases = Versiones
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz es un reproductor de música moderno y ligero construido con Rust y Dioxus. Transmite desde Jellyfin o Navidrome, navega archivos locales, letras sincronizadas, ecualizador, temas y más.
home-meta-keywords = Kopuz, reproductor música, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, letras
og-title = Kopuz — Music Player
og-desc = Reproductor de música moderno y ligero construido con Rust. Archivos locales, Jellyfin, Navidrome, letras sincronizadas, ecualizador, Discord RPC y más. Gratis y open source.
og-image-alt = Reproductor de música Kopuz
twitter-title = Kopuz — Music Player
twitter-desc = Reproductor de música moderno y ligero construido con Rust. Gratis y open source.
