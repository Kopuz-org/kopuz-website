## Language
lang-name = Bahasa Indonesia

## Nav
nav-new = Baru
nav-announce = Streaming YouTube Music, Discover, radio mix, 20+ bahasa
nav-features = Fitur
nav-install = Pasang
nav-download = Unduh
nav-sponsors = Sponsor
nav-github = GitHub ↗
nav-gallery = Galeri
nav-lang-label = Bahasa

## Hero
hero-title-1 = Musikmu.
hero-title-2 = Caramu.
hero-desc = Kopuz adalah pemutar musik modern dan ringan yang dibangun dengan Rust dan Dioxus. Pindai folder lokal, streaming dari Jellyfin atau Navidrome, dan jelajahi pustaka semaumu.
hero-cta-download = Unduh
hero-cta-github = Lihat di GitHub
hero-screenshot-alt = Kopuz — tampilan beranda

## Features
features-title = Semua yang kamu butuhkan.
features-chip = Tanpa langganan. Tanpa pelacakan. Hanya musikmu.
features-works-with = Bekerja dengan
features-source-local = Berkas lokal
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-ytmusic = YouTube Music
features-source-soundcloud = SoundCloud

feat-local-title = Lokal + Streaming
feat-local-desc = Arahkan ke folder lokal atau sambungkan ke Jellyfin / Subsonic (Navidrome). Campur sesukamu.
feat-theming-title = Tema
feat-theming-desc = Tema dinamis dengan kontrol penuh atas variabel warna. Bangun temamu dari nol atau pilih preset.
feat-native-title = Integrasi Native
feat-native-desc = MPRIS di Linux, Now Playing di macOS, System Media Transport di Windows. Tersambung penuh.
feat-lyrics-title = Lirik Tersinkron
feat-lyrics-desc = Lirik bergulir realtime, tersinkron atau biasa, gulir-otomatis mengikuti musik.
feat-eq-title = Equalizer
feat-eq-desc = Equalizer 5-band dengan preset bawaan dan kontrol penuh atas suaramu.
feat-fav-title = Favorit & Daftar Putar
feat-fav-desc = Tandai lagu secara lokal atau sinkronkan favorit dengan servermu. Buat daftar putar, tambah seluruh album sekaligus.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobbling ListenBrainz terintegrasi. Pengguna Jellyfin juga dapat menggunakan plugin listenbrainz.
feat-discord-title = Discord RPC
feat-discord-desc = Tunjukkan teman apa yang sedang kamu dengar dengan Discord Rich Presence tertanam. Tanpa pengaturan.
feat-search-title = Pencarian
feat-search-desc = Pencarian realtime di artis, album, dan lagu. Hasil instan saat kamu mengetik.
feat-genre-title = Telusur Genre
feat-genre-desc = Telusuri seluruh pustakamu berdasarkan genre — bekerja untuk berkas lokal dan musik server.
feat-logs-title = Catatan Dengar
feat-logs-desc = Hitungan pemutaran dilacak lokal. Lihat apa yang sebenarnya paling sering kamu dengarkan.
feat-i18n-title = i18n
feat-i18n-desc = Inggris, Rusia, Jerman, Prancis, Spanyol, Turki, Ukraina, Polandia, Arab, Yunani, Ibrani, Hungaria, Indonesia, Jepang, Korea, Rumania, Portugis, Tionghoa, dan lainnya.
feat-ytdlp-title = Pengunduh yt-dlp
feat-ytdlp-desc = Unduh audio langsung dari YouTube. Keluaran MP3, FLAC, WAV, MP4, atau kualitas terbaik. SponsorBlock, pemisahan bab, dan pembatasan kecepatan termasuk.
feat-crossfade-title = Crossfade & Transisi
feat-crossfade-desc = Padukan transisi lagu untuk pemutaran lebih halus. Dukungan crossfade pada build desktop native.
feat-channels-title = Mode Kanal
feat-channels-desc = Stereo, Mono, Hanya-Kiri, Hanya-Kanan, dan tukar L/R. Kontrol kanal audio yang detail.

feat-youtube-title = YouTube Music
feat-youtube-desc = Backend streaming penuh dengan halaman Discover ala Spotify, profil artis lengkap, dan radio mix. Masuk untuk pustaka, Liked Music, dan daftar putarmu — atau jelajahi secara anonim.
feat-metadata-title = Gambar Artis
feat-metadata-desc = Pilih bagaimana gambar artis diambil: sampul album pertama (bawaan) atau foto artis asli yang diambil dari server Jellyfin atau Subsonic-mu, dengan fallback otomatis.
feat-debug-title = Catatan & Laporan Crash
feat-debug-desc = Buka atau ekspor catatan langsung dari Pengaturan, dengan trace performa opsional yang dapat kamu buka di Speedscope atau Perfetto. Crash menulis laporan secara otomatis.
feat-cleanup-title = Pembersihan Otomatis
feat-cleanup-desc = Lagu yang hilang atau terhapus dibuang dari pustakamu secara otomatis saat pemindaian ulang. Tanpa entri hantu.
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = Streaming dari SoundCloud setelah sekali masuk lewat browser. Pencarian, pemutaran progressive MP3 dan Go+ AAC/HLS, lagu Liked-mu sebagai favorit, dan daftar putar hanya-baca.
feat-miniplayer-title = Mini-Player
feat-miniplayer-desc = Overlay now-playing ringkas yang dapat kamu aktifkan dari bilah bawah untuk tampilan lebih kecil.
feat-tray-title = Kecilkan ke Tray
feat-tray-desc = Tutup ke ikon system tray alih-alih keluar, sehingga pemutaran tetap berjalan di latar belakang. Aktifkan di Pengaturan.
feat-badges-title = Lencana Tipe Berkas
feat-badges-desc = Lagu lokal menampilkan lencana format kecil — MP3, FLAC, WAV, dan lainnya — langsung di baris lagu.

## Performance
perf-title = Dibangun untuk cepat.
perf-subtitle = Pustaka besar. Mulai instan. Tanpa lag.
perf-skip-label = Lewati yang sudah terindeks
perf-skip-desc = Pemindaian ulang hanya memproses berkas baru. 10k lagu + 5 baru = hanya 5 yang dibaca.
perf-parallel-label = Mulai paralel
perf-parallel-desc-1 = Pustaka, config, daftar putar, dan favorit semua dimuat paralel dengan
perf-parallel-desc-2 = . Pembukaan hampir instan.
perf-art-label = Cache sampul album
perf-art-desc = Sampul diekstrak sekali, disimpan ke disk. Tidak pernah didekode ulang pada tampilan berulang.
perf-lazy-label = Pemuatan gambar lazy
perf-lazy-desc = Ratusan sampul album di hasil pencarian — tidak ada yang dimuat sampai terlihat.
perf-io-label = I/O non-blocking
perf-io-desc-1 = Pekerjaan berat berjalan di thread
perf-io-desc-2 = { " " }. UI tetap responsif selama pemindaian pustaka penuh.
perf-http-label = Cache HTTP sampul
perf-http-desc-1 = Protokol
perf-http-desc-2 = { " " }kustom menyajikan sampul dengan header cache 1-tahun. Webview tidak pernah mengambil ulang.
perf-sort-label = Pengurutan lebih cerdas
perf-sort-desc-1 = Tampilan pustaka diurutkan dengan
perf-sort-desc-2 = { " " }sehingga kunci urut dihitung sekali, bukan pada setiap perbandingan.

## Install
install-title = Pemasangan
install-nix-title = Nix / NixOS
install-nix-run = Jalankan tanpa memasang:
install-nix-profile = Atau tambahkan ke profilmu:
install-nix-note = NixOS flake didukung dengan Cachix binary cache.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Pasang dengan helper pilihanmu:
install-aur-note-1 = Membutuhkan
install-aur-note-2 = { " " }terpasang lebih dulu pada versi yang cocok dengan dioxus 0.7.x.
install-flatpak-title = Flatpak
install-flatpak-desc = Pasang dari manifest sumber:
install-flatpak-note = Listing Flathub segera.
install-appimage-title = AppImage
install-appimage-desc-1 = Unduh dari GitHub Releases dan jalankan langsung. Membutuhkan
install-appimage-desc-2 = { " " }dan
install-appimage-desc-3 = { " " }di sistemmu.
install-appimage-note-1 = Pengguna Arch: jika crash dengan error WebKitNetworkProcess, jalankan dengan awalan
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Unduh
install-macos-desc-2 = { " " }dari GitHub Releases. Jika macOS memblokir, bersihkan tanda karantina:

## Platforms
platforms-title = Unduh Kopuz
platforms-subtitle = Gratis dan open source. Semua rilis di GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Hanya Apple Silicon
platforms-linux = Linux
platforms-download = Unduh →

## Support
support-title = Dukung Kopuz
support-subtitle = Kopuz gratis dan open source. Dukungan menjaganya tetap hidup.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — atau kirim kripto —
support-usdt-note = (jaringan Solana)

donate-title = Donasi Kripto
donate-subtitle = Dukung pengembangan dengan mengirim kripto ke alamat-alamat ini.
donate-solana = Solana: 2fapJYRztnTRLpJbmyEUnsuZ36AzLK2JrMmmLEfDqKpN
donate-bitcoin = Bitcoin: bc1qz94yz9xvufa6hxlvjzaajgd2zyfu86arn68hu4
donate-monero = Monero: 86mz3HxTrKyYpuvx78m6pufbXdwAnoyoZBztz6HyYrnM1XP5YVrMy9jTVRY5vzgGtkizACLpFwHEdafKTMoj6y8mAVgvWMz
donate-ethereum = Ethereum: 0xa490D50470cdFf837B6663F7f6cBe50B157224e5
donate-usdt-sol = USDT (Solana): GYmnAcrA5MbF6cUxT2m5d5cwdfr14qSY9WFYRwXxaibW

## YouTube Music
ytmusic-title = Penyiapan YouTube Music
ytmusic-subtitle = Tambahkan dari Pengaturan → Server media → Tambah → YouTube Music.
ytmusic-token-title = Tanpa helper
ytmusic-token-desc-1 = Pemutaran anonim membutuhkan PO token konten, yang kini dibuat Kopuz di dalam aplikasi dengan WebView tersembunyi yang menjalankan BotGuard milik YouTube. Subprocess
ytmusic-token-desc-2 = { " " }yang lama sudah hilang — tidak ada yang perlu dipasang, dan ini bekerja di dalam Flatpak.
ytmusic-signin-title = Masuk dengan browser
ytmusic-signin-desc = Kopuz membuka sign-in Google di profil browser terisolasi — penjelajahan normalmu tidak pernah tersentuh — dan mengekstrak cookie sesi. Membuka pustaka, Liked Music, daftar putar, dan artis yang kamu ikuti.
ytmusic-signin-note = Di Windows, sign-in lewat browser saat ini dinonaktifkan; pengguna Windows otomatis mendapat mode anonim. Sign-in bekerja di Linux dan macOS.
ytmusic-anon-title = Mode anonim
ytmusic-anon-desc = Tanpa sign-in, tanpa cookie. Jelajahi, cari, buka halaman artis, album, dan daftar putar, mulai radio mix, dan putar lagu publik. Menyukai dan tampilan pustaka dinonaktifkan.
ytmusic-premium-title = Lagu Premium
ytmusic-premium-desc-1 = Lagu yang terkunci Music Premium jatuh ke resolve
ytmusic-premium-desc-2 = { " " }lokal saat jalur utama mengembalikan UNPLAYABLE, jadi memasangnya membantu. Mode anonim sama sekali tidak dapat memutar konten khusus Premium.

## SoundCloud
soundcloud-title = Penyiapan SoundCloud
soundcloud-subtitle = Tambahkan dari Pengaturan → Server media → Tambah → SoundCloud.
soundcloud-signin-title = Sekali masuk lewat browser
soundcloud-signin-desc = Tanpa URL atau kata sandi untuk diketik. Kopuz membuka soundcloud.com/signin di profil browser terisolasi — penjelajahan normalmu tidak pernah tersentuh — dan menarik oauth_token sesi. Pilih browser keluarga Chromium yang akan dipakai (Chrome, Chromium, Brave, Edge, atau Vivaldi).
soundcloud-features-title = Apa yang kamu dapat
soundcloud-features-desc = Pencarian, pemutaran lagu (progressive MP3 plus stream Go+ AAC/HLS), lagu Liked-mu sebagai favorit, daftar putar hanya-baca, dan suka/batal suka. Menghapus sumber akan membersihkan profil terisolasinya.

## Sponsors
sponsors-title = Sponsor
sponsors-subtitle = Orang-orang yang membuat Kopuz tetap berjalan.
sponsors-cta = Jadi Sponsor

## Gallery
gallery-page-title = Galeri — Kopuz Music Player
gallery-page-desc = Tangkapan layar Kopuz beraksi — beranda, pustaka, daftar putar, pemutar layar penuh, lirik, editor tema, dan lainnya.
gallery-title = Galeri
gallery-subtitle = Tangkapan layar Kopuz — gaya Normal dan Modern.
gallery-home = Beranda
gallery-home-styles = Normal / Modern
gallery-library = Pustaka
gallery-library-styles = Normal / Vaxry
gallery-playlist = Daftar Putar
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Pemutar layar penuh
gallery-fullscreen-desc = Tampilan pemutaran imersif jendela penuh.
gallery-fullscreen-lyrics-title = Lirik layar penuh
gallery-fullscreen-lyrics-desc = Lirik tersinkron di mode jendela penuh.
gallery-search-title = Pencarian
gallery-search-desc = Pencarian realtime di artis, album, dan lagu.
gallery-theme-title = Editor tema
gallery-theme-desc = Kontrol penuh variabel warna. Bangun atau pilih preset.
gallery-settings-title = Pengaturan pemutar
gallery-settings-desc = Atur audio, perilaku, integrasi, dan lainnya.
gallery-downloader-title = Pengunduh
gallery-downloader-desc = Unduh lagu langsung dari dalam Kopuz.
gallery-prev = ← Sebelumnya
gallery-next = Berikutnya →
gallery-label-normal-home = Normal — Beranda
gallery-label-modern-home = Modern — Beranda
gallery-label-normal-library = Normal — Pustaka
gallery-label-vaxry-library = Vaxry — Pustaka
gallery-label-normal-playlist = Normal — Daftar Putar
gallery-label-modern-playlist = Modern — Daftar Putar
gallery-label-fullscreen = Pemutar layar penuh
gallery-label-fullscreen-lyrics = Lirik layar penuh
gallery-label-search = Pencarian
gallery-label-theme-editor = Editor tema
gallery-label-player-settings = Pengaturan pemutar
gallery-label-downloader = Pengunduh

## Footer
footer-license = Lisensi MIT — Gratis & Open Source
footer-github = GitHub
footer-releases = Rilis
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz adalah pemutar musik modern dan ringan yang dibangun dengan Rust dan Dioxus. Streaming dari Jellyfin atau Navidrome, jelajahi berkas lokal, lirik tersinkron, equalizer, tema, dan lainnya.
home-meta-keywords = Kopuz, pemutar musik, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, lirik
og-title = Kopuz — Music Player
og-desc = Pemutar musik modern dan ringan yang dibangun dengan Rust. Berkas lokal, Jellyfin, Navidrome, lirik tersinkron, equalizer, Discord RPC, dan lainnya. Gratis dan open source.
og-image-alt = Pemutar musik Kopuz
twitter-title = Kopuz — Music Player
twitter-desc = Pemutar musik modern dan ringan yang dibangun dengan Rust. Gratis dan open source.

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
