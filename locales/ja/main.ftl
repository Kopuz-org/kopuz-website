## Language
lang-name = 日本語

## Nav
nav-new = 新着
nav-announce = モダンUI、Vaxryテーマ、yt-dlpダウンローダー、20以上の言語
nav-features = 機能
nav-install = インストール
nav-download = ダウンロード
nav-sponsors = スポンサー
nav-github = GitHub ↗
nav-gallery = ギャラリー
nav-lang-label = 言語

## Hero
hero-title-1 = あなたの音楽。
hero-title-2 = あなたのやり方で。
hero-desc = KopuzはRustとDioxusで構築されたモダンで軽量な音楽プレイヤーです。ローカルフォルダをスキャンし、JellyfinやNavidromeからストリーミングし、ライブラリを好きなように閲覧できます。
hero-cta-download = ダウンロード
hero-cta-github = GitHubで見る
hero-screenshot-alt = Kopuz — ホーム画面

## Features
features-title = 必要なものすべて。
features-chip = サブスクなし。トラッキングなし。あなたの音楽だけ。
features-works-with = 対応サービス
features-source-local = ローカルファイル
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API

feat-local-title = ローカル + ストリーミング
feat-local-desc = ローカルフォルダを指定するか、Jellyfin / Subsonic (Navidrome)に接続。自由に組み合わせ可能。
feat-theming-title = テーマ
feat-theming-desc = 色変数の完全制御によるダイナミックテーマ。自分のテーマをゼロから作るかプリセットを選べます。
feat-native-title = ネイティブ統合
feat-native-desc = LinuxのMPRIS、macOSのNow Playing、WindowsのSystem Media Transport。完全に接続。
feat-lyrics-title = 同期歌詞
feat-lyrics-desc = リアルタイムスクロール歌詞、同期またはプレーン、音楽に合わせた自動スクロール。
feat-eq-title = イコライザー
feat-eq-desc = 内蔵プリセットとサウンドの完全制御を備えた5バンドイコライザー。
feat-fav-title = お気に入りとプレイリスト
feat-fav-desc = ローカルでトラックにスターを付けるか、お気に入りをサーバーと同期。プレイリストを作成し、アルバム全体を一度に追加。
feat-scrobble-title = Scrobbling
feat-scrobble-desc = ListenBrainzスクロブリング内蔵。Jellyfinユーザーはlistenbrainzプラグインも使用可能。
feat-discord-title = Discord RPC
feat-discord-desc = 埋め込みDiscord Rich Presenceで友達に再生中の曲を表示。セットアップ不要。
feat-search-title = 検索
feat-search-desc = アーティスト、アルバム、トラックを横断するリアルタイム検索。入力中に即座に結果表示。
feat-genre-title = ジャンル別ブラウジング
feat-genre-desc = ジャンル別にライブラリ全体を閲覧 — ローカルファイルとサーバー音楽の両方に対応。
feat-logs-title = 再生履歴
feat-logs-desc = 再生回数をローカルで追跡。実際に最もよく聴いている曲を時間の経過とともに確認。
feat-i18n-title = i18n
feat-i18n-desc = 英語、ロシア語、ドイツ語、フランス語、スペイン語、トルコ語、ウクライナ語、ポーランド語、アラビア語、ギリシャ語、ヘブライ語、ハンガリー語、インドネシア語、日本語、韓国語、ルーマニア語、ポルトガル語、中国語など。
feat-ytdlp-title = yt-dlpダウンローダー
feat-ytdlp-desc = YouTubeから音声を直接ダウンロード。MP3、FLAC、WAV、MP4、または最高品質で出力。SponsorBlock、チャプター分割、レート制限を含む。
feat-crossfade-title = クロスフェードとトランジション
feat-crossfade-desc = スムーズな再生のためにトラック間のトランジションをブレンド。ネイティブデスクトップビルドでクロスフェードをサポート。
feat-channels-title = チャンネルモード
feat-channels-desc = ステレオ、モノ、左のみ、右のみ、L/R入れ替え。きめ細かなオーディオチャンネル制御。

## Performance
perf-title = 速さを追求した設計。
perf-subtitle = 大規模ライブラリ。即時起動。フリーズなし。
perf-skip-label = インデックス済みをスキップ
perf-skip-desc = 再スキャンは新規ファイルのみ処理。1万トラック + 5新規 = 5のみ読み込み。
perf-parallel-label = 並列起動
perf-parallel-desc-1 = ライブラリ、設定、プレイリスト、お気に入りすべてが並列で読み込まれます
perf-parallel-desc-2 = { " " }で。ほぼ瞬時に起動。
perf-art-label = アルバムアートキャッシュ
perf-art-desc = カバーを一度抽出してディスクに保存。再表示時に再デコードされません。
perf-lazy-label = 遅延画像読み込み
perf-lazy-desc = 検索結果の数百のアルバムカバー — 表示されるまで読み込まれません。
perf-io-label = 非ブロッキングI/O
perf-io-desc-1 = 重い処理は
perf-io-desc-2 = { " " }スレッドで実行。ライブラリのフルスキャン中もUIは応答性を維持。
perf-http-label = HTTPアートキャッシュ
perf-http-desc-1 = カスタム
perf-http-desc-2 = { " " }プロトコルは1年キャッシュヘッダー付きでカバーを配信。Webviewは再取得しません。

## Install
install-title = インストール
install-nix-title = Nix / NixOS
install-nix-run = インストールせずに実行:
install-nix-profile = またはプロファイルに追加:
install-nix-note = NixOS flakeはCachixバイナリキャッシュでサポート。
install-flatpak-title = Flatpak
install-flatpak-desc = ソースマニフェストからインストール:
install-flatpak-note = Flathub掲載予定。
install-appimage-title = AppImage
install-appimage-desc-1 = GitHub Releasesからダウンロードして直接実行。必要なもの:
install-appimage-desc-2 = { " " }と
install-appimage-desc-3 = { " " }がシステムに必要。
install-appimage-note-1 = Archユーザー: WebKitNetworkProcessエラーでクラッシュする場合は、次のプレフィックスで実行
install-appimage-note-2 = 。
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = GitHub Releasesから
install-macos-desc-2 = { " " }をダウンロード。macOSがブロックした場合、検疫フラグをクリア:

## Platforms
platforms-title = Kopuzをダウンロード
platforms-subtitle = 無料でオープンソース。すべてのリリースはGitHubで。
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apple Siliconのみ
platforms-linux = Linux
platforms-download = ダウンロード →

## Support
support-title = Kopuzをサポート
support-subtitle = Kopuzは無料でオープンソース。サポートが存続を支えます。
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — または暗号通貨を送る —
support-usdt-note = (Solanaチェーン)

## Sponsors
sponsors-title = スポンサー
sponsors-subtitle = Kopuzを支える人々。
sponsors-cta = スポンサーになる

## Gallery
gallery-page-title = ギャラリー — Kopuz Music Player
gallery-page-desc = Kopuzの動作スクリーンショット — ホーム、ライブラリ、プレイリスト、フルスクリーンプレイヤー、歌詞、テーマエディターなど。
gallery-title = ギャラリー
gallery-subtitle = Kopuzのスクリーンショット — NormalとModernスタイル。
gallery-home = ホーム
gallery-home-styles = Normal / Modern
gallery-library = ライブラリ
gallery-library-styles = Normal / Vaxry
gallery-playlist = プレイリスト
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = フルスクリーンプレイヤー
gallery-fullscreen-desc = 没入型のフルウィンドウ再生ビュー。
gallery-fullscreen-lyrics-title = フルスクリーン歌詞
gallery-fullscreen-lyrics-desc = フルウィンドウモードの同期歌詞。
gallery-search-title = 検索
gallery-search-desc = アーティスト、アルバム、トラックを横断するリアルタイム検索。
gallery-theme-title = テーマエディター
gallery-theme-desc = 色変数の完全制御。作成またはプリセットを選択。
gallery-settings-title = プレイヤー設定
gallery-settings-desc = オーディオ、動作、統合などを設定。
gallery-downloader-title = ダウンローダー
gallery-downloader-desc = Kopuz内から直接トラックをダウンロード。
gallery-prev = ← 前へ
gallery-next = 次へ →
gallery-label-normal-home = Normal — ホーム
gallery-label-modern-home = Modern — ホーム
gallery-label-normal-library = Normal — ライブラリ
gallery-label-vaxry-library = Vaxry — ライブラリ
gallery-label-normal-playlist = Normal — プレイリスト
gallery-label-modern-playlist = Modern — プレイリスト
gallery-label-fullscreen = フルスクリーンプレイヤー
gallery-label-fullscreen-lyrics = フルスクリーン歌詞
gallery-label-search = 検索
gallery-label-theme-editor = テーマエディター
gallery-label-player-settings = プレイヤー設定
gallery-label-downloader = ダウンローダー

## Footer
footer-license = MITライセンス — 無料 & オープンソース
footer-github = GitHub
footer-releases = リリース
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = KopuzはRustとDioxusで構築されたモダンで軽量な音楽プレイヤーです。JellyfinやNavidromeからストリーミング、ローカルファイルの閲覧、同期歌詞、イコライザー、テーマなど。
home-meta-keywords = Kopuz, 音楽プレイヤー, Rust, Dioxus, Jellyfin, Navidrome, オープンソース, Linux, macOS, Windows, MPRIS, 歌詞
og-title = Kopuz — Music Player
og-desc = Rustで構築されたモダンで軽量な音楽プレイヤー。ローカルファイル、Jellyfin、Navidrome、同期歌詞、イコライザー、Discord RPCなど。無料でオープンソース。
og-image-alt = Kopuz音楽プレイヤー
twitter-title = Kopuz — Music Player
twitter-desc = Rustで構築されたモダンで軽量な音楽プレイヤー。無料でオープンソース。
