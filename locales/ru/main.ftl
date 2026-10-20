## Language

## Nav
nav-features = Возможности
nav-download = Скачать
nav-lang-label = Язык

## Hero
hero-cta-download = Скачать
hero-cta-github = Открыть на GitHub
hero-screenshot-alt = Главный экран Kopuz

## Features
features-title = Возможности
features-source-local = Локальные файлы
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-nextcloud = Nextcloud
features-source-ytmusic = YouTube Music
features-source-applemusic = Apple Music
features-source-soundcloud = SoundCloud

feat-theming-title = Темы
feat-native-title = Нативная интеграция
feat-android-title = Android
feat-android-desc = ARM64-сборка для смартфонов с жестами смахивания, кнопками перемешивания и повтора в медиауведомлении и AMOLED-режимом для Android 7.0 и новее.
feat-lyrics-title = Синхронизированный текст
feat-lyrics-desc = Синхронизированные построчно и пословно, а также обычные тексты песен с инструментальными проигрышами, активной строкой по центру, эффектом глубины через размытие и ручным или автоматическим смещением синхронизации.
feat-eq-title = Эквалайзер
feat-eq-desc = 5-полосный эквалайзер со встроенными пресетами и полным контролем звука.
feat-fav-title = Избранное и плейлисты
feat-scrobble-title = Скробблинг
feat-scrobble-desc = Встроенный скробблинг ListenBrainz. Пользователи Jellyfin могут использовать плагин listenbrainz.
feat-radio-title = Интернет-радио
feat-radio-desc = Слушай интернет-радиостанции из каталогов с метаданными потока и текущего трека.
feat-offline-title = Офлайн-загрузки
feat-offline-desc = Скачивай доступные для загрузки треки из удалённых источников и слушай их без подключения к сети.
feat-fonts-title = Пользовательские шрифты интерфейса
feat-fonts-desc = Выбирай собственный шрифт интерфейса в дополнение к встроенным темам.
feat-discord-title = Discord RPC
feat-discord-desc = Покажи друзьям что ты слушаешь со встроенным Discord Rich Presence. Без настройки.
feat-search-title = Поиск
feat-search-desc = Поиск в реальном времени по исполнителям, альбомам и трекам. Мгновенные результаты.
feat-genre-title = Просмотр по жанрам
feat-genre-desc = Просматривай локальную и серверную музыку по жанрам.
feat-logs-title = Логи прослушивания
feat-logs-desc = Счётчики проигрывания локально. Смотри что слушаешь больше всего.
feat-i18n-title = i18n
feat-channels-title = Режимы каналов
feat-channels-desc = Стерео, моно, только левый, только правый и L/R swap. Точное управление каналами.
feat-youtube-title = YouTube Music
feat-applemusic-title = Apple Music
feat-applemusic-desc = Просматривай каталог и свою медиатеку, синхронизируй избранное, добавляй треки в плейлисты и удаляй их, запускай радио, скачивай треки и используй тексты с пословной синхронизацией. В настольной версии для воспроизведения требуется Widevine.
feat-nextcloud-title = Nextcloud
feat-nextcloud-desc = Подключайся по WebDAV, выбирай папки медиатеки отдельно для каждого сервера, слушай или скачивай треки. Kopuz кэширует обложки и определяет длительность по заголовкам файлов.
feat-metadata-title = Изображения исполнителей
feat-metadata-desc = Выбери источник изображений исполнителей: обложка первого альбома (по умолчанию) или реальные фото исполнителей с твоего сервера Jellyfin или Subsonic, с автоматическим запасным вариантом.
feat-debug-title = Логи и отчёты о сбоях
feat-debug-desc = Открывай или экспортируй логи прямо из Настроек, с опциональной трассировкой производительности, которую можно открыть в Speedscope или Perfetto. Сбои автоматически записывают отчёт.
feat-cleanup-title = Автоочистка
feat-cleanup-desc = Отсутствующие или удалённые треки автоматически убираются из библиотеки при повторном сканировании. Никаких призрачных записей.
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = Стримь с SoundCloud после однократного входа через браузер. Поиск, прогрессивный MP3 и Go+ AAC/HLS, понравившиеся треки как избранное и плейлисты только для чтения.
feat-miniplayer-title = Мини-плеер
feat-miniplayer-desc = Компактный оверлей текущего воспроизведения, который можно включить из нижней панели для уменьшенного вида.
feat-tray-title = Сворачивание в трей
feat-tray-desc = Закрывай в значок системного трея вместо выхода, чтобы воспроизведение продолжалось в фоне. Переключается в Настройках.
feat-badges-title = Значки типов файлов
feat-badges-desc = В строке локального трека отображается формат файла, например MP3, FLAC или WAV.

## What's new

## Install chooser

## Performance
perf-skip-label = Пропуск проиндексированного
perf-skip-desc = Повторное сканирование обрабатывает только новые файлы. 10k треков + 5 новых = читаются только 5.
perf-parallel-label = Параллельный запуск
perf-art-label = Кэширование обложек
perf-art-desc = Обложки извлекаются один раз, сохраняются на диск. Никакого повторного декодирования.
perf-lazy-label = Ленивая загрузка изображений
perf-lazy-desc = Изображения загружаются при появлении в видимой области.
perf-io-label = Неблокирующий I/O
perf-http-label = HTTP-кэш обложек
perf-sort-label = Умная сортировка

## Install
install-nix-title = Nix / NixOS
install-nix-run = Запуск без установки:
install-nix-profile = Или добавь в свой профиль:
install-nix-note = Поддержка NixOS flake с Cachix binary cache.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Установка с помощью предпочитаемого помощника:
install-flatpak-title = Flatpak
install-flatpak-desc = Установите Kopuz из Flathub:
install-flatpak-note = Обновления будут поступать через стандартный механизм обновления Flatpak.
install-appimage-title = AppImage
install-appimage-desc-1 = Скачай с GitHub Releases и запусти. Требуется
install-appimage-desc-2 = { " " }и
install-appimage-desc-3 = { " " }в системе.
install-appimage-note-1 = Пользователи Arch: если падает с ошибкой WebKitNetworkProcess, запускай с префиксом
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-desc-1 = Скачай
install-macos-desc-2 = { " " }с GitHub Releases. Если macOS блокирует, сними флаг карантина:

## Platforms
platforms-title = Скачать Kopuz
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Только Apple Silicon
platforms-linux = Linux

## Support
support-title = Поддержать Kopuz
support-subtitle = Kopuz бесплатен и имеет открытый исходный код. Пожертвования поддерживают разработку.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-usdt-note = (сеть Solana)

## Nextcloud
nextcloud-subtitle = Добавь через Настройки → Медиа-серверы → Добавить → Nextcloud.
nextcloud-connect-title = Используй пароль приложения
nextcloud-connect-desc = Укажи базовый URL Nextcloud, имя пользователя и пароль приложения, который можно отозвать. Создай его в разделе Настройки Nextcloud → Безопасность.
nextcloud-folders-title = Выбери папки медиатеки
nextcloud-folders-desc = Выбери одну или несколько папок для каждого источника. Kopuz читает дерево папок WebDAV, кэширует найденные обложки и определяет длительность треков по заголовкам файлов.
nextcloud-webdav-title = Работа напрямую через WebDAV
nextcloud-webdav-desc = Медиатека повторяет структуру папок; плейлисты и радио недоступны. Если Nextcloud Music предоставляет доступ по Subsonic, используй этот источник для более полных метаданных и плейлистов.
nextcloud-playback-title = Слушай или скачивай
nextcloud-playback-desc = Воспроизводи файлы прямо со своего сервера или скачивай их для офлайн-прослушивания: аудио не проходит через инфраструктуру Kopuz.

## YouTube Music
ytmusic-subtitle = Добавь через Настройки → Медиа-серверы → Добавить → YouTube Music.
ytmusic-token-title = Помощник не нужен
ytmusic-signin-title = Вход через браузер
ytmusic-signin-desc = Kopuz открывает страницу входа Google в отдельном профиле браузера и получает cookie сессии. После входа доступны медиатека, понравившаяся музыка, плейлисты и отслеживаемые исполнители.
ytmusic-anon-title = Анонимный режим
ytmusic-anon-desc = Без входа, без cookie. Просматривай, ищи, открывай страницы исполнителей, альбомов и плейлистов, запускай микс-радио и слушай публичные треки. Лайки и просмотры библиотеки отключены.

## Apple Music
applemusic-subtitle = Добавь через Настройки → Медиа-серверы → Добавить → Apple Music.
applemusic-signin-title = Войди или вставь токен
applemusic-signin-desc = В настольной версии Kopuz открывает Apple Music в изолированном профиле браузера семейства Chromium. Вместо этого можно вручную вставить media-user-token.
applemusic-playback-title = Widevine в настольной версии
applemusic-playback-desc = Для воспроизведения Apple Music нужен Widevine CDM из установленного браузера или сервиса плагинов Mozilla. Kopuz не поставляет CDM.
applemusic-features-title = Что доступно
applemusic-features-desc = Просмотр каталога и медиатеки, синхронизация избранного, добавление и удаление треков в плейлистах, радио на основе треков и плейлистов, загрузки и тексты с пословной синхронизацией.
applemusic-android-title = Поддержка Android
applemusic-android-desc = Вход выполняется во встроенном WebView, но воспроизведение Apple Music на Android пока не поддерживается.

## SoundCloud
soundcloud-subtitle = Добавь через Настройки → Медиа-серверы → Добавить → SoundCloud.
soundcloud-signin-title = Однократный вход через браузер
soundcloud-features-title = Что ты получаешь
soundcloud-features-desc = Поиск, воспроизведение треков (прогрессивный MP3 плюс потоки Go+ AAC/HLS), понравившиеся треки как избранное, плейлисты только для чтения и лайки/дизлайки. Удаление источника очищает его изолированный профиль.

## Feature guides
guides-title = Руководства
## Privacy and storage

## Requirements

## Community
community-title = Участие в проекте Kopuz

## Sponsors
sponsors-cta = Стать спонсором

## Gallery

## Footer
footer-license = Лицензия EUPL v1.2 | Бесплатное ПО с открытым исходным кодом
footer-github = GitHub
footer-releases = Релизы
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz: бесплатный музыкальный плеер с открытым кодом
og-title = Kopuz: бесплатный музыкальный плеер с открытым кодом
og-image-alt = Музыкальный плеер Kopuz
twitter-title = Kopuz: бесплатный музыкальный плеер с открытым кодом
## User-friendly aliases (synced with EN updates)
perf-parallel-desc = { perf-parallel-desc-1 } { perf-parallel-desc-2 }
perf-io-desc = { perf-io-desc-1 } { perf-io-desc-2 }
perf-http-desc = { perf-http-desc-1 } { perf-http-desc-2 }
perf-sort-desc = { perf-sort-desc-1 } { perf-sort-desc-2 }

## Deep link (/j)
join-title = Открываем Kopuz…
join-opening = Открываем Kopuz…
join-fallback = Kopuz не открылся. Ещё не установлен?
join-no-payload = К этой ссылке не прикреплена очередь.
join-download = Скачать Kopuz

## Privacy policy
footer-privacy = Конфиденциальность
privacy-english-note = Эта политика ведётся на английском языке, чтобы её формулировки оставались точными. Переводы остальной части сайта не меняют её содержания.
privacy-back = Вернуться на главную

## Added keys
hero-title-1 = Один плеер для
hero-title-2 = всей твоей музыки.
hero-desc = Kopuz собирает локальные файлы, NAS, твой сервер и аккаунты стриминговых сервисов в одну медиатеку. Он бесплатный и с открытым исходным кодом.
features-source-spotify = Spotify
feat-local-title = Папки и серверы
feat-local-desc = Добавляй локальные папки или подключай сервер Jellyfin или Subsonic, например Navidrome. Треки из всех добавленных источников собираются в одной медиатеке, а Subsonic синхронизирует твою очередь между клиентами.
feat-theming-desc = Выбери одну из 24 встроенных светлых и тёмных тем или создай свою и настрой каждый цвет сам.
feat-native-desc = Управляй воспроизведением с помощью медиаклавиш и элементов управления системы: MPRIS в Linux, «Сейчас исполняется» в macOS и медиаоверлей в Windows.
feat-fav-desc = Храни избранное на устройстве или синхронизируй его с сервером. Создавай плейлисты, добавляй в них альбомы целиком и синхронизируй плейлисты с сервером.
feat-i18n-desc = Используй Kopuz на 28 языках.
feat-ytdlp-title = Загрузка с YouTube
feat-ytdlp-desc = Вставь ссылку на песню, альбом или плейлист из YouTube или YouTube Music. Kopuz скачает их через собственный клиент YouTube, добавит теги с названием, исполнителем, альбомом и обложкой и разложит по альбомам. Режим лучшего звука сохраняет исходный поток, и ничего устанавливать не нужно. Чтобы преобразовать в MP3, FLAC или WAV, установи ffmpeg.
feat-crossfade-title = Кроссфейд и воспроизведение без пауз
feat-crossfade-desc = Плавно переходи между треками в настольной версии или отключи кроссфейд для альбомов без пауз.
feat-youtube-desc = Просматривай, ищи и запускай радио-микс от любого трека. Раздел «Рекомендации» предлагает песни, альбомы, исполнителей и настроения, а на страницах исполнителей есть лучшие песни, релизы и похожие исполнители. Войди, чтобы получить медиатеку, Liked Music и плейлисты, или просматривай без аккаунта.
feat-spotify-title = Spotify
feat-spotify-desc = Просматривай сохранённые треки, альбомы, плейлисты и рекомендации, ищи музыку и скробблируй. Слушай через Web Playback SDK от Spotify или на любом из своих устройств Spotify Connect. Для воспроизведения нужна подписка Spotify Premium.
new-title = Что нового в Kopuz
install-cargo-title = Cargo (crates.io)
install-cargo-desc = Установи последнюю версию через Cargo:
install-macos-homebrew = Установи сборку для Apple silicon из tap Homebrew:
install-android-title = Android
install-android-desc = Установи Kopuz из Google Play или скачай подписанный APK arm64-v8a с GitHub Releases. Нужен Android 7.0 (API 24) или новее.
install-android-note = Если установленный APK получен из другого источника, удали его перед установкой этого.
ytmusic-token-desc = Слушать можно без входа, и устанавливать помощник не нужно.
ytmusic-signin-note = Kopuz использует твой браузер по умолчанию. Вместо него можно выбрать Firefox, Chrome, Chromium, Brave, Edge, Vivaldi, Helium, LibreWolf, Zen или Floorp.
ytmusic-premium-title = Премиальное качество
ytmusic-premium-desc = С аккаунтом Music Premium Kopuz воспроизводит и скачивает потоки более высокого качества через собственный клиент YouTube. В анонимном режиме треки только для Premium не воспроизводятся.
soundcloud-signin-desc = URL, пароль и токен не нужны. Kopuz открывает вход в SoundCloud в отдельном профиле браузера и удаляет этот профиль, когда ты удаляешь источник.
spotify-guide-subtitle = Kopuz использует официальный Web API Spotify и воспроизведение в браузере. Он не видит твой пароль и не передаёт звук через себя.
spotify-step-1-title = Создай приложение Spotify
spotify-step-1-desc = На панели разработчика Spotify создай приложение и включи Web API и Web Playback SDK. В разделе User Management добавь каждый аккаунт, который будет слушать (в Development Mode не больше пяти). Затем укажи в точности такой redirect URI:
spotify-step-2-title = Добавь свой Client ID
spotify-step-2-desc = В разделе Настройки → Медиа-серверы → Добавить → Spotify вставь Client ID и подтверди доступ. Client Secret не нужен. Во время входа порт 8898 должен быть свободен.
spotify-step-3-title = Выбери, где слушать
spotify-step-3-desc = Оставь открытой вкладку встроенного плеера или выбери одно из своих устройств Spotify Connect в меню устройств.
spotify-requirement = Для воспроизведения нужна подписка Spotify Premium. Используй Chrome, Edge, Brave, Chromium, Vivaldi, Helium или Safari в macOS. Firefox не подходит.
spotify-full-guide = Читать полное руководство по настройке Spotify и решению проблем
privacy-local-title = Одна локальная база данных
privacy-local-desc = Kopuz хранит медиатеку, плейлисты, избранное, счётчики воспроизведений и данные источников в kopuz.db, а настройки рядом с ней в settings.toml.
privacy-accounts-title = Раздельные входы
privacy-accounts-desc = У каждого источника свой вход и своё избранное. Для входа через браузер используются отдельные профили, а не твой обычный сеанс браузера.
privacy-files-title = Кэш и логи
privacy-files-desc = Обложки альбомов и офлайн-треки хранятся в системной папке кэша. Логи можно открыть или экспортировать в Настройках.
requirements-title = Требования и ограничения
about-title = Что такое копуз?
about-desc-1 = Копуз относится к древним тюркским струнным инструментам. На нём играли сказители и шаманы. Считается, что он предок нескольких среднеазиатских лютен.
about-desc-2 = По тюркскому преданию, он принадлежит сказителю Деде Коркуту. Родственные инструменты: кыргызский комуз и казахский кобыз. Хомус, несмотря на похожее название, к ним не относится.
community-issues-title = Issues
community-issues-desc = Сообщи об ошибке или возьми открытую задачу.
community-discussions-title = Discussions
community-discussions-desc = Задавай вопросы и предлагай изменения.
community-discord-desc = Общайся с пользователями и участниками проекта.
community-contribute-title = Участвуй в проекте
community-contribute-desc = Получи исходный код, настрой среду разработки или помоги с переводом.
platforms-android = Android
platforms-android-note = ARM64 · Android 7.0+ · API 24+
home-meta-desc = Kopuz это бесплатный музыкальный плеер для компьютера и Android. Слушай локальные файлы, ресурсы SMB, Jellyfin, Subsonic, YouTube Music, Spotify и другое в одной медиатеке.
home-meta-keywords = Kopuz, музыкальный плеер, Jellyfin, Navidrome, Subsonic, SMB, NAS, Clippsly, Nextcloud, YouTube Music, Apple Music, SoundCloud, Spotify, открытый исходный код, Android, Linux, macOS, Windows, тексты песен, эквалайзер
og-desc = Бесплатный музыкальный плеер для локальных файлов, серверов и стриминговых сервисов на компьютере и Android: с синхронизированными текстами, темами и 10-полосным эквалайзером.
twitter-desc = Слушай локальные файлы, ресурсы SMB, Jellyfin, Subsonic, YouTube Music, SoundCloud, Spotify и другое в одной медиатеке.
nav-home = Главная
footer-license-line = EUPL 1.2, бесплатное ПО с открытым исходным кодом
shelf-link-to-us = Поставь ссылку на нас с кнопкой 88×31
hero-release = Последний релиз
hero-cta-download-for = Скачать для
hero-cta-all = Все платформы
home-sources-title = Слушай музыку из
source-local = Локальные папки
source-navidrome = Navidrome и Subsonic
source-radio = Интернет-радио
moment-lyrics-title = Тексты, которые идут в такт
moment-lyrics-alt = Kopuz воспроизводит трек с синхронизированным текстом на весь экран
moment-library-title = Одна медиатека для всех источников
moment-library-alt = Медиатека Kopuz с альбомами из нескольких источников
moment-themes-title = Твои цвета или цвета твоих обоев
moment-themes-alt = Редактор тем Kopuz
moment-themes-extra = Или позволь Kopuz создать тему по обоям рабочего стола.
theme-default = Gruvbox dark soft
theme-amoled = AMOLED
theme-wallpaper = Обои
wallpaper-sunset = Закат
wallpaper-lake = Озеро
home-see-all-features = Все возможности
home-get-title = Получи Kopuz
home-get-download = Скачать
home-also-on = Kopuz можно также установить через Cargo, Nix, AUR, Flatpak или Homebrew.
home-sponsors-title = Спонсоры
home-sponsor-cta = Стать спонсором на GitHub
copy = Копировать
copied = Скопировано
copy-address = Копировать адрес { $coin }
nav-primary-aria = Основная навигация
nav-menu-open = Открыть меню
nav-menu-close = Закрыть меню
footer-nav-aria = Нижняя навигация
theme-use-light = Включить светлую тему
theme-use-dark = Включить тёмную тему
theme-leave-moe = Выйти из режима moe
mode-simple = Простой сайт
mode-full = Полный сайт
webbutton-alt = Веб-кнопка Kopuz 88x31
features-page-title = Функции Kopuz: источники, тексты песен, темы, воспроизведение
download-page-title = Скачать Kopuz для Windows, macOS, Linux и Android
guides-page-title = Руководства по настройке Kopuz для серверов и стриминга
support-page-title = Поддержка Kopuz: спонсоры, пожертвования, сообщество
notfound-page-title = Страница не найдена | Kopuz
guide-title-suffix = руководство по настройке | Kopuz
notfound-title = Страница не найдена.
notfound-desc = По этому адресу ничего не играет.
notfound-home = На главную
features-group-sources = Источники
features-group-playback = Воспроизведение
features-group-library = Медиатека
features-group-desktop = Компьютер
features-group-android = Android
features-group-integrations = Интеграции
features-group-performance = Производительность
features-group-privacy = Конфиденциальность
features-jump-aria = Группы возможностей
features-privacy-link = Читать политику конфиденциальности
install-other-title = Другие способы установки
req-spotify = Для воспроизведения нужны подписка Premium, собственный Client ID и поддерживаемый браузер.
req-applemusic = Для воспроизведения на компьютере нужен Widevine CDM. На Android можно войти, но воспроизведение там пока не работает.
req-nextcloud = Обычный WebDAV не поддерживает плейлисты и радио. Если в Nextcloud Music есть конечная точка Subsonic, используй её.
req-android = Релизные APK требуют телефон с arm64-v8a и Android 7.0 (API 24) или новее. Статус Discord и системный трей недоступны.
req-appimage = Нужны webkit2gtk-4.1 и GTK 3. Для трея также нужна библиотека appindicator.
req-ytmusic = В анонимном режиме треки только для Premium не воспроизводятся. Войди с аккаунтом Music Premium, чтобы слушать их.
req-crossfade = Кроссфейд и воспроизведение без пауз работают с нативным воспроизведением на компьютере, но не со звуком Spotify, который играет в браузере.
req-spotify-limits-label = Ограничения Spotify
req-spotify-limits = В Development Mode приложение допускает не больше пяти слушателей, поиск возвращает не больше десяти результатов каждого типа, плейлисты доступны только для чтения, а загрузки, радио, редактирование тегов, эквалайзер и кроссфейд отключены.
support-note = Я Temidaradev, и я создаю Kopuz. Я студент без постоянного дохода, и именно спонсоры позволяют мне развивать Kopuz во время учёбы.
support-goal-label = Цель GitHub Sponsors
support-goal-amount = ${ $current } из ${ $goal } в месяц
support-goal-aria = Прогресс цели GitHub Sponsors
support-crypto-summary = Криптоадреса
sponsors-tier-special = Особые
sponsors-tier-monthly = Ежемесячные
sponsors-tier-one-time = Разовые
features-source-smb = SMB и NAS
features-source-clippsly = Clippsly
feat-smb-title = SMB и NAS
feat-smb-desc = Добавь ресурс SMB 2 или 3 или папку внутри него как медиатеку. Kopuz читает встроенные теги и обложки и воспроизводит прямо с ресурса с перемоткой, поэтому монтировать его не нужно.
feat-clippsly-title = Clippsly
feat-clippsly-desc = Войди с именем пользователя Clippsly и паролем приложения с clippsly.app. Слушай и скачивай из своего хранилища, загружай аудиофайлы и следи за использованием места. Удалённые плейлисты пока не поддерживаются.
feat-updates-title = Автоматические обновления
feat-updates-desc = В настольной версии баннер сообщает о выходе новой версии, а Kopuz сам скачивает и устанавливает её.
feat-cli-title = Демон и командная строка
feat-cli-desc = Плеер работает как демон kopuzd, к которому подключается окно. Через kopuzctl можно управлять воспроизведением, настраивать источники и запускать сканирование медиатеки из терминала или скрипта, а также обращаться к API gRPC напрямую.
install-android-play = Установить из Google Play
footer-made-with = Сайт сделан с
footer-love = любовью
footer-made-by = автор:
source-smb = SMB и NAS
source-clippsly = Clippsly
player-prev-label = Предыдущий раздел
player-next-label = Следующий раздел
player-seek-label = Положение на странице
player-queue-label = Разделы этой страницы
player-queue-title = На этой странице
dl-file-exe = Установщик (.exe)
dl-file-msi = Установщик (.msi)
dl-file-dmg = Образ диска (.dmg)
dl-file-appimage = AppImage
dl-file-deb = Debian и Ubuntu (.deb)
dl-file-rpm = Fedora и RHEL (.rpm)
dl-file-apk = APK
dl-release-page = Страница релиза
dl-this-device = Это устройство
dl-dialog-ready = Последний релиз: { $version }. Выбери установщик для своего устройства.
dl-dialog-close = Закрыть
dl-dialog-release = Открыть релиз на GitHub
guide-nextcloud-summary = Подключись по WebDAV с паролем приложения и выбери папки медиатеки.
guide-ytmusic-summary = Войди через браузер или слушай без аккаунта.
guide-applemusic-summary = Войди, настрой Widevine на компьютере и узнай, что работает на Android.
guide-soundcloud-summary = Войди один раз в браузере и узнай, что поддерживает источник.
guide-spotify-summary = Создай приложение разработчика, добавь его Client ID и выбери, где слушать.
chapter-home = Главная
chapter-sources = Источники
chapter-lyrics = Тексты
chapter-library = Медиатека
chapter-themes = Темы
chapter-download = Скачать
chapter-sponsors = Спонсоры
chapter-whats-new = Что нового
chapter-name = Название
chapter-not-found = Не найдено
chapter-privacy = Конфиденциальность
chapter-install = Установка
chapter-requirements = Требования
chapter-support = Поддержка
chapter-community = Сообщество
chapter-features = Возможности
chapter-guides = Руководства
guide-smb-summary = Укажи Kopuz ресурс на своём NAS и войди с аккаунтом NAS.
smb-subtitle = Выбери SMB / NAS в списке источников. Монтировать ресурс заранее не нужно.
smb-share-title = Укажи ресурс
smb-share-desc = Введи URL ресурса или папки внутри него, например:
smb-share-port = Если твой NAS использует нестандартный порт, добавь его в URL:
smb-signin-title = Войди с аккаунтом NAS
smb-signin-desc = Введи имя пользователя и пароль NAS. Для доменного аккаунта введи DOMAIN\username. Kopuz требует SMB 2 или 3 и аккаунт: он не ищет ресурсы в сети и не входит как гость.
smb-library-title = Как работает медиатека
smb-library-desc = Kopuz сканирует ресурс и его подпапки в поисках аудиофайлов и читает встроенные теги и обложки. Он воспроизводит прямо с ресурса, с перемоткой, в том числе на Android. Твоё избранное хранится в Kopuz.
smb-limits-title = Ресурс остаётся только для чтения
smb-limits-desc = Kopuz ничего не меняет на ресурсе, поэтому нельзя редактировать там теги, хранить плейлисты или скачивать его треки для офлайн-прослушивания.
guide-clippsly-summary = Войди с паролем приложения, слушай своё хранилище и загружай музыку.
clippsly-subtitle = Добавь Clippsly в настройках источников.
clippsly-signin-title = Войди с паролем приложения
clippsly-signin-desc = Создай пароль приложения на clippsly.app. Затем войди в Kopuz с именем пользователя Clippsly и этим паролем приложения.
clippsly-locker-title = Твоё хранилище
clippsly-locker-desc = Просматривай и слушай музыку из хранилища Clippsly, а также скачивай треки для офлайн-прослушивания.
clippsly-upload-title = Загрузка музыки
clippsly-upload-desc = Загружай аудиофайлы с карточки настроек источника. Когда вся партия загрузится, Kopuz обновит медиатеку. Карточка также показывает, сколько места ты используешь, а квоту задаёт Clippsly.
clippsly-bonus = Привяжи Kopuz к аккаунту Clippsly, и Clippsly бесплатно добавит 3 ГБ места.
clippsly-limits-title = Плейлисты
clippsly-limits-desc = Плейлисты, которые ты хранишь в Clippsly, пока не отображаются в Kopuz.
clippsly-site = Перейти на clippsly.app
