## Language
lang-name = Français

## Nav
nav-new = Nouveau
nav-announce = Streaming YouTube Music, Discover, radio mix, 20+ langues
nav-features = Fonctionnalités
nav-install = Installer
nav-download = Télécharger
nav-sponsors = Sponsors
nav-github = GitHub ↗
nav-gallery = Galerie
nav-lang-label = Langue

## Hero
hero-title-1 = Ta musique.
hero-title-2 = À ta façon.
hero-desc = Kopuz est un lecteur de musique moderne et léger construit avec Rust et Dioxus. Scanne tes dossiers locaux, diffuse depuis Jellyfin ou Navidrome, et parcours ta bibliothèque comme tu veux.
hero-cta-download = Télécharger
hero-cta-github = Voir sur GitHub
hero-screenshot-alt = Kopuz — vue accueil

## Features
features-title = Tout ce qu'il te faut.
features-chip = Pas d'abonnement. Pas de pistage. Juste ta musique.
features-works-with = Fonctionne avec
features-source-local = Fichiers locaux
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = API Subsonic
features-source-ytmusic = YouTube Music
features-source-soundcloud = SoundCloud

feat-local-title = Local + Streaming
feat-local-desc = Pointe vers un dossier local ou connecte-toi à Jellyfin / Subsonic (Navidrome). Mélange comme tu veux.
feat-theming-title = Thèmes
feat-theming-desc = Thèmes dynamiques avec contrôle complet des variables de couleur. Construis ton propre thème ou choisis un preset.
feat-native-title = Intégration native
feat-native-desc = MPRIS sur Linux, Now Playing sur macOS, System Media Transport sur Windows. Tout est branché.
feat-lyrics-title = Paroles synchronisées
feat-lyrics-desc = Paroles défilantes en temps réel, synchronisées ou non, défilement auto pour suivre la musique.
feat-eq-title = Égaliseur
feat-eq-desc = Égaliseur 5 bandes avec presets intégrés et contrôle complet sur ton son.
feat-fav-title = Favoris & Playlists
feat-fav-desc = Marque des pistes localement ou synchronise les favoris avec ton serveur. Crée des playlists, ajoute des albums entiers d'un coup.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobbling ListenBrainz intégré. Les utilisateurs Jellyfin peuvent aussi utiliser le plugin listenbrainz.
feat-discord-title = Discord RPC
feat-discord-desc = Montre à tes amis ce que tu écoutes avec Discord Rich Presence intégré. Aucune configuration.
feat-search-title = Recherche
feat-search-desc = Recherche en temps réel sur artistes, albums et pistes. Résultats instantanés.
feat-genre-title = Navigation par genre
feat-genre-desc = Parcours toute ta bibliothèque par genre — fonctionne pour les fichiers locaux et la musique serveur.
feat-logs-title = Journaux d'écoute
feat-logs-desc = Compteurs de lecture suivis localement. Vois ce que tu écoutes vraiment le plus.
feat-i18n-title = i18n
feat-i18n-desc = Anglais, russe, allemand, français, espagnol, turc, ukrainien, polonais, arabe, grec, hébreu, hongrois, indonésien, japonais, coréen, roumain, portugais, chinois et plus.
feat-ytdlp-title = Téléchargeur yt-dlp
feat-ytdlp-desc = Télécharge l'audio directement depuis YouTube. Sortie en MP3, FLAC, WAV, MP4 ou meilleure qualité. SponsorBlock, découpage par chapitre et limitation de débit inclus.
feat-crossfade-title = Crossfade & transitions
feat-crossfade-desc = Mélange les transitions de pistes pour une lecture plus fluide. Support crossfade sur builds desktop natifs.
feat-channels-title = Modes canaux
feat-channels-desc = Stéréo, Mono, Gauche-seul, Droite-seule, et échange L/R. Contrôle fin des canaux audio.
feat-youtube-title = YouTube Music
feat-youtube-desc = Backend de streaming complet avec une page Discover façon Spotify, des profils d'artistes riches et la radio mix. Connecte-toi pour ta bibliothèque, ta musique aimée et tes playlists — ou navigue en anonyme.
feat-metadata-title = Images d'artistes
feat-metadata-desc = Choisis la source des images d'artistes : première pochette d'album (par défaut) ou vraies photos d'artistes récupérées depuis ton serveur Jellyfin ou Subsonic, avec repli automatique.
feat-debug-title = Journaux & rapports de crash
feat-debug-desc = Ouvre ou exporte les journaux directement depuis les Paramètres, avec une trace de performance optionnelle que tu peux ouvrir dans Speedscope ou Perfetto. Les crashs écrivent un rapport automatiquement.
feat-cleanup-title = Nettoyage auto
feat-cleanup-desc = Les pistes manquantes ou supprimées sont retirées de ta bibliothèque automatiquement lors du rescan. Aucune entrée fantôme.
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = Streame depuis SoundCloud après une connexion navigateur unique. Recherche, lecture progressive MP3 et Go+ AAC/HLS, tes pistes aimées comme favoris, et playlists en lecture seule.
feat-miniplayer-title = Mini-lecteur
feat-miniplayer-desc = Un overlay de lecture compact que tu peux activer depuis la barre du bas pour une vue plus petite.
feat-tray-title = Réduire dans la barre des tâches
feat-tray-desc = Ferme vers une icône de barre système au lieu de quitter, pour que la lecture continue en arrière-plan. Active dans les Paramètres.
feat-badges-title = Badges de type de fichier
feat-badges-desc = Les pistes locales affichent un petit badge de format — MP3, FLAC, WAV, et plus — directement dans la ligne de la piste.

## Performance
perf-title = Conçu pour être rapide.
perf-subtitle = Grandes bibliothèques. Démarrage instantané. Aucun gel.
perf-skip-label = Ignorer déjà indexés
perf-skip-desc = Les rescans ne traitent que les nouveaux fichiers. 10k pistes + 5 nouvelles = seulement 5 lues.
perf-parallel-label = Démarrage parallèle
perf-parallel-desc-1 = Bibliothèque, config, playlists et favoris chargent tous en parallèle avec
perf-parallel-desc-2 = . Ouverture quasi-instantanée.
perf-art-label = Cache pochettes
perf-art-desc = Pochettes extraites une fois, sauvegardées sur disque. Jamais redécodées à la consultation.
perf-lazy-label = Chargement d'images paresseux
perf-lazy-desc = Des centaines de pochettes dans les résultats de recherche — aucune ne charge avant d'être visible.
perf-io-label = I/O non-bloquant
perf-io-desc-1 = Le travail lourd tourne sur des threads
perf-io-desc-2 = { " " }. L'UI reste réactive pendant les scans complets.
perf-http-label = Cache HTTP pochettes
perf-http-desc-1 = Le protocole
perf-http-desc-2 = { " " }personnalisé sert les pochettes avec des en-têtes cache de 1 an. Webview ne refait jamais la requête.
perf-sort-label = Tri plus intelligent
perf-sort-desc-1 = Les vues de bibliothèque trient avec
perf-sort-desc-2 = { " " }pour que les clés de tri soient calculées une fois, pas à chaque comparaison.

## Install
install-title = Installation
install-nix-title = Nix / NixOS
install-nix-run = Lancer sans installer :
install-nix-profile = Ou ajouter à ton profil :
install-nix-note = Flake NixOS supporté avec cache binaire Cachix.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Installe avec ton helper préféré :
install-aur-note-1 = Nécessite
install-aur-note-2 = { " " }installé au préalable dans une version correspondant à dioxus 0.7.x.
install-flatpak-title = Flatpak
install-flatpak-desc = Installer depuis le manifeste source :
install-flatpak-note = Listing Flathub à venir.
install-appimage-title = AppImage
install-appimage-desc-1 = Télécharge depuis GitHub Releases et lance directement. Nécessite
install-appimage-desc-2 = { " " }et
install-appimage-desc-3 = { " " }sur ton système.
install-appimage-note-1 = Utilisateurs Arch : si ça crashe avec une erreur WebKitNetworkProcess, lance avec le préfixe
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Télécharge le
install-macos-desc-2 = { " " }depuis GitHub Releases. Si macOS bloque, retire le flag quarantaine :

## Platforms
platforms-title = Télécharger Kopuz
platforms-subtitle = Gratuit et open source. Toutes les versions sur GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apple Silicon uniquement
platforms-linux = Linux
platforms-download = Télécharger →

## Support
support-title = Soutenir Kopuz
support-subtitle = Kopuz est gratuit et open source. Le soutien le maintient en vie.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — ou envoie de la crypto —
support-usdt-note = (chaîne Solana)

## Donate
donate-title = Dons en crypto
donate-subtitle = Soutiens le développement en envoyant de la crypto à ces adresses.
donate-solana = Solana: 2fapJYRztnTRLpJbmyEUnsuZ36AzLK2JrMmmLEfDqKpN
donate-bitcoin = Bitcoin: bc1qz94yz9xvufa6hxlvjzaajgd2zyfu86arn68hu4
donate-monero = Monero: 86mz3HxTrKyYpuvx78m6pufbXdwAnoyoZBztz6HyYrnM1XP5YVrMy9jTVRY5vzgGtkizACLpFwHEdafKTMoj6y8mAVgvWMz
donate-ethereum = Ethereum: 0xa490D50470cdFf837B6663F7f6cBe50B157224e5
donate-usdt-sol = USDT (Solana): GYmnAcrA5MbF6cUxT2m5d5cwdfr14qSY9WFYRwXxaibW

## YouTube Music
ytmusic-title = Configuration YouTube Music
ytmusic-subtitle = Ajoute-le depuis Paramètres → Serveurs média → Ajouter → YouTube Music.
ytmusic-token-title = Aucun helper requis
ytmusic-token-desc-1 = La lecture anonyme nécessite un PO token de contenu, que Kopuz génère désormais dans l'app avec une WebView cachée exécutant le BotGuard de YouTube. L'ancien
ytmusic-token-desc-2 = { " " }sous-processus a disparu — rien à installer, et ça marche dans Flatpak.
ytmusic-signin-title = Connexion via le navigateur
ytmusic-signin-desc = Kopuz ouvre la connexion Google dans un profil de navigateur isolé — ta navigation normale n'est jamais touchée — et extrait les cookies de session. Débloque ta bibliothèque, ta musique aimée, tes playlists et les artistes suivis.
ytmusic-signin-note = Sur Windows, la connexion par navigateur est actuellement désactivée ; les utilisateurs Windows passent automatiquement en mode anonyme. La connexion fonctionne sur Linux et macOS.
ytmusic-anon-title = Mode anonyme
ytmusic-anon-desc = Pas de connexion, pas de cookies. Navigue, recherche, ouvre les pages d'artistes, d'albums et de playlists, lance la radio mix, et joue les pistes publiques. Les favoris et les vues de bibliothèque sont désactivés.
ytmusic-premium-title = Pistes Premium
ytmusic-premium-desc-1 = Les pistes verrouillées par Music Premium se rabattent sur une résolution
ytmusic-premium-desc-2 = { " " }locale quand le chemin principal renvoie UNPLAYABLE, donc l'avoir installé aide. Le mode anonyme ne peut pas du tout lire le contenu réservé au Premium.

## SoundCloud
soundcloud-title = Configuration SoundCloud
soundcloud-subtitle = Ajoute-le depuis Paramètres → Serveurs média → Ajouter → SoundCloud.
soundcloud-signin-title = Connexion navigateur unique
soundcloud-signin-desc = Aucune URL ni mot de passe à taper. Kopuz ouvre soundcloud.com/signin dans un profil de navigateur isolé — ta navigation normale n'est jamais touchée — et récupère le oauth_token de la session. Choisis le navigateur de la famille Chromium à utiliser (Chrome, Chromium, Brave, Edge ou Vivaldi).
soundcloud-features-title = Ce que tu obtiens
soundcloud-features-desc = Recherche, lecture de pistes (MP3 progressif plus flux Go+ AAC/HLS), tes pistes aimées comme favoris, playlists en lecture seule, et like/unlike. Retirer la source nettoie son profil isolé.

## Sponsors
sponsors-title = Sponsors
sponsors-subtitle = Les gens qui maintiennent Kopuz en vie.
sponsors-cta = Devenir sponsor

## Gallery
gallery-page-title = Galerie — Kopuz Music Player
gallery-page-desc = Captures d'écran de Kopuz en action — accueil, bibliothèque, playlist, lecteur plein écran, paroles, éditeur de thème et plus.
gallery-title = Galerie
gallery-subtitle = Captures de Kopuz — styles Normal et Modern.
gallery-home = Accueil
gallery-home-styles = Normal / Modern
gallery-library = Bibliothèque
gallery-library-styles = Normal / Vaxry
gallery-playlist = Playlist
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Lecteur plein écran
gallery-fullscreen-desc = Vue de lecture immersive plein fenêtre.
gallery-fullscreen-lyrics-title = Paroles plein écran
gallery-fullscreen-lyrics-desc = Paroles synchronisées en mode plein fenêtre.
gallery-search-title = Recherche
gallery-search-desc = Recherche en temps réel sur artistes, albums et pistes.
gallery-theme-title = Éditeur de thème
gallery-theme-desc = Contrôle complet des variables de couleur. Construis ou choisis un preset.
gallery-settings-title = Paramètres du lecteur
gallery-settings-desc = Configure audio, comportement, intégrations et plus.
gallery-downloader-title = Téléchargeur
gallery-downloader-desc = Télécharge des pistes directement depuis Kopuz.
gallery-prev = ← Précédent
gallery-next = Suivant →
gallery-label-normal-home = Normal — Accueil
gallery-label-modern-home = Modern — Accueil
gallery-label-normal-library = Normal — Bibliothèque
gallery-label-vaxry-library = Vaxry — Bibliothèque
gallery-label-normal-playlist = Normal — Playlist
gallery-label-modern-playlist = Modern — Playlist
gallery-label-fullscreen = Lecteur plein écran
gallery-label-fullscreen-lyrics = Paroles plein écran
gallery-label-search = Recherche
gallery-label-theme-editor = Éditeur de thème
gallery-label-player-settings = Paramètres du lecteur
gallery-label-downloader = Téléchargeur

## Footer
footer-license = Licence MIT — Gratuit & Open Source
footer-github = GitHub
footer-releases = Versions
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz est un lecteur de musique moderne et léger construit avec Rust et Dioxus. Streame depuis Jellyfin ou Navidrome, parcours fichiers locaux, paroles synchronisées, égaliseur, thèmes et plus.
home-meta-keywords = Kopuz, lecteur musique, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, paroles
og-title = Kopuz — Music Player
og-desc = Lecteur de musique moderne et léger construit avec Rust. Fichiers locaux, Jellyfin, Navidrome, paroles synchronisées, égaliseur, Discord RPC et plus. Gratuit et open source.
og-image-alt = Lecteur de musique Kopuz
twitter-title = Kopuz — Music Player
twitter-desc = Lecteur de musique moderne et léger construit avec Rust. Gratuit et open source.

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
