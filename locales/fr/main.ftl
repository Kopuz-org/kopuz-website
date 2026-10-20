## Language

## Nav
nav-features = Fonctionnalités
nav-download = Télécharger
nav-lang-label = Langue

## Hero
hero-cta-download = Télécharger
hero-cta-github = Voir sur GitHub
hero-screenshot-alt = Écran d’accueil de Kopuz

## Features
features-title = Fonctionnalités
features-source-local = Fichiers locaux
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = API Subsonic
features-source-nextcloud = Nextcloud
features-source-ytmusic = YouTube Music
features-source-applemusic = Apple Music
features-source-soundcloud = SoundCloud

feat-theming-title = Thèmes
feat-native-title = Intégration native
feat-android-title = Android
feat-android-desc = Une version ARM64 adaptée aux écrans de téléphone, avec des gestes de balayage, les commandes de lecture aléatoire et de répétition dans la notification multimédia, et un mode AMOLED pour Android 7.0 et versions ultérieures.
feat-lyrics-title = Paroles synchronisées
feat-lyrics-desc = Paroles synchronisées, minutées mot à mot ou non synchronisées, avec des pauses instrumentales, une ligne active centrée, un flou de profondeur et un décalage temporel manuel ou automatique.
feat-eq-title = Égaliseur
feat-eq-desc = Égaliseur 5 bandes avec presets intégrés et contrôle complet sur ton son.
feat-fav-title = Favoris & Playlists
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobbling ListenBrainz intégré. Les utilisateurs Jellyfin peuvent aussi utiliser le plugin listenbrainz.
feat-radio-title = Radio Internet
feat-radio-desc = Écoute des stations Internet issues d’annuaires de radios, avec les métadonnées du flux et du titre en cours.
feat-offline-title = Téléchargements hors ligne
feat-offline-desc = Télécharge les pistes distantes prises en charge pour les écouter sans connexion réseau.
feat-fonts-title = Polices d’interface personnalisées
feat-fonts-desc = Choisis ta propre police d’interface en complément des thèmes intégrés.
feat-discord-title = Discord RPC
feat-discord-desc = Montre à tes amis ce que tu écoutes avec Discord Rich Presence intégré. Aucune configuration.
feat-search-title = Recherche
feat-search-desc = Recherche en temps réel sur artistes, albums et pistes. Résultats instantanés.
feat-genre-title = Navigation par genre
feat-genre-desc = Parcours les fichiers locaux et la musique du serveur par genre.
feat-logs-title = Journaux d'écoute
feat-logs-desc = Compteurs de lecture suivis localement. Vois ce que tu écoutes vraiment le plus.
feat-i18n-title = i18n
feat-channels-title = Modes canaux
feat-channels-desc = Stéréo, Mono, Gauche-seul, Droite-seule, et échange L/R. Contrôle fin des canaux audio.
feat-youtube-title = YouTube Music
feat-applemusic-title = Apple Music
feat-applemusic-desc = Parcours le catalogue et ta bibliothèque, synchronise les favoris, ajoute ou retire des pistes des playlists, lance la radio, télécharge des pistes et utilise les paroles minutées mot à mot. La lecture sur ordinateur nécessite Widevine.
feat-nextcloud-title = Nextcloud
feat-nextcloud-desc = Connecte-toi via WebDAV, choisis les dossiers de bibliothèque de chaque serveur, écoute des pistes en streaming ou télécharge-les, mets les pochettes en cache et détermine les durées depuis les en-têtes des fichiers.
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
feat-badges-desc = Les pistes locales affichent leur format, par exemple MP3, FLAC ou WAV, directement dans la ligne de la piste.

## Nouveautés

## Choix du téléchargement

## Performance
perf-skip-label = Ignorer déjà indexés
perf-skip-desc = Les rescans ne traitent que les nouveaux fichiers. 10k pistes + 5 nouvelles = seulement 5 lues.
perf-parallel-label = Démarrage parallèle
perf-art-label = Cache pochettes
perf-art-desc = Pochettes extraites une fois, sauvegardées sur disque. Jamais redécodées à la consultation.
perf-lazy-label = Chargement d'images paresseux
perf-lazy-desc = Les pochettes dans les résultats de recherche ne se chargent que lorsqu’elles sont visibles.
perf-io-label = I/O non-bloquant
perf-http-label = Cache HTTP pochettes
perf-sort-label = Tri plus intelligent

## Install
install-nix-title = Nix / NixOS
install-nix-run = Lancer sans installer :
install-nix-profile = Ou ajouter à ton profil :
install-nix-note = Flake NixOS supporté avec cache binaire Cachix.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Installe avec ton helper préféré :
install-flatpak-title = Flatpak
install-flatpak-desc = Installez Kopuz depuis Flathub :
install-flatpak-note = Les mises à jour arriveront via le processus habituel de mise à jour de Flatpak.
install-appimage-title = AppImage
install-appimage-desc-1 = Télécharge depuis GitHub Releases et lance directement. Nécessite
install-appimage-desc-2 = { " " }et
install-appimage-desc-3 = { " " }sur ton système.
install-appimage-note-1 = Utilisateurs Arch : si ça crashe avec une erreur WebKitNetworkProcess, lance avec le préfixe
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-desc-1 = Télécharge le
install-macos-desc-2 = { " " }depuis GitHub Releases. Si macOS bloque, retire le flag quarantaine :

## Platforms
platforms-title = Télécharger Kopuz
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apple Silicon uniquement
platforms-linux = Linux

## Support
support-title = Soutenir Kopuz
support-subtitle = Kopuz est gratuit et open source. Les dons soutiennent le développement.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-usdt-note = (chaîne Solana)

## Donate

## Nextcloud
nextcloud-subtitle = Ajoute-le dans Paramètres → Serveurs média → Ajouter → Nextcloud.
nextcloud-connect-title = Utilise un mot de passe d’application
nextcloud-connect-desc = Saisis l’URL de base de ton instance Nextcloud, ton nom d’utilisateur et un mot de passe d’application révocable créé sous Paramètres Nextcloud → Sécurité.
nextcloud-folders-title = Choisis les dossiers de la bibliothèque
nextcloud-folders-desc = Sélectionne un ou plusieurs dossiers pour chaque source. Kopuz parcourt l’arborescence de dossiers WebDAV, met en cache les pochettes disponibles et analyse les en-têtes des fichiers pour déterminer la durée des pistes.
nextcloud-webdav-title = Fonctionnement en WebDAV brut
nextcloud-webdav-desc = La bibliothèque suit l’organisation des dossiers et ne fournit ni playlists ni radio. Si Nextcloud Music expose Subsonic, utilise cette source pour bénéficier de métadonnées plus riches et de playlists.
nextcloud-playback-title = Écoute en streaming ou télécharge
nextcloud-playback-desc = Lis les fichiers directement depuis ton serveur ou télécharge-les pour une écoute hors ligne, sans faire transiter l’audio par l’infrastructure de Kopuz.

## YouTube Music
ytmusic-subtitle = Ajoute-le depuis Paramètres → Serveurs média → Ajouter → YouTube Music.
ytmusic-token-title = Aucun helper requis
ytmusic-signin-title = Connexion via le navigateur
ytmusic-signin-desc = Kopuz ouvre la connexion Google dans un profil de navigateur distinct et lit les cookies de session obtenus. La connexion donne accès à ta bibliothèque, à ta musique aimée, à tes playlists et aux artistes suivis.
ytmusic-anon-title = Mode anonyme
ytmusic-anon-desc = Pas de connexion, pas de cookies. Navigue, recherche, ouvre les pages d'artistes, d'albums et de playlists, lance la radio mix, et joue les pistes publiques. Les favoris et les vues de bibliothèque sont désactivés.

## Apple Music
applemusic-subtitle = Ajoute-le dans Paramètres → Serveurs média → Ajouter → Apple Music.
applemusic-signin-title = Connecte-toi ou colle un jeton
applemusic-signin-desc = Sur ordinateur, Kopuz ouvre Apple Music dans un profil isolé d’un navigateur de la famille Chromium. Tu peux aussi coller manuellement un media-user-token.
applemusic-playback-title = Widevine sur ordinateur
applemusic-playback-desc = La lecture Apple Music nécessite un CDM Widevine provenant d’un navigateur installé ou du service de plug-ins de Mozilla. Kopuz ne fournit pas le CDM.
applemusic-features-title = Ce que tu obtiens
applemusic-features-desc = Navigation dans le catalogue et la bibliothèque, synchronisation des favoris, ajout et retrait de pistes des playlists, radio à partir d’une piste ou d’une playlist, téléchargements et paroles minutées mot à mot.
applemusic-android-title = État sur Android
applemusic-android-desc = La connexion utilise une WebView intégrée à l’application, mais la lecture Apple Music n’est pas encore prise en charge sur Android.

## SoundCloud
soundcloud-subtitle = Ajoute-le depuis Paramètres → Serveurs média → Ajouter → SoundCloud.
soundcloud-signin-title = Connexion navigateur unique
soundcloud-features-title = Ce que tu obtiens
soundcloud-features-desc = Recherche, lecture de pistes (MP3 progressif plus flux Go+ AAC/HLS), tes pistes aimées comme favoris, playlists en lecture seule, et like/unlike. Retirer la source nettoie son profil isolé.

## Guides
guides-title = Guides
## Confidentialité et stockage

## Configuration requise

## Communauté
community-title = Contribuer à Kopuz

## Sponsors
sponsors-cta = Devenir sponsor

## Gallery

## Footer
footer-license = Licence EUPL v1.2 | Gratuit et open source
footer-github = GitHub
footer-releases = Versions
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz : lecteur de musique gratuit et open source
og-title = Kopuz : lecteur de musique gratuit et open source
og-image-alt = Lecteur de musique Kopuz
twitter-title = Kopuz : lecteur de musique gratuit et open source
## User-friendly aliases (synced with EN updates)
perf-parallel-desc = { perf-parallel-desc-1 } { perf-parallel-desc-2 }
perf-io-desc = { perf-io-desc-1 } { perf-io-desc-2 }
perf-http-desc = { perf-http-desc-1 } { perf-http-desc-2 }
perf-sort-desc = { perf-sort-desc-1 } { perf-sort-desc-2 }

## Deep link (/j)
join-title = Ouverture de Kopuz…
join-opening = Ouverture de Kopuz…
join-fallback = Kopuz ne s'est pas ouvert. Pas encore installé ?
join-no-payload = Ce lien ne contient aucune file d'attente.
join-download = Télécharger Kopuz

## Privacy policy
footer-privacy = Confidentialité
privacy-english-note = Cette politique est rédigée en anglais afin que ses termes restent exacts. Les traductions du reste du site n'en modifient pas le contenu.
privacy-back = Retour à l'accueil

## Added strings
hero-title-1 = Un seul lecteur pour
hero-title-2 = toute ta musique.
hero-desc = Kopuz réunit tes fichiers locaux, ton NAS, ton propre serveur et tes comptes de streaming dans une seule bibliothèque. Il est gratuit et open source.
features-source-spotify = Spotify
feat-local-title = Dossiers et serveurs
feat-local-desc = Ajoute des dossiers locaux ou connecte un serveur Jellyfin ou Subsonic, comme Navidrome. Les titres de chaque source ajoutée apparaissent dans une seule bibliothèque, et Subsonic garde ta file d’attente synchronisée entre les clients.
feat-theming-desc = Choisis parmi 24 thèmes clairs et sombres intégrés, ou crée le tien et règle chaque couleur toi-même.
feat-native-desc = Contrôle la lecture avec les touches et les commandes multimédias de ton système : MPRIS sous Linux, Now Playing sous macOS et la surimpression multimédia sous Windows.
feat-fav-desc = Garde tes favoris sur ton appareil ou synchronise-les avec ton serveur. Crée des playlists, ajoute des albums entiers d’un coup et synchronise tes playlists avec ton serveur.
feat-i18n-desc = Utilise Kopuz dans l’une des 28 langues disponibles.
feat-ytdlp-title = Téléchargeur YouTube
feat-ytdlp-desc = Colle un lien YouTube ou YouTube Music vers un titre, un album ou une playlist. Kopuz le télécharge avec son propre client YouTube, ajoute le titre, l’artiste, l’album et la pochette, puis le range par album. La meilleure qualité audio conserve le flux d’origine et ne demande rien d’installé. Pour convertir en MP3, FLAC ou WAV, installe ffmpeg.
feat-crossfade-title = Fondu enchaîné et lecture sans interruption
feat-crossfade-desc = Fais un fondu entre les titres pendant la lecture sur ordinateur, ou désactive le fondu enchaîné pour les albums sans interruption.
feat-youtube-desc = Parcours, recherche et lance une radio mix à partir de n’importe quel titre. Découvrir propose des titres, des albums, des artistes et des ambiances, et les pages d’artiste affichent les meilleurs titres, les sorties et les artistes similaires. Connecte-toi pour accéder à ta bibliothèque, à Titres likés et à tes playlists, ou navigue sans compte.
feat-spotify-title = Spotify
feat-spotify-desc = Parcours tes titres enregistrés, albums, playlists et Découvrir, recherche et scrobble. Écoute avec le Web Playback SDK de Spotify ou sur n’importe lequel de tes appareils Spotify Connect. Il te faut Spotify Premium pour lire de la musique.
new-title = Nouveautés de Kopuz
install-cargo-title = Cargo (crates.io)
install-cargo-desc = Installe la dernière version avec Cargo :
install-macos-homebrew = Installe la version Apple silicon depuis le tap Homebrew :
install-android-title = Android
install-android-desc = Récupère Kopuz sur Google Play, ou télécharge l’APK arm64-v8a signé depuis GitHub Releases. Il te faut Android 7.0 (API 24) ou une version plus récente.
install-android-note = Si ton APK installé vient d’une autre source, désinstalle-le avant d’installer celui-ci.
ytmusic-token-desc = Tu peux écouter sans te connecter, et tu n’as pas besoin d’installer d’outil auxiliaire.
ytmusic-signin-note = Kopuz utilise ton navigateur par défaut. Tu peux choisir Firefox, Chrome, Chromium, Brave, Edge, Vivaldi, Helium, LibreWolf, Zen ou Floorp à la place.
ytmusic-premium-title = Qualité Premium
ytmusic-premium-desc = Avec un compte Music Premium, Kopuz lit et télécharge les flux de meilleure qualité avec son propre client YouTube. Le mode anonyme ne peut pas lire les titres réservés à Premium.
soundcloud-signin-desc = Tu n’as besoin ni d’URL, ni de mot de passe, ni de jeton. Kopuz ouvre la connexion SoundCloud dans un profil de navigateur séparé et supprime ce profil quand tu retires la source.
spotify-guide-subtitle = Kopuz utilise la Web API officielle de Spotify et la lecture dans le navigateur. Il ne voit jamais ton mot de passe et ne relaie pas l’audio.
spotify-step-1-title = Crée une application Spotify
spotify-step-1-desc = Dans le tableau de bord développeur de Spotify, crée une application et active Web API et Web Playback SDK. Sous User Management, ajoute chaque compte qui écoutera (jusqu’à cinq en Development Mode). Puis définis exactement cet URI de redirection :
spotify-step-2-title = Ajoute ton Client ID
spotify-step-2-desc = Dans Paramètres → Serveurs média → Ajouter → Spotify, colle le Client ID et autorise l’accès. Tu n’as pas besoin de Client Secret. Le port 8898 doit être libre pendant la connexion.
spotify-step-3-title = Choisis où lire
spotify-step-3-desc = Garde l’onglet du lecteur de l’application ouvert, ou choisis l’un de tes appareils Spotify Connect dans le menu des appareils.
spotify-requirement = Il te faut Spotify Premium pour lire de la musique. Utilise Chrome, Edge, Brave, Chromium, Vivaldi, Helium ou Safari sous macOS. Firefox ne fonctionne pas.
spotify-full-guide = Lis le guide complet de configuration et de dépannage de Spotify
privacy-local-title = Une base de données locale
privacy-local-desc = Kopuz garde ta bibliothèque, tes playlists, tes favoris, tes nombres de lectures et les données de tes sources dans kopuz.db, et tes préférences à côté, dans settings.toml.
privacy-accounts-title = Connexions séparées
privacy-accounts-desc = Chaque source garde sa propre connexion et ses propres favoris. Les connexions par navigateur utilisent des profils séparés, pas ta session de navigateur habituelle.
privacy-files-title = Cache et journaux
privacy-files-desc = Les pochettes et les titres hors ligne vont dans le dossier de cache de ton système. Tu peux ouvrir ou exporter les journaux depuis les Paramètres.
requirements-title = Prérequis et limites
about-title = Qu’est-ce qu’un kopuz ?
about-desc-1 = Le kopuz est un ancien instrument à cordes turcique que jouaient les bardes et les chamanes. On pense qu’il est l’ancêtre de plusieurs luths d’Asie centrale.
about-desc-2 = Dans la légende turcique, il appartient à Dede Korkut, un barde. Le komuz kirghize et le kobyz kazakh sont des instruments apparentés. Le xomus, malgré son nom proche, n’en est pas un.
community-issues-title = Issues
community-issues-desc = Signale un bug ou prends une tâche ouverte.
community-discussions-title = Discussions
community-discussions-desc = Pose des questions et propose des changements.
community-discord-desc = Discute avec les utilisateurs et les contributeurs.
community-contribute-title = Contribuer
community-contribute-desc = Récupère le code source, configure un environnement de développement ou aide à traduire.
platforms-android = Android
platforms-android-note = ARM64 · Android 7.0+ · API 24+
home-meta-desc = Kopuz est un lecteur de musique gratuit pour ordinateur et Android. Lis tes fichiers, SMB, Jellyfin, YouTube Music, Spotify et plus dans une bibliothèque.
home-meta-keywords = Kopuz, lecteur de musique, Jellyfin, Navidrome, Subsonic, SMB, NAS, Clippsly, Nextcloud, YouTube Music, Apple Music, SoundCloud, Spotify, open source, Android, Linux, macOS, Windows, paroles, égaliseur
og-desc = Un lecteur de musique gratuit pour les fichiers locaux, les serveurs et les services de streaming sur ordinateur et Android, avec paroles synchronisées, thèmes et égaliseur 10 bandes.
twitter-desc = Lis des fichiers locaux, des partages SMB, Jellyfin, Subsonic, YouTube Music, SoundCloud, Spotify et plus dans une seule bibliothèque.
nav-home = Accueil
footer-license-line = EUPL 1.2, gratuit et open source
shelf-link-to-us = Fais un lien vers nous avec un bouton 88×31
hero-release = Dernière version
hero-cta-download-for = Télécharger pour
hero-cta-all = Toutes les plateformes
home-sources-title = Écoute de la musique depuis
source-local = Dossiers locaux
source-navidrome = Navidrome et Subsonic
source-radio = Radio internet
moment-lyrics-title = Des paroles en rythme
moment-lyrics-alt = Kopuz lisant un titre avec des paroles synchronisées en plein écran
moment-library-title = Une bibliothèque pour toutes les sources
moment-library-alt = La bibliothèque Kopuz avec des albums de plusieurs sources
moment-themes-title = Tes couleurs, ou celles de ton fond d’écran
moment-themes-alt = L’éditeur de thèmes de Kopuz
moment-themes-extra = Ou laisse Kopuz créer un thème à partir du fond d’écran de ton bureau.
theme-default = Gruvbox dark soft
theme-amoled = AMOLED
theme-wallpaper = Fond d’écran
wallpaper-sunset = Coucher de soleil
wallpaper-lake = Lac
home-see-all-features = Voir toutes les fonctionnalités
home-get-title = Obtenir Kopuz
home-get-download = Télécharger
home-also-on = Tu peux aussi installer Kopuz avec Cargo, Nix, l’AUR, Flatpak ou Homebrew.
home-sponsors-title = Sponsors
home-sponsor-cta = Sponsoriser sur GitHub
copy = Copier
copied = Copié
copy-address = Copier l’adresse { $coin }
nav-primary-aria = Navigation principale
nav-menu-open = Ouvrir le menu
nav-menu-close = Fermer le menu
footer-nav-aria = Pied de page
theme-use-light = Utiliser le thème clair
theme-use-dark = Utiliser le thème sombre
theme-leave-moe = Quitter le mode moe
mode-simple = Site simple
mode-full = Site complet
webbutton-alt = Bouton web Kopuz 88x31
features-page-title = Fonctions de Kopuz : sources, paroles, thèmes et lecture
download-page-title = Télécharger Kopuz pour Windows, macOS, Linux et Android
guides-page-title = Guides de configuration Kopuz : serveurs et streaming
support-page-title = Soutenir Kopuz : sponsors, dons et communauté
notfound-page-title = Page introuvable | Kopuz
guide-title-suffix = guide de configuration | Kopuz
notfound-title = Page introuvable.
notfound-desc = Rien n’est en cours de lecture à cette adresse.
notfound-home = Aller à la page d’accueil
features-group-sources = Sources
features-group-playback = Lecture
features-group-library = Bibliothèque
features-group-desktop = Ordinateur
features-group-android = Android
features-group-integrations = Intégrations
features-group-performance = Performances
features-group-privacy = Confidentialité
features-jump-aria = Groupes de fonctionnalités
features-privacy-link = Lis la politique de confidentialité
install-other-title = Autres façons d’installer
req-spotify = Il te faut Premium, ton propre Client ID et un navigateur compatible pour lire de la musique.
req-applemusic = La lecture sur ordinateur demande un Widevine CDM. Tu peux te connecter sur Android, mais la lecture n’y fonctionne pas encore.
req-nextcloud = WebDAV simple ne prend pas en charge les playlists ni la radio. Si Nextcloud Music propose un endpoint Subsonic, utilise-le à la place.
req-android = Les APK de version demandent un téléphone arm64-v8a sous Android 7.0 (API 24) ou plus récent. Le statut Discord et la zone de notification ne sont pas disponibles.
req-appimage = Nécessite webkit2gtk-4.1 et GTK 3. La zone de notification demande aussi une bibliothèque appindicator.
req-ytmusic = Le mode anonyme ne peut pas lire les titres réservés à Premium. Connecte-toi avec un compte Music Premium pour les lire.
req-crossfade = Le fondu enchaîné et la lecture sans interruption fonctionnent avec la lecture native sur ordinateur, pas avec l’audio Spotify lu dans le navigateur.
req-spotify-limits-label = Limites de Spotify
req-spotify-limits = En Development Mode, une application accepte jusqu’à cinq auditeurs, la recherche renvoie jusqu’à dix résultats par type, les playlists sont en lecture seule, et les téléchargements, la radio, l’édition des tags, l’égaliseur et le fondu enchaîné sont désactivés.
support-note = Je suis Temidaradev et je développe Kopuz. Je suis étudiant sans revenu stable, donc ce sont les sponsors qui me permettent de continuer à travailler sur Kopuz pendant mes études.
support-goal-label = Objectif GitHub Sponsors
support-goal-amount = ${ $current } sur ${ $goal } par mois
support-goal-aria = Progression de l’objectif GitHub Sponsors
support-crypto-summary = Adresses crypto
sponsors-tier-special = Spécial
sponsors-tier-monthly = Mensuel
sponsors-tier-one-time = Ponctuel
features-source-smb = SMB et NAS
features-source-clippsly = Clippsly
feat-smb-title = SMB et NAS
feat-smb-desc = Ajoute un partage SMB 2 ou 3, ou un dossier à l’intérieur, comme bibliothèque. Kopuz lit les tags intégrés et les pochettes, et lit directement depuis le partage avec recherche dans le titre, donc tu n’as pas à le monter.
feat-clippsly-title = Clippsly
feat-clippsly-desc = Connecte-toi avec ton nom d’utilisateur Clippsly et un mot de passe d’application créé sur clippsly.app. Écoute et télécharge depuis ton locker, envoie des fichiers audio et vérifie ton stockage utilisé. Les playlists distantes ne sont pas encore prises en charge.
feat-updates-title = Mises à jour automatiques
feat-updates-desc = Sur ordinateur, une bannière t’indique quand une nouvelle version est disponible, et Kopuz la télécharge et l’installe pour toi.
feat-cli-title = Daemon et ligne de commande
feat-cli-desc = Le lecteur tourne comme un daemon, kopuzd, auquel la fenêtre se connecte. Utilise kopuzctl pour contrôler la lecture, configurer les sources et lancer des analyses de la bibliothèque depuis un terminal ou un script, ou parle directement à l’API gRPC.
install-android-play = Disponible sur Google Play
footer-made-with = Site web fait avec
footer-love = amour
footer-made-by = par
source-smb = SMB et NAS
source-clippsly = Clippsly
player-prev-label = Section précédente
player-next-label = Section suivante
player-seek-label = Position dans la page
player-queue-label = Sections de cette page
player-queue-title = Sur cette page
dl-file-exe = Programme d’installation (.exe)
dl-file-msi = Installateur (.msi)
dl-file-dmg = Image disque (.dmg)
dl-file-appimage = AppImage
dl-file-deb = Debian et Ubuntu (.deb)
dl-file-rpm = Fedora et RHEL (.rpm)
dl-file-apk = APK
dl-release-page = Page de la version
dl-this-device = Cet appareil
dl-dialog-ready = Dernière version : { $version }. Choisis le programme d’installation pour ton appareil.
dl-dialog-close = Fermer
dl-dialog-release = Voir la version sur GitHub
guide-nextcloud-summary = Connecte-toi via WebDAV avec un mot de passe d’application, puis choisis les dossiers de ta bibliothèque.
guide-ytmusic-summary = Connecte-toi via ton navigateur, ou écoute sans compte.
guide-applemusic-summary = Connecte-toi, fais fonctionner Widevine sur ordinateur et vois ce qui marche sur Android.
guide-soundcloud-summary = Connecte-toi une fois dans ton navigateur et vois ce que la source prend en charge.
guide-spotify-summary = Crée une application développeur, ajoute son Client ID et choisis où lire.
chapter-home = Accueil
chapter-sources = Sources
chapter-lyrics = Paroles
chapter-library = Bibliothèque
chapter-themes = Thèmes
chapter-download = Télécharger
chapter-sponsors = Sponsors
chapter-whats-new = Nouveautés
chapter-name = Le nom
chapter-not-found = Introuvable
chapter-privacy = Confidentialité
chapter-install = Installer
chapter-requirements = Prérequis
chapter-support = Soutien
chapter-community = Communauté
chapter-features = Fonctionnalités
chapter-guides = Guides
guide-smb-summary = Pointe Kopuz vers un partage de ton NAS et connecte-toi avec ton compte NAS.
smb-subtitle = Choisis SMB / NAS dans le sélecteur de source. Tu n’as pas besoin de monter le partage d’abord.
smb-share-title = Pointe vers un partage
smb-share-desc = Saisis l’URL d’un partage, ou d’un dossier à l’intérieur, par exemple :
smb-share-port = Si ton NAS utilise un autre port que celui par défaut, ajoute-le à l’URL :
smb-signin-title = Connecte-toi avec ton compte NAS
smb-signin-desc = Utilise le nom d’utilisateur et le mot de passe de ton NAS. Pour un compte de domaine, saisis DOMAIN\username. Kopuz demande SMB 2 ou 3 et un compte : il ne cherche pas de partages sur le réseau et ne se connecte pas en invité.
smb-library-title = Comment fonctionne la bibliothèque
smb-library-desc = Kopuz analyse le partage et ses sous-dossiers à la recherche de fichiers audio, et lit leurs tags intégrés et leurs pochettes. Il lit directement depuis le partage, avec recherche dans le titre, aussi sur Android. Tes favoris sont conservés dans Kopuz.
smb-limits-title = Le partage reste en lecture seule
smb-limits-desc = Kopuz ne modifie rien sur le partage, donc tu ne peux pas y modifier les tags, y garder des playlists ni télécharger ses titres pour les écouter hors ligne.
guide-clippsly-summary = Connecte-toi avec un mot de passe d’application, écoute ton locker et envoie de la musique.
clippsly-subtitle = Ajoute Clippsly depuis les paramètres des sources.
clippsly-signin-title = Connecte-toi avec un mot de passe d’application
clippsly-signin-desc = Crée un mot de passe d’application sur clippsly.app. Puis connecte-toi à Kopuz avec ton nom d’utilisateur Clippsly et ce mot de passe d’application.
clippsly-locker-title = Ton locker
clippsly-locker-desc = Parcours et écoute la musique de ton locker Clippsly, et télécharge des titres pour les écouter hors ligne.
clippsly-upload-title = Envoyer de la musique
clippsly-upload-desc = Envoie des fichiers audio depuis la carte de paramètres de la source. Kopuz actualise ta bibliothèque quand tout le lot est terminé. La carte affiche aussi ton stockage utilisé ; Clippsly fixe ton quota.
clippsly-bonus = Associe Kopuz à ton compte Clippsly et Clippsly ajoute 3 Go de stockage gratuitement.
clippsly-limits-title = Playlists
clippsly-limits-desc = Les playlists que tu gardes sur Clippsly n’apparaissent pas encore dans Kopuz.
clippsly-site = Aller sur clippsly.app
