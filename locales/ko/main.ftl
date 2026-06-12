## Language
lang-name = 한국어

## Nav
nav-new = 신규
nav-announce = YouTube Music 스트리밍, Discover, 믹스 라디오, 20+ 언어
nav-features = 기능
nav-install = 설치
nav-download = 다운로드
nav-sponsors = 후원자
nav-github = GitHub ↗
nav-gallery = 갤러리
nav-lang-label = 언어

## Hero
hero-title-1 = 당신의 음악.
hero-title-2 = 당신의 방식으로.
hero-desc = Kopuz는 Rust와 Dioxus로 구축된 현대적이고 가벼운 음악 플레이어입니다. 로컬 폴더를 스캔하고, Jellyfin 또는 Navidrome에서 스트리밍하고, 원하는 방식으로 라이브러리를 탐색하세요.
hero-cta-download = 다운로드
hero-cta-github = GitHub에서 보기
hero-screenshot-alt = Kopuz — 홈 뷰

## Features
features-title = 필요한 모든 것.
features-chip = 구독 없음. 추적 없음. 오직 당신의 음악.
features-works-with = 함께 작동
features-source-local = 로컬 파일
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API

feat-local-title = 로컬 + 스트리밍
feat-local-desc = 로컬 폴더를 지정하거나 Jellyfin / Subsonic (Navidrome)에 연결. 원하는 대로 혼합 사용.
feat-theming-title = 테마
feat-theming-desc = 색상 변수 완전 제어가 가능한 동적 테마. 처음부터 자신만의 테마를 만들거나 프리셋 선택.
feat-native-title = 네이티브 통합
feat-native-desc = Linux의 MPRIS, macOS의 Now Playing, Windows의 System Media Transport. 완전 연결.
feat-lyrics-title = 동기화된 가사
feat-lyrics-desc = 실시간 스크롤 가사, 동기화 또는 일반, 음악에 맞춰 자동 스크롤.
feat-eq-title = 이퀄라이저
feat-eq-desc = 내장 프리셋과 사운드 완전 제어가 가능한 5밴드 이퀄라이저.
feat-fav-title = 즐겨찾기 & 재생목록
feat-fav-desc = 로컬에서 트랙에 별표를 매기거나 서버와 즐겨찾기 동기화. 재생목록 생성, 앨범 전체를 한 번에 추가.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = 내장 ListenBrainz 스크로블링. Jellyfin 사용자는 listenbrainz 플러그인도 사용 가능.
feat-discord-title = Discord RPC
feat-discord-desc = 내장 Discord Rich Presence로 친구에게 듣고 있는 곡 표시. 설정 불필요.
feat-search-title = 검색
feat-search-desc = 아티스트, 앨범, 트랙 전반에 걸친 실시간 검색. 입력하는 즉시 결과 표시.
feat-genre-title = 장르 탐색
feat-genre-desc = 장르별로 전체 라이브러리 탐색 — 로컬 파일과 서버 음악 모두 작동.
feat-logs-title = 청취 로그
feat-logs-desc = 재생 횟수를 로컬에서 추적. 시간이 지나면서 실제로 가장 많이 듣는 음악 확인.
feat-i18n-title = i18n
feat-i18n-desc = 영어, 러시아어, 독일어, 프랑스어, 스페인어, 터키어, 우크라이나어, 폴란드어, 아랍어, 그리스어, 히브리어, 헝가리어, 인도네시아어, 일본어, 한국어, 루마니아어, 포르투갈어, 중국어 등.
feat-ytdlp-title = yt-dlp 다운로더
feat-ytdlp-desc = YouTube에서 오디오 직접 다운로드. MP3, FLAC, WAV, MP4 또는 최고 품질로 출력. SponsorBlock, 챕터 분할, 속도 제한 포함.
feat-crossfade-title = 크로스페이드 & 전환
feat-crossfade-desc = 트랙 전환을 부드럽게 혼합. 네이티브 데스크탑 빌드에서 크로스페이드 지원.
feat-channels-title = 채널 모드
feat-channels-desc = 스테레오, 모노, 왼쪽 전용, 오른쪽 전용 및 L/R 스왑. 세밀한 오디오 채널 제어.

## Performance
perf-title = 빠르게 설계됨.
perf-subtitle = 대용량 라이브러리. 즉시 시작. 멈춤 없음.
perf-skip-label = 이미 인덱싱된 항목 건너뛰기
perf-skip-desc = 재스캔은 새 파일만 처리. 1만 트랙 + 5개 신규 = 5개만 읽기.
perf-parallel-label = 병렬 시작
perf-parallel-desc-1 = 라이브러리, 설정, 재생목록, 즐겨찾기가 모두 병렬로 로드됩니다
perf-parallel-desc-2 = { " " }로. 거의 즉시 열림.
perf-art-label = 앨범 아트 캐싱
perf-art-desc = 커버는 한 번 추출되어 디스크에 저장. 반복 보기 시 다시 디코딩되지 않음.
perf-lazy-label = 지연 이미지 로딩
perf-lazy-desc = 검색 결과의 수백 개 앨범 커버 — 화면에 보일 때까지 로드되지 않음.
perf-io-label = 비차단 I/O
perf-io-desc-1 = 무거운 작업은
perf-io-desc-2 = { " " }스레드에서 실행. 전체 라이브러리 스캔 중에도 UI 응답 유지.
perf-http-label = HTTP 아트 캐싱
perf-http-desc-1 = 사용자 정의
perf-http-desc-2 = { " " }프로토콜이 1년 캐시 헤더와 함께 커버를 제공. Webview는 다시 가져오지 않음.

## Install
install-title = 설치
install-nix-title = Nix / NixOS
install-nix-run = 설치 없이 실행:
install-nix-profile = 또는 프로필에 추가:
install-nix-note = NixOS flake는 Cachix 바이너리 캐시로 지원.
install-flatpak-title = Flatpak
install-flatpak-desc = 소스 매니페스트에서 설치:
install-flatpak-note = Flathub 등록 예정.
install-appimage-title = AppImage
install-appimage-desc-1 = GitHub Releases에서 다운로드하고 직접 실행. 필요:
install-appimage-desc-2 = { " " }및
install-appimage-desc-3 = { " " }이(가) 시스템에 필요.
install-appimage-note-1 = Arch 사용자: WebKitNetworkProcess 오류로 충돌 시, 다음 접두사로 실행
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = GitHub Releases에서
install-macos-desc-2 = { " " }를 다운로드. macOS가 차단하면 격리 플래그 제거:

## Platforms
platforms-title = Kopuz 다운로드
platforms-subtitle = 무료 오픈소스. 모든 릴리스는 GitHub에 있음.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apple Silicon 전용
platforms-linux = Linux
platforms-download = 다운로드 →

## Support
support-title = Kopuz 후원
support-subtitle = Kopuz는 무료 오픈소스입니다. 후원이 유지를 가능하게 합니다.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — 또는 암호화폐 전송 —
support-usdt-note = (Solana 체인)

## Sponsors
sponsors-title = 후원자
sponsors-subtitle = Kopuz를 유지하는 사람들.
sponsors-cta = 후원자 되기

## Gallery
gallery-page-title = 갤러리 — Kopuz Music Player
gallery-page-desc = 동작 중인 Kopuz 스크린샷 — 홈, 라이브러리, 재생목록, 전체화면 플레이어, 가사, 테마 편집기 등.
gallery-title = 갤러리
gallery-subtitle = Kopuz 스크린샷 — Normal 및 Modern 스타일.
gallery-home = 홈
gallery-home-styles = Normal / Modern
gallery-library = 라이브러리
gallery-library-styles = Normal / Vaxry
gallery-playlist = 재생목록
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = 전체화면 플레이어
gallery-fullscreen-desc = 몰입형 전체 창 재생 뷰.
gallery-fullscreen-lyrics-title = 전체화면 가사
gallery-fullscreen-lyrics-desc = 전체 창 모드의 동기화된 가사.
gallery-search-title = 검색
gallery-search-desc = 아티스트, 앨범, 트랙 전반에 걸친 실시간 검색.
gallery-theme-title = 테마 편집기
gallery-theme-desc = 색상 변수 완전 제어. 직접 만들거나 프리셋 선택.
gallery-settings-title = 플레이어 설정
gallery-settings-desc = 오디오, 동작, 통합 등을 구성.
gallery-downloader-title = 다운로더
gallery-downloader-desc = Kopuz 내에서 트랙을 직접 다운로드.
gallery-prev = ← 이전
gallery-next = 다음 →
gallery-label-normal-home = Normal — 홈
gallery-label-modern-home = Modern — 홈
gallery-label-normal-library = Normal — 라이브러리
gallery-label-vaxry-library = Vaxry — 라이브러리
gallery-label-normal-playlist = Normal — 재생목록
gallery-label-modern-playlist = Modern — 재생목록
gallery-label-fullscreen = 전체화면 플레이어
gallery-label-fullscreen-lyrics = 전체화면 가사
gallery-label-search = 검색
gallery-label-theme-editor = 테마 편집기
gallery-label-player-settings = 플레이어 설정
gallery-label-downloader = 다운로더

## Footer
footer-license = MIT 라이선스 — 무료 & 오픈소스
footer-github = GitHub
footer-releases = 릴리스
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz는 Rust와 Dioxus로 구축된 현대적이고 가벼운 음악 플레이어입니다. Jellyfin 또는 Navidrome에서 스트리밍, 로컬 파일 탐색, 동기화 가사, 이퀄라이저, 테마 등.
home-meta-keywords = Kopuz, 음악 플레이어, Rust, Dioxus, Jellyfin, Navidrome, 오픈소스, Linux, macOS, Windows, MPRIS, 가사
og-title = Kopuz — Music Player
og-desc = Rust로 구축된 현대적이고 가벼운 음악 플레이어. 로컬 파일, Jellyfin, Navidrome, 동기화 가사, 이퀄라이저, Discord RPC 등. 무료 오픈소스.
og-image-alt = Kopuz 음악 플레이어
twitter-title = Kopuz — Music Player
twitter-desc = Rust로 구축된 현대적이고 가벼운 음악 플레이어. 무료 오픈소스.
