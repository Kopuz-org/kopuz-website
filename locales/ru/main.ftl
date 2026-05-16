## Language
lang-name = Русский

## Nav
nav-new = Новое
nav-announce = Современный UI, тема Vaxry, загрузчик yt-dlp, 20+ языков
nav-features = Возможности
nav-install = Установка
nav-download = Скачать
nav-sponsors = Спонсоры
nav-github = GitHub ↗
nav-gallery = Галерея
nav-lang-label = Язык

## Hero
hero-title-1 = Твоя музыка.
hero-title-2 = Твой стиль.
hero-desc = Kopuz — современный лёгкий музыкальный плеер на Rust и Dioxus. Сканируй локальные папки, стримь с Jellyfin или Navidrome, просматривай библиотеку как удобно.
hero-cta-download = Скачать
hero-cta-github = Открыть на GitHub
hero-screenshot-alt = Kopuz — главный экран

## Features
features-title = Всё, что нужно.
features-chip = Никаких подписок. Никакого слежения. Только твоя музыка.
features-works-with = Работает с
features-source-local = Локальные файлы
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API

feat-local-title = Локально + стриминг
feat-local-desc = Укажи локальную папку или подключись к Jellyfin / Subsonic (Navidrome). Смешивай как хочешь.
feat-theming-title = Темы
feat-theming-desc = Динамические темы с полным контролем переменных цвета. Создай свою с нуля или выбери пресет.
feat-native-title = Нативная интеграция
feat-native-desc = MPRIS на Linux, Now Playing на macOS, System Media Transport на Windows. Полностью подключено.
feat-lyrics-title = Синхронизированный текст
feat-lyrics-desc = Прокручиваемый текст в реальном времени, синхронизированный или обычный, с автопрокруткой.
feat-eq-title = Эквалайзер
feat-eq-desc = 5-полосный эквалайзер со встроенными пресетами и полным контролем звука.
feat-fav-title = Избранное и плейлисты
feat-fav-desc = Помечай треки локально или синхронизируй избранное с сервером. Создавай плейлисты, добавляй целые альбомы.
feat-scrobble-title = Скробблинг
feat-scrobble-desc = Встроенный скробблинг ListenBrainz. Пользователи Jellyfin могут использовать плагин listenbrainz.
feat-discord-title = Discord RPC
feat-discord-desc = Покажи друзьям что ты слушаешь со встроенным Discord Rich Presence. Без настройки.
feat-search-title = Поиск
feat-search-desc = Поиск в реальном времени по исполнителям, альбомам и трекам. Мгновенные результаты.
feat-genre-title = Просмотр по жанрам
feat-genre-desc = Просматривай всю библиотеку по жанрам — работает для локальных файлов и серверной музыки.
feat-logs-title = Логи прослушивания
feat-logs-desc = Счётчики проигрывания локально. Смотри что слушаешь больше всего.
feat-i18n-title = i18n
feat-i18n-desc = Английский, русский, немецкий, французский, испанский, турецкий, украинский, польский, арабский, греческий, иврит, венгерский, индонезийский, японский, корейский, румынский, португальский, китайский и другие.
feat-ytdlp-title = Загрузчик yt-dlp
feat-ytdlp-desc = Скачивай аудио прямо с YouTube. Вывод в MP3, FLAC, WAV, MP4 или лучшем качестве. SponsorBlock, разделение на главы, ограничение скорости.
feat-crossfade-title = Кроссфейд и переходы
feat-crossfade-desc = Плавные переходы между треками. Поддержка кроссфейда в нативных десктоп-сборках.
feat-channels-title = Режимы каналов
feat-channels-desc = Стерео, моно, только левый, только правый и L/R swap. Точное управление каналами.

## Performance
perf-title = Создано быть быстрым.
perf-subtitle = Большие библиотеки. Мгновенный запуск. Без зависаний.
perf-skip-label = Пропуск проиндексированного
perf-skip-desc = Повторное сканирование обрабатывает только новые файлы. 10k треков + 5 новых = читаются только 5.
perf-parallel-label = Параллельный запуск
perf-parallel-desc-1 = Библиотека, конфиг, плейлисты и избранное загружаются параллельно через
perf-parallel-desc-2 = . Почти мгновенное открытие.
perf-art-label = Кэширование обложек
perf-art-desc = Обложки извлекаются один раз, сохраняются на диск. Никакого повторного декодирования.
perf-lazy-label = Ленивая загрузка изображений
perf-lazy-desc = Сотни обложек в результатах поиска — ни одна не загружается пока не попадёт в видимую область.
perf-io-label = Неблокирующий I/O
perf-io-desc-1 = Тяжёлые задачи работают на
perf-io-desc-2 = { " " }потоках. UI остаётся отзывчивым во время полного сканирования.
perf-http-label = HTTP-кэш обложек
perf-http-desc-1 = Кастомный
perf-http-desc-2 = { " " }протокол отдаёт обложки с заголовками кэша на год. Webview не перезапрашивает.

## Install
install-title = Установка
install-nix-title = Nix / NixOS
install-nix-run = Запуск без установки:
install-nix-profile = Или добавь в свой профиль:
install-nix-note = Поддержка NixOS flake с Cachix binary cache.
install-flatpak-title = Flatpak
install-flatpak-desc = Установка из манифеста:
install-flatpak-note = Листинг на Flathub скоро.
install-appimage-title = AppImage
install-appimage-desc-1 = Скачай с GitHub Releases и запусти. Требуется
install-appimage-desc-2 = { " " }и
install-appimage-desc-3 = { " " }в системе.
install-appimage-note-1 = Пользователи Arch: если падает с ошибкой WebKitNetworkProcess, запускай с префиксом
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Скачай
install-macos-desc-2 = { " " }с GitHub Releases. Если macOS блокирует, сними флаг карантина:

## Platforms
platforms-title = Скачать Kopuz
platforms-subtitle = Бесплатно и с открытым кодом. Все релизы на GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Только Apple Silicon
platforms-linux = Linux
platforms-download = Скачать →

## Support
support-title = Поддержать Kopuz
support-subtitle = Kopuz бесплатный и open source. Поддержка держит его на плаву.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — или отправь крипту —
support-usdt-note = (сеть Solana)

## Sponsors
sponsors-title = Спонсоры
sponsors-subtitle = Люди, поддерживающие Kopuz.
sponsors-cta = Стать спонсором

## Gallery
gallery-page-title = Галерея — Kopuz Music Player
gallery-page-desc = Скриншоты Kopuz в действии — главная, библиотека, плейлист, полноэкранный плеер, текст, редактор тем и многое другое.
gallery-title = Галерея
gallery-subtitle = Скриншоты Kopuz — стили Normal и Modern.
gallery-home = Главная
gallery-home-styles = Normal / Modern
gallery-library = Библиотека
gallery-library-styles = Normal / Vaxry
gallery-playlist = Плейлист
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Полноэкранный плеер
gallery-fullscreen-desc = Иммерсивный полноэкранный режим воспроизведения.
gallery-fullscreen-lyrics-title = Полноэкранный текст
gallery-fullscreen-lyrics-desc = Синхронизированный текст в полноэкранном режиме.
gallery-search-title = Поиск
gallery-search-desc = Поиск в реальном времени по исполнителям, альбомам и трекам.
gallery-theme-title = Редактор тем
gallery-theme-desc = Полный контроль переменных цвета. Создай или выбери пресет.
gallery-settings-title = Настройки плеера
gallery-settings-desc = Настрой звук, поведение, интеграции и многое другое.
gallery-downloader-title = Загрузчик
gallery-downloader-desc = Скачивай треки прямо из Kopuz.
gallery-prev = ← Назад
gallery-next = Вперёд →
gallery-label-normal-home = Normal — Главная
gallery-label-modern-home = Modern — Главная
gallery-label-normal-library = Normal — Библиотека
gallery-label-vaxry-library = Vaxry — Библиотека
gallery-label-normal-playlist = Normal — Плейлист
gallery-label-modern-playlist = Modern — Плейлист
gallery-label-fullscreen = Полноэкранный плеер
gallery-label-fullscreen-lyrics = Полноэкранный текст
gallery-label-search = Поиск
gallery-label-theme-editor = Редактор тем
gallery-label-player-settings = Настройки плеера
gallery-label-downloader = Загрузчик

## Footer
footer-license = MIT License — Бесплатно и Open Source
footer-github = GitHub
footer-releases = Релизы
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz — современный лёгкий музыкальный плеер на Rust и Dioxus. Стрим с Jellyfin или Navidrome, локальные файлы, синхронизированный текст, эквалайзер, темы и многое другое.
home-meta-keywords = Kopuz, музыкальный плеер, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, текст песен
og-title = Kopuz — Music Player
og-desc = Современный лёгкий музыкальный плеер на Rust. Локальные файлы, Jellyfin, Navidrome, синхронизированный текст, эквалайзер, Discord RPC и многое другое. Бесплатно и open source.
og-image-alt = Музыкальный плеер Kopuz
twitter-title = Kopuz — Music Player
twitter-desc = Современный лёгкий музыкальный плеер на Rust. Бесплатно и open source.
