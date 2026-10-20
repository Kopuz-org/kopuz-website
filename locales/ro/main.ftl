## Language

## Nav
nav-features = Funcții
nav-download = Descarcă
nav-lang-label = Limbă

## Hero
hero-cta-download = Descarcă
hero-cta-github = Vezi pe GitHub
hero-screenshot-alt = Ecranul principal Kopuz

## Features
features-title = Funcții
features-source-local = Fișiere locale
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-nextcloud = Nextcloud
features-source-ytmusic = YouTube Music
features-source-applemusic = Apple Music
features-source-soundcloud = SoundCloud

feat-theming-title = Teme
feat-native-title = Integrare nativă
feat-android-title = Android
feat-android-desc = O versiune ARM64 adaptată telefoanelor, cu gesturi de glisare, comenzi de redare aleatorie și repetare în notificarea media și mod AMOLED pentru Android 7.0 sau mai nou.
feat-lyrics-title = Versuri sincronizate
feat-lyrics-desc = Versuri sincronizate pe rânduri sau pe cuvinte, precum și versuri simple, cu pauze instrumentale, versul activ centrat, estompare în funcție de profunzime și ajustarea manuală sau automată a decalajului de sincronizare.
feat-eq-title = Egalizator
feat-eq-desc = Egalizator cu 5 benzi cu presetări încorporate și control complet al sunetului.
feat-fav-title = Favorite și liste
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobbling ListenBrainz încorporat. Utilizatorii Jellyfin pot folosi și plugin-ul listenbrainz.
feat-radio-title = Radio pe internet
feat-radio-desc = Redă posturi din cataloage radio, cu metadate despre flux și piesa în curs de redare.
feat-offline-title = Descărcări offline
feat-offline-desc = Descarcă piese din sursele la distanță compatibile și ascultă-le fără conexiune la rețea.
feat-fonts-title = Fonturi personalizate pentru interfață
feat-fonts-desc = Alege propriul font pentru interfață, pe lângă temele încorporate.
feat-discord-title = Discord RPC
feat-discord-desc = Arată prietenilor ce asculți cu Discord Rich Presence încorporat. Fără setare.
feat-search-title = Căutare
feat-search-desc = Căutare în timp real în artiști, albume și piese. Rezultate instant în timp ce scrii.
feat-genre-title = Răsfoire după gen
feat-genre-desc = Răsfoiește toată biblioteca după gen. Funcția este disponibilă pentru fișiere locale și muzică de pe server.
feat-logs-title = Jurnale de ascultare
feat-logs-desc = Contoare de redare urmărite local. Vezi ce asculți cu adevărat mai mult de-a lungul timpului.
feat-i18n-title = i18n
feat-channels-title = Moduri canal
feat-channels-desc = Stereo, Mono, Doar-Stânga, Doar-Dreapta și schimbare L/R. Control fin al canalelor audio.

feat-youtube-title = YouTube Music
feat-applemusic-title = Apple Music
feat-applemusic-desc = Răsfoiește catalogul și biblioteca ta, sincronizează favoritele, adaugă sau elimină piese din listele de redare, pornește radio, descarcă piese și folosește versuri sincronizate la nivel de cuvânt. Redarea pe desktop necesită Widevine.
feat-nextcloud-title = Nextcloud
feat-nextcloud-desc = Conectează-te prin WebDAV, alege folderele bibliotecii pentru fiecare server, redă în flux sau descarcă piese, memorează coperțile în cache și determină durata pieselor din antetele fișierelor.
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
feat-badges-desc = Rândurile pieselor locale afișează formatul fișierului, de exemplu MP3, FLAC sau WAV.

## What's new

## Install chooser

## Performance
perf-skip-label = Sări peste cele deja indexate
perf-skip-desc = Rescanările procesează doar fișiere noi. 10k piese + 5 noi = doar 5 citite.
perf-parallel-label = Pornire paralelă
perf-art-label = Cache coperți album
perf-art-desc = Coperțile sunt extrase o singură dată, salvate pe disc. Nu sunt niciodată decodificate din nou la vizionări repetate.
perf-lazy-label = Încărcare leneșă imagini
perf-lazy-desc = Rezultatele căutării pot conține sute de coperți. Imaginile se încarcă atunci când devin vizibile.
perf-io-label = I/O non-blocant
perf-http-label = Cache HTTP coperți
perf-sort-label = Sortare mai inteligentă

## Install
install-nix-title = Nix / NixOS
install-nix-run = Rulează fără instalare:
install-nix-profile = Sau adaugă la profil:
install-nix-note = NixOS flake suportat cu cache binar Cachix.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Instalează cu helper-ul preferat:
install-flatpak-title = Flatpak
install-flatpak-desc = Instalează Kopuz de pe Flathub:
install-flatpak-note = Actualizările vor fi livrate prin mecanismul obișnuit de actualizare Flatpak.
install-appimage-title = AppImage
install-appimage-desc-1 = Descarcă de pe GitHub Releases și rulează direct. Necesită
install-appimage-desc-2 = { " " }și
install-appimage-desc-3 = { " " }pe sistemul tău.
install-appimage-note-1 = Utilizatori Arch: dacă crash-uiește cu eroare WebKitNetworkProcess, rulează cu prefixul
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-desc-1 = Descarcă
install-macos-desc-2 = { " " }de pe GitHub Releases. Dacă macOS blochează, șterge flag-ul de carantină:

## Platforms
platforms-title = Descarcă Kopuz
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Doar Apple Silicon
platforms-linux = Linux

## Support
support-title = Susține Kopuz
support-subtitle = Kopuz este gratuit și open source. Donațiile susțin dezvoltarea.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-usdt-note = (rețeaua Solana)

## Nextcloud
nextcloud-subtitle = Adaugă-l din Setări → Servere media → Adaugă → Nextcloud.
nextcloud-connect-title = Folosește o parolă de aplicație
nextcloud-connect-desc = Introdu URL-ul de bază al instanței Nextcloud, numele de utilizator și o parolă de aplicație revocabilă creată în Nextcloud, din Setări → Securitate.
nextcloud-folders-title = Alege folderele bibliotecii
nextcloud-folders-desc = Alege unul sau mai multe foldere pentru fiecare sursă. Kopuz citește arborele de foldere WebDAV, memorează în cache coperțile disponibile și analizează antetele fișierelor pentru a determina durata pieselor.
nextcloud-webdav-title = Acces WebDAV direct
nextcloud-webdav-desc = Biblioteca respectă structura folderelor și nu oferă liste de redare sau radio. Dacă Nextcloud Music pune la dispoziție API-ul Subsonic, folosește acea sursă pentru metadate mai bogate și liste de redare.
nextcloud-playback-title = Redă în flux sau descarcă
nextcloud-playback-desc = Redă fișiere direct de pe serverul tău sau descarcă-le pentru ascultare offline, fără ca fluxul audio să treacă prin infrastructura Kopuz.

## YouTube Music
ytmusic-subtitle = Adaugă-l din Setări → Servere media → Adaugă → YouTube Music.
ytmusic-token-title = Niciun helper necesar
ytmusic-signin-title = Conectare cu un browser
ytmusic-signin-desc = Kopuz deschide conectarea Google într-un profil separat de browser și preia cookie-urile sesiunii. Nu folosește profilul tău obișnuit. Conectarea oferă acces la bibliotecă, Liked Music, liste și artiștii urmăriți.
ytmusic-anon-title = Mod anonim
ytmusic-anon-desc = Fără conectare, fără cookie-uri. Răsfoiește, caută, deschide pagini de artiști, albume și liste, pornește radio mix și redă piese publice. Aprecierea și vizualizările bibliotecii sunt dezactivate.

## Apple Music
applemusic-subtitle = Adaugă-l din Setări → Servere media → Adaugă → Apple Music.
applemusic-signin-title = Conectează-te sau lipește un token
applemusic-signin-desc = Pe desktop, Kopuz deschide Apple Music într-un profil izolat al unui browser din familia Chromium. Alternativ, poți lipi manual un media-user-token.
applemusic-playback-title = Widevine pe desktop
applemusic-playback-desc = Redarea din Apple Music necesită un modul CDM Widevine dintr-un browser instalat sau din serviciul de pluginuri Mozilla. Kopuz nu include modulul CDM.
applemusic-features-title = Ce primești
applemusic-features-desc = Răsfoirea catalogului și a bibliotecii, sincronizarea favoritelor, adăugarea sau eliminarea pieselor din listele de redare, radio bazat pe piese și liste de redare, descărcări și versuri sincronizate la nivel de cuvânt.
applemusic-android-title = Disponibilitate pe Android
applemusic-android-desc = Conectarea folosește un WebView în aplicație, dar redarea Apple Music nu este încă disponibilă pe Android.

## SoundCloud
soundcloud-subtitle = Adaugă-l din Setări → Servere media → Adaugă → SoundCloud.
soundcloud-signin-title = Conectare unică prin browser
soundcloud-features-title = Ce primești
soundcloud-features-desc = Căutare, redare de piese (MP3 progresiv plus fluxuri Go+ AAC/HLS), piesele tale Liked ca favorite, liste doar pentru citire și apreciere/dezapreciere. Eliminarea sursei curăță profilul său izolat.

## Feature guides
guides-title = Ghiduri
## Privacy and storage

## Requirements

## Community
community-title = Contribuie la Kopuz

## Sponsors
sponsors-cta = Devino sponsor

## Gallery

## Footer
footer-license = Licență EUPL v1.2 | Gratuit și open source
footer-github = GitHub
footer-releases = Versiuni
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz: player de muzică gratuit, open source
og-title = Kopuz: player de muzică gratuit, open source
og-image-alt = Player de muzică Kopuz
twitter-title = Kopuz: player de muzică gratuit, open source
## User-friendly aliases (synced with EN updates)
perf-parallel-desc = { perf-parallel-desc-1 } { perf-parallel-desc-2 }
perf-io-desc = { perf-io-desc-1 } { perf-io-desc-2 }
perf-http-desc = { perf-http-desc-1 } { perf-http-desc-2 }
perf-sort-desc = { perf-sort-desc-1 } { perf-sort-desc-2 }

## Deep link (/j)
join-title = Se deschide Kopuz…
join-opening = Se deschide Kopuz…
join-fallback = Kopuz nu s-a deschis. Încă nu îl ai?
join-no-payload = Acest link nu are nicio coadă atașată.
join-download = Descarcă Kopuz

## Privacy policy
footer-privacy = Confidențialitate
privacy-english-note = Această politică este menținută în limba engleză pentru ca formularea ei să rămână exactă. Traducerile restului site-ului nu îi schimbă conținutul.
privacy-back = Înapoi la pagina principală

## Added strings
hero-title-1 = Un singur player pentru
hero-title-2 = toată muzica ta.
hero-desc = Kopuz adună într-o singură bibliotecă fișierele locale, NAS-ul, propriul server și conturile de streaming. Este gratuit și open source.
features-source-spotify = Spotify
feat-local-title = Foldere și servere
feat-local-desc = Adaugă foldere locale sau conectează un server Jellyfin ori Subsonic, de exemplu Navidrome. Piesele din fiecare sursă adăugată apar într-o singură bibliotecă, iar Subsonic îți păstrează coada de redare sincronizată între clienți.
feat-theming-desc = Alege una dintre cele 24 de teme luminoase și întunecate încorporate sau creează-ți propria temă și setează tu fiecare culoare.
feat-native-desc = Controlează redarea de la tastele media și comenzile sistemului: MPRIS pe Linux, Now Playing pe macOS și suprapunerea media pe Windows.
feat-fav-desc = Păstrează favoritele pe dispozitiv sau sincronizează-le cu serverul tău. Creează liste de redare, adaugă albume întregi dintr-o dată și sincronizează listele cu serverul.
feat-i18n-desc = Folosește Kopuz în oricare dintre cele 28 de limbi.
feat-ytdlp-title = Descărcare de pe YouTube
feat-ytdlp-desc = Lipește un link YouTube sau YouTube Music către o melodie, un album sau o listă de redare. Kopuz o descarcă cu propriul client YouTube, o etichetează cu titlu, artist, album și copertă și o organizează pe albume. Cea mai bună calitate audio păstrează fluxul original și nu necesită nimic instalat. Pentru conversie în MP3, FLAC sau WAV, instalează ffmpeg.
feat-crossfade-title = Crossfade și redare fără pauze
feat-crossfade-desc = Treci lin de la o piesă la alta în redarea pe desktop sau dezactivează crossfade pentru albume fără pauze.
feat-youtube-desc = Răsfoiește, caută și pornește radio de mix de la orice piesă. Discover sugerează melodii, albume, artiști și stări, iar paginile artiștilor arată cele mai ascultate melodii, lansările și artiștii similari. Autentifică-te pentru bibliotecă, Liked Music și listele de redare sau răsfoiește fără cont.
feat-spotify-title = Spotify
feat-spotify-desc = Răsfoiește piesele salvate, albumele, listele de redare și Discover, caută și fă scrobble. Redă prin Web Playback SDK de la Spotify sau pe oricare dintre dispozitivele tale Spotify Connect. Ai nevoie de Spotify Premium pentru a reda muzică.
new-title = Noutăți în Kopuz
install-cargo-title = Cargo (crates.io)
install-cargo-desc = Instalează ultima versiune cu Cargo:
install-macos-homebrew = Instalează versiunea pentru Apple silicon din Homebrew tap:
install-android-title = Android
install-android-desc = Instalează Kopuz din Google Play sau descarcă APK-ul semnat arm64-v8a de pe GitHub Releases. Ai nevoie de Android 7.0 (API 24) sau mai nou.
install-android-note = Dacă APK-ul instalat provine din altă sursă, dezinstalează-l înainte de a instala acesta.
ytmusic-token-desc = Poți reda fără autentificare și nu trebuie să instalezi un program ajutător.
ytmusic-signin-note = Kopuz folosește browserul tău implicit. Poți alege în schimb Firefox, Chrome, Chromium, Brave, Edge, Vivaldi, Helium, LibreWolf, Zen sau Floorp.
ytmusic-premium-title = Calitate Premium
ytmusic-premium-desc = Cu un cont Music Premium, Kopuz redă și descarcă fluxurile de calitate mai bună prin propriul client YouTube. Modul anonim nu poate reda piesele disponibile doar pentru Premium.
soundcloud-signin-desc = Nu ai nevoie de URL, parolă sau token. Kopuz deschide autentificarea SoundCloud într-un profil de browser separat și șterge acel profil când elimini sursa.
spotify-guide-subtitle = Kopuz folosește Web API oficial de la Spotify și redarea în browser. Nu îți vede niciodată parola și nu retransmite sunetul.
spotify-step-1-title = Creează o aplicație Spotify
spotify-step-1-desc = În panoul pentru dezvoltatori Spotify, creează o aplicație și activează Web API și Web Playback SDK. La User Management, adaugă fiecare cont care va asculta (până la cinci în Development Mode). Apoi setează exact acest redirect URI:
spotify-step-2-title = Adaugă Client ID
spotify-step-2-desc = În Setări → Media servers → Add → Spotify, lipește Client ID și aprobă accesul. Nu ai nevoie de Client Secret. Portul 8898 trebuie să fie liber cât timp te autentifici.
spotify-step-3-title = Alege unde se redă
spotify-step-3-desc = Ține deschisă fila playerului din aplicație sau alege unul dintre dispozitivele tale Spotify Connect din meniul de dispozitive.
spotify-requirement = Ai nevoie de Spotify Premium pentru a reda muzică. Folosește Chrome, Edge, Brave, Chromium, Vivaldi, Helium sau Safari pe macOS. Firefox nu funcționează.
spotify-full-guide = Citește ghidul complet de configurare și depanare Spotify
privacy-local-title = O singură bază de date locală
privacy-local-desc = Kopuz păstrează biblioteca, listele de redare, favoritele, numărul de redări și datele surselor în kopuz.db, iar preferințele tale lângă ea, în settings.toml.
privacy-accounts-title = Autentificări separate
privacy-accounts-desc = Fiecare sursă își păstrează propria autentificare și propriile favorite. Autentificările din browser folosesc profiluri separate, nu sesiunea ta obișnuită de browser.
privacy-files-title = Cache și jurnale
privacy-files-desc = Coperțile de album și piesele offline se salvează în folderul de cache al sistemului. Poți deschide sau exporta jurnalele din Setări.
requirements-title = Cerințe și limitări
about-title = Ce este un kopuz?
about-desc-1 = Kopuzul este un vechi instrument turcic cu coarde, la care cântau bardzii și șamanii. Se consideră strămoșul mai multor lăute din Asia Centrală.
about-desc-2 = În legenda turcică, aparține lui Dede Korkut, un bard. Komuzul kârgâz și kobyzul kazah sunt instrumente înrudite. Xomus, în ciuda numelui asemănător, nu este.
community-issues-title = Issues
community-issues-desc = Raportează o eroare sau preia o sarcină deschisă.
community-discussions-title = Discuții
community-discussions-desc = Pune întrebări și propune modificări.
community-discord-desc = Discută cu utilizatorii și contribuitorii.
community-contribute-title = Contribuie
community-contribute-desc = Obține codul sursă, configurează un mediu de dezvoltare sau ajută la traducere.
platforms-android = Android
platforms-android-note = ARM64 · Android 7.0+ · API 24+
home-meta-desc = Kopuz este un player de muzică gratuit pentru desktop și Android. Redă fișiere locale, partajări SMB, Jellyfin, Subsonic, YouTube Music, Spotify și altele într-o singură bibliotecă.
home-meta-keywords = Kopuz, player de muzică, Jellyfin, Navidrome, Subsonic, SMB, NAS, Clippsly, Nextcloud, YouTube Music, Apple Music, SoundCloud, Spotify, open source, Android, Linux, macOS, Windows, versuri, egalizator
og-desc = Un player de muzică gratuit pentru fișiere locale, servere și servicii de streaming pe desktop și Android, cu versuri sincronizate, teme și egalizator cu 10 benzi.
twitter-desc = Redă fișiere locale, partajări SMB, Jellyfin, Subsonic, YouTube Music, SoundCloud, Spotify și altele într-o singură bibliotecă.
nav-home = Acasă
footer-license-line = EUPL 1.2, gratuit și open source
shelf-link-to-us = Trimite spre noi cu un buton 88×31
hero-release = Ultima versiune
hero-cta-download-for = Descarcă pentru
hero-cta-all = Toate platformele
home-sources-title = Redă muzică din
source-local = Foldere locale
source-navidrome = Navidrome și Subsonic
source-radio = Radio pe internet
moment-lyrics-title = Versuri care țin ritmul
moment-lyrics-alt = Kopuz redând o piesă cu versuri sincronizate pe tot ecranul
moment-library-title = O singură bibliotecă pentru fiecare sursă
moment-library-alt = Biblioteca Kopuz cu albume din mai multe surse
moment-themes-title = Culorile tale sau ale fundalului tău
moment-themes-alt = Editorul de teme Kopuz
moment-themes-extra = Sau lasă Kopuz să creeze o temă din fundalul desktopului tău.
theme-default = Gruvbox dark soft
theme-amoled = AMOLED
theme-wallpaper = Fundal
wallpaper-sunset = Apus
wallpaper-lake = Lac
home-see-all-features = Vezi toate funcțiile
home-get-title = Instalează Kopuz
home-get-download = Descarcă
home-also-on = Poți instala Kopuz și cu Cargo, Nix, AUR, Flatpak sau Homebrew.
home-sponsors-title = Sponsori
home-sponsor-cta = Sponsorizează pe GitHub
copy = Copiază
copied = Copiat
copy-address = Copiază adresa { $coin }
nav-primary-aria = Navigare principală
nav-menu-open = Deschide meniul
nav-menu-close = Închide meniul
footer-nav-aria = Subsol
theme-use-light = Folosește tema luminoasă
theme-use-dark = Folosește tema întunecată
theme-leave-moe = Ieși din modul moe
mode-simple = Site simplu
mode-full = Site complet
webbutton-alt = Buton web Kopuz 88x31
features-page-title = Funcții Kopuz: surse, versuri, teme și redare
download-page-title = Descarcă Kopuz pentru Windows, macOS, Linux și Android
guides-page-title = Ghiduri de configurare Kopuz pentru servere și streaming
support-page-title = Susține Kopuz: sponsori, donații și comunitate
notfound-page-title = Pagina nu a fost găsită | Kopuz
guide-title-suffix = ghid de configurare | Kopuz
notfound-title = Pagina nu a fost găsită.
notfound-desc = Nu se redă nimic la această adresă.
notfound-home = Mergi la pagina principală
features-group-sources = Surse
features-group-playback = Redare
features-group-library = Bibliotecă
features-group-desktop = Desktop
features-group-android = Android
features-group-integrations = Integrări
features-group-performance = Performanță
features-group-privacy = Confidențialitate
features-jump-aria = Grupuri de funcții
features-privacy-link = Citește politica de confidențialitate
install-other-title = Alte moduri de instalare
req-spotify = Ai nevoie de Premium, de propriul Client ID și de un browser compatibil pentru a reda muzică.
req-applemusic = Redarea pe desktop necesită un Widevine CDM. Te poți autentifica pe Android, dar redarea nu funcționează încă acolo.
req-nextcloud = WebDAV simplu nu acceptă liste de redare sau radio. Dacă Nextcloud Music oferă un endpoint Subsonic, folosește-l pe acela.
req-android = APK-urile de versiune necesită un telefon arm64-v8a cu Android 7.0 (API 24) sau mai nou. Starea Discord și system tray nu sunt disponibile.
req-appimage = Necesită webkit2gtk-4.1 și GTK 3. Pentru tray este nevoie și de o bibliotecă appindicator.
req-ytmusic = Modul anonim nu poate reda piesele disponibile doar pentru Premium. Autentifică-te cu un cont Music Premium ca să le redai.
req-crossfade = Crossfade și redarea fără pauze funcționează cu redarea nativă pe desktop, nu cu sunetul Spotify redat în browser.
req-spotify-limits-label = Limite Spotify
req-spotify-limits = În Development Mode, o aplicație permite până la cinci ascultători, căutarea returnează până la zece rezultate pe tip, listele de redare sunt doar pentru citire, iar descărcările, radioul, editarea etichetelor, egalizatorul și crossfade sunt dezactivate.
support-note = Sunt Temidaradev și construiesc Kopuz. Sunt student și nu am un venit stabil, așa că sponsorii mă ajută să continui să lucrez la Kopuz cât timp studiez.
support-goal-label = Obiectiv GitHub Sponsors
support-goal-amount = ${ $current } din ${ $goal } pe lună
support-goal-aria = Progresul obiectivului GitHub Sponsors
support-crypto-summary = Adrese crypto
sponsors-tier-special = Special
sponsors-tier-monthly = Lunar
sponsors-tier-one-time = O singură dată
features-source-smb = SMB și NAS
features-source-clippsly = Clippsly
feat-smb-title = SMB și NAS
feat-smb-desc = Adaugă ca bibliotecă o partajare SMB 2 sau 3 ori un folder din ea. Kopuz citește etichetele încorporate și coperțile și redă direct din partajare, cu derulare în piesă, așa că nu trebuie s-o montezi.
feat-clippsly-title = Clippsly
feat-clippsly-desc = Autentifică-te cu numele tău de utilizator Clippsly și o parolă de aplicație de pe clippsly.app. Redă și descarcă din locker, încarcă fișiere audio și vezi cât spațiu de stocare folosești. Listele de redare de la distanță nu sunt acceptate încă.
feat-updates-title = Actualizări automate
feat-updates-desc = Pe desktop, un banner te anunță când apare o versiune nouă, iar Kopuz o descarcă și o instalează pentru tine.
feat-cli-title = Daemon și linie de comandă
feat-cli-desc = Playerul rulează ca daemon, kopuzd, la care se conectează fereastra. Folosește kopuzctl pentru a controla redarea, a configura surse și a porni scanări ale bibliotecii dintr-un terminal sau dintr-un script, sau vorbește direct cu API-ul gRPC.
install-android-play = Instalează din Google Play
footer-made-with = Site creat cu
footer-love = dragoste
footer-made-by = de
source-smb = SMB și NAS
source-clippsly = Clippsly
player-prev-label = Secțiunea anterioară
player-next-label = Secțiunea următoare
player-seek-label = Poziția în pagină
player-queue-label = Secțiunile din această pagină
player-queue-title = În această pagină
dl-file-exe = Program de instalare (.exe)
dl-file-msi = Program de instalare (.msi)
dl-file-dmg = Imagine de disc (.dmg)
dl-file-appimage = AppImage
dl-file-deb = Debian și Ubuntu (.deb)
dl-file-rpm = Fedora și RHEL (.rpm)
dl-file-apk = APK
dl-release-page = Pagina versiunii
dl-this-device = Acest dispozitiv
dl-dialog-ready = Ultima versiune: { $version }. Alege programul de instalare pentru dispozitivul tău.
dl-dialog-close = Închide
dl-dialog-release = Vezi versiunea pe GitHub
guide-nextcloud-summary = Conectează-te prin WebDAV cu o parolă de aplicație, apoi alege folderele bibliotecii.
guide-ytmusic-summary = Autentifică-te prin browser sau ascultă fără cont.
guide-applemusic-summary = Autentifică-te, configurează Widevine pe desktop și vezi ce funcționează pe Android.
guide-soundcloud-summary = Autentifică-te o dată în browser și vezi ce acceptă sursa.
guide-spotify-summary = Creează o aplicație de dezvoltator, adaugă Client ID-ul ei și alege unde se redă.
chapter-home = Acasă
chapter-sources = Surse
chapter-lyrics = Versuri
chapter-library = Bibliotecă
chapter-themes = Teme
chapter-download = Descarcă
chapter-sponsors = Sponsori
chapter-whats-new = Noutăți
chapter-name = Numele
chapter-not-found = Negăsit
chapter-privacy = Confidențialitate
chapter-install = Instalare
chapter-requirements = Cerințe
chapter-support = Suport
chapter-community = Comunitate
chapter-features = Funcții
chapter-guides = Ghiduri
guide-smb-summary = Îndreaptă Kopuz spre o partajare de pe NAS și autentifică-te cu contul NAS.
smb-subtitle = Alege SMB / NAS în selectorul de surse. Nu trebuie să montezi mai întâi partajarea.
smb-share-title = Indică o partajare
smb-share-desc = Introdu URL-ul unei partajări sau al unui folder din ea, de exemplu:
smb-share-port = Dacă NAS-ul folosește alt port decât cel implicit, adaugă-l la URL:
smb-signin-title = Autentifică-te cu contul NAS
smb-signin-desc = Folosește numele de utilizator și parola NAS. Pentru un cont de domeniu, introdu DOMAIN\username. Kopuz are nevoie de SMB 2 sau 3 și de un cont: nu caută partajări în rețea și nu se autentifică ca invitat.
smb-library-title = Cum funcționează biblioteca
smb-library-desc = Kopuz scanează partajarea și subfolderele ei după fișiere audio și citește etichetele încorporate și coperțile. Redă direct din partajare, cu derulare în piesă, inclusiv pe Android. Favoritele tale se păstrează în Kopuz.
smb-limits-title = Partajarea rămâne doar pentru citire
smb-limits-desc = Kopuz nu modifică nimic în partajare, deci nu poți edita etichete acolo, nu poți ține liste de redare pe ea și nu poți descărca piesele ei pentru ascultare offline.
guide-clippsly-summary = Autentifică-te cu o parolă de aplicație, redă din locker și încarcă muzică.
clippsly-subtitle = Adaugă Clippsly din setările surselor.
clippsly-signin-title = Autentifică-te cu o parolă de aplicație
clippsly-signin-desc = Creează o parolă de aplicație pe clippsly.app. Apoi autentifică-te în Kopuz cu numele tău de utilizator Clippsly și acea parolă de aplicație.
clippsly-locker-title = Lockerul tău
clippsly-locker-desc = Răsfoiește și redă muzica din lockerul tău Clippsly și descarcă piese pentru ascultare offline.
clippsly-upload-title = Încarcă muzică
clippsly-upload-desc = Încarcă fișiere audio din cardul de setări al sursei. Kopuz îți reîmprospătează biblioteca după ce se termină tot lotul. Cardul arată și cât spațiu de stocare folosești: cota o stabilește Clippsly.
clippsly-bonus = Leagă Kopuz de contul tău Clippsly și Clippsly îți adaugă gratuit 3 GB de stocare.
clippsly-limits-title = Liste de redare
clippsly-limits-desc = Listele de redare pe care le ții în Clippsly nu apar încă în Kopuz.
clippsly-site = Mergi la clippsly.app
