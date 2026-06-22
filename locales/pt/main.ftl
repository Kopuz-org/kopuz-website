## Language
lang-name = Português

## Nav
nav-new = Novo
nav-announce = Streaming do YouTube Music, Discover, rádio mix, 20+ idiomas
nav-features = Recursos
nav-install = Instalar
nav-download = Baixar
nav-sponsors = Patrocinadores
nav-github = GitHub ↗
nav-gallery = Galeria
nav-lang-label = Idioma

## Hero
hero-title-1 = Sua música.
hero-title-2 = Do seu jeito.
hero-desc = Kopuz é um reprodutor de música moderno e leve construído com Rust e Dioxus. Escaneie pastas locais, transmita do Jellyfin ou Navidrome e navegue sua biblioteca como quiser.
hero-cta-download = Baixar
hero-cta-github = Ver no GitHub
hero-screenshot-alt = Kopuz — vista inicial

## Features
features-title = Tudo que você precisa.
features-chip = Sem assinaturas. Sem rastreamento. Só sua música.
features-works-with = Funciona com
features-source-local = Arquivos locais
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-ytmusic = YouTube Music
features-source-soundcloud = SoundCloud

feat-local-title = Local + Streaming
feat-local-desc = Aponte para uma pasta local ou conecte ao Jellyfin / Subsonic (Navidrome). Misture como quiser.
feat-theming-title = Temas
feat-theming-desc = Temas dinâmicos com controle total das variáveis de cor. Construa seu tema do zero ou escolha um preset.
feat-native-title = Integração nativa
feat-native-desc = MPRIS no Linux, Now Playing no macOS, System Media Transport no Windows. Totalmente conectado.
feat-lyrics-title = Letras sincronizadas
feat-lyrics-desc = Letras com rolagem em tempo real, sincronizadas ou simples, auto-rolagem para acompanhar a música.
feat-eq-title = Equalizador
feat-eq-desc = Equalizador de 5 bandas com presets integrados e controle total sobre seu som.
feat-fav-title = Favoritos e playlists
feat-fav-desc = Marque faixas localmente ou sincronize favoritos com seu servidor. Crie playlists, adicione álbuns inteiros de uma vez.
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobbling ListenBrainz integrado. Usuários do Jellyfin também podem usar o plugin listenbrainz.
feat-discord-title = Discord RPC
feat-discord-desc = Mostre aos amigos o que você está ouvindo com Discord Rich Presence embarcado. Sem configuração.
feat-search-title = Busca
feat-search-desc = Busca em tempo real entre artistas, álbuns e faixas. Resultados instantâneos enquanto digita.
feat-genre-title = Navegação por gênero
feat-genre-desc = Navegue toda sua biblioteca por gênero — funciona para arquivos locais e música de servidor.
feat-logs-title = Logs de escuta
feat-logs-desc = Contagens de reprodução rastreadas localmente. Veja o que você realmente escuta mais ao longo do tempo.
feat-i18n-title = i18n
feat-i18n-desc = Inglês, russo, alemão, francês, espanhol, turco, ucraniano, polonês, árabe, grego, hebraico, húngaro, indonésio, japonês, coreano, romeno, português, chinês e mais.
feat-ytdlp-title = Baixador yt-dlp
feat-ytdlp-desc = Baixe áudio direto do YouTube. Saída como MP3, FLAC, WAV, MP4 ou melhor qualidade. SponsorBlock, divisão por capítulos e limite de velocidade incluídos.
feat-crossfade-title = Crossfade e transições
feat-crossfade-desc = Mescle transições entre faixas para reprodução mais suave. Suporte crossfade em builds desktop nativos.
feat-channels-title = Modos de canal
feat-channels-desc = Estéreo, Mono, Só-Esquerdo, Só-Direito e troca L/R. Controle fino dos canais de áudio.
feat-youtube-title = YouTube Music
feat-youtube-desc = Backend de streaming completo com uma página Discover no estilo Spotify, perfis de artistas detalhados e rádio mix. Faça login para acessar sua biblioteca, suas Músicas Curtidas e playlists — ou navegue anonimamente.
feat-metadata-title = Imagens de artistas
feat-metadata-desc = Escolha como as imagens de artistas são obtidas: primeira capa de álbum (padrão) ou fotos reais de artistas buscadas do seu servidor Jellyfin ou Subsonic, com fallback automático.
feat-debug-title = Logs e relatórios de falha
feat-debug-desc = Abra ou exporte logs direto das Configurações, com um trace de desempenho opcional que você pode abrir no Speedscope ou Perfetto. Falhas geram um relatório automaticamente.
feat-cleanup-title = Limpeza automática
feat-cleanup-desc = Faixas ausentes ou excluídas são removidas da sua biblioteca automaticamente ao reescanear. Sem entradas fantasmas.
feat-soundcloud-title = SoundCloud
feat-soundcloud-desc = Transmita do SoundCloud após um login único pelo navegador. Busca, reprodução de MP3 progressivo e AAC/HLS do Go+, suas faixas Curtidas como favoritos e playlists somente leitura.
feat-miniplayer-title = Mini-player
feat-miniplayer-desc = Uma sobreposição compacta de reprodução atual que você pode alternar pela barra inferior para uma visão menor.
feat-tray-title = Minimizar para a bandeja
feat-tray-desc = Feche para um ícone na bandeja do sistema em vez de encerrar, para que a reprodução continue rodando em segundo plano. Alterne nas Configurações.
feat-badges-title = Selos de tipo de arquivo
feat-badges-desc = Faixas locais exibem um pequeno selo de formato — MP3, FLAC, WAV e mais — direto na linha da faixa.

## Performance
perf-title = Construído para ser rápido.
perf-subtitle = Bibliotecas grandes. Início instantâneo. Sem travamentos.
perf-skip-label = Pular já indexados
perf-skip-desc = Reescaneamentos só processam arquivos novos. 10k faixas + 5 novas = só 5 lidas.
perf-parallel-label = Início paralelo
perf-parallel-desc-1 = Biblioteca, config, playlists e favoritos carregam todos em paralelo com
perf-parallel-desc-2 = . Abertura quase instantânea.
perf-art-label = Cache de capas
perf-art-desc = Capas extraídas uma vez, salvas em disco. Nunca decodificadas novamente em visualizações repetidas.
perf-lazy-label = Carregamento preguiçoso de imagens
perf-lazy-desc = Centenas de capas em resultados de busca — nenhuma carrega até estar visível.
perf-io-label = I/O não-bloqueante
perf-io-desc-1 = Trabalho pesado roda em threads
perf-io-desc-2 = { " " }. UI permanece responsiva durante escaneamentos completos.
perf-http-label = Cache HTTP de capas
perf-http-desc-1 = Protocolo
perf-http-desc-2 = { " " }personalizado serve capas com cabeçalhos de cache de 1 ano. Webview nunca refaz a requisição.
perf-sort-label = Ordenação mais inteligente
perf-sort-desc-1 = Visualizações da biblioteca ordenam com
perf-sort-desc-2 = { " " }para que as chaves de ordenação sejam calculadas uma vez, não a cada comparação.

## Install
install-title = Instalação
install-nix-title = Nix / NixOS
install-nix-run = Executar sem instalar:
install-nix-profile = Ou adicionar ao seu perfil:
install-nix-note = Flake NixOS suportado com cache binário Cachix.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Instale com seu helper preferido:
install-aur-note-1 = Requer
install-aur-note-2 = { " " }instalado primeiro em uma versão compatível com dioxus 0.7.x.
install-flatpak-title = Flatpak
install-flatpak-desc = Instalar a partir do manifesto fonte:
install-flatpak-note = Listagem no Flathub em breve.
install-appimage-title = AppImage
install-appimage-desc-1 = Baixe do GitHub Releases e execute diretamente. Requer
install-appimage-desc-2 = { " " }e
install-appimage-desc-3 = { " " }no seu sistema.
install-appimage-note-1 = Usuários Arch: se travar com erro WebKitNetworkProcess, execute com prefixo
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-chip = Apple Silicon
install-macos-desc-1 = Baixe o
install-macos-desc-2 = { " " }do GitHub Releases. Se o macOS bloquear, limpe a flag de quarentena:

## Platforms
platforms-title = Baixar Kopuz
platforms-subtitle = Grátis e open source. Todos os releases no GitHub.
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apenas Apple Silicon
platforms-linux = Linux
platforms-download = Baixar →

## Support
support-title = Apoiar Kopuz
support-subtitle = Kopuz é grátis e open source. O apoio o mantém vivo.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-crypto-divider = — ou envie cripto —
support-usdt-note = (rede Solana)

donate-title = Doações em cripto
donate-subtitle = Apoie o desenvolvimento enviando cripto para estes endereços.
donate-solana = Solana: 2fapJYRztnTRLpJbmyEUnsuZ36AzLK2JrMmmLEfDqKpN
donate-bitcoin = Bitcoin: bc1qz94yz9xvufa6hxlvjzaajgd2zyfu86arn68hu4
donate-monero = Monero: 86mz3HxTrKyYpuvx78m6pufbXdwAnoyoZBztz6HyYrnM1XP5YVrMy9jTVRY5vzgGtkizACLpFwHEdafKTMoj6y8mAVgvWMz
donate-ethereum = Ethereum: 0xa490D50470cdFf837B6663F7f6cBe50B157224e5
donate-usdt-sol = USDT (Solana): GYmnAcrA5MbF6cUxT2m5d5cwdfr14qSY9WFYRwXxaibW

## YouTube Music
ytmusic-title = Configuração do YouTube Music
ytmusic-subtitle = Adicione em Configurações → Servidores de mídia → Adicionar → YouTube Music.
ytmusic-token-title = Sem helper necessário
ytmusic-token-desc-1 = A reprodução anônima precisa de um PO token de conteúdo, que o Kopuz agora gera no próprio app com uma WebView oculta rodando o BotGuard do YouTube. O antigo
ytmusic-token-desc-2 = { " " }subprocesso acabou — nada para instalar, e funciona dentro do Flatpak.
ytmusic-signin-title = Faça login com um navegador
ytmusic-signin-desc = O Kopuz abre o login do Google em um perfil de navegador isolado — sua navegação normal nunca é tocada — e extrai os cookies da sessão. Desbloqueia sua biblioteca, Músicas Curtidas, playlists e artistas seguidos.
ytmusic-signin-note = No Windows, o login pelo navegador está atualmente desativado; usuários do Windows recebem o modo anônimo automaticamente. O login funciona no Linux e no macOS.
ytmusic-anon-title = Modo anônimo
ytmusic-anon-desc = Sem login, sem cookies. Navegue, busque, abra páginas de artistas, álbuns e playlists, inicie a rádio mix e reproduza faixas públicas. Curtir e as visualizações da biblioteca ficam desativados.
ytmusic-premium-title = Faixas Premium
ytmusic-premium-desc-1 = Faixas bloqueadas pelo Music Premium recorrem a uma resolução local
ytmusic-premium-desc-2 = { " " }quando o caminho principal retorna UNPLAYABLE, então tê-la instalada ajuda. O modo anônimo não consegue reproduzir conteúdo exclusivo Premium de forma alguma.

## SoundCloud
soundcloud-title = Configuração do SoundCloud
soundcloud-subtitle = Adicione em Configurações → Servidores de mídia → Adicionar → SoundCloud.
soundcloud-signin-title = Login único pelo navegador
soundcloud-signin-desc = Sem URL ou senha para digitar. O Kopuz abre soundcloud.com/signin em um perfil de navegador isolado — sua navegação normal nunca é tocada — e obtém o oauth_token da sessão. Escolha qual navegador da família Chromium usar (Chrome, Chromium, Brave, Edge ou Vivaldi).
soundcloud-features-title = O que você ganha
soundcloud-features-desc = Busca, reprodução de faixas (MP3 progressivo mais streams AAC/HLS do Go+), suas faixas Curtidas como favoritos, playlists somente leitura e curtir/descurtir. Remover a fonte limpa seu perfil isolado.

## Sponsors
sponsors-title = Patrocinadores
sponsors-subtitle = Pessoas mantendo Kopuz funcionando.
sponsors-cta = Tornar-se patrocinador

## Gallery
gallery-page-title = Galeria — Kopuz Music Player
gallery-page-desc = Capturas do Kopuz em ação — início, biblioteca, playlist, reprodutor em tela cheia, letras, editor de temas e mais.
gallery-title = Galeria
gallery-subtitle = Capturas do Kopuz — estilos Normal e Modern.
gallery-home = Início
gallery-home-styles = Normal / Modern
gallery-library = Biblioteca
gallery-library-styles = Normal / Vaxry
gallery-playlist = Playlist
gallery-playlist-styles = Normal / Modern
gallery-fullscreen-title = Reprodutor em tela cheia
gallery-fullscreen-desc = Vista imersiva de reprodução em janela cheia.
gallery-fullscreen-lyrics-title = Letras em tela cheia
gallery-fullscreen-lyrics-desc = Letras sincronizadas em modo janela cheia.
gallery-search-title = Busca
gallery-search-desc = Busca em tempo real entre artistas, álbuns e faixas.
gallery-theme-title = Editor de temas
gallery-theme-desc = Controle total das variáveis de cor. Construa ou escolha um preset.
gallery-settings-title = Configurações do reprodutor
gallery-settings-desc = Configure áudio, comportamento, integrações e mais.
gallery-downloader-title = Baixador
gallery-downloader-desc = Baixe faixas direto de dentro do Kopuz.
gallery-prev = ← Anterior
gallery-next = Próximo →
gallery-label-normal-home = Normal — Início
gallery-label-modern-home = Modern — Início
gallery-label-normal-library = Normal — Biblioteca
gallery-label-vaxry-library = Vaxry — Biblioteca
gallery-label-normal-playlist = Normal — Playlist
gallery-label-modern-playlist = Modern — Playlist
gallery-label-fullscreen = Reprodutor em tela cheia
gallery-label-fullscreen-lyrics = Letras em tela cheia
gallery-label-search = Busca
gallery-label-theme-editor = Editor de temas
gallery-label-player-settings = Configurações do reprodutor
gallery-label-downloader = Baixador

## Footer
footer-license = Licença MIT — Grátis e Open Source
footer-github = GitHub
footer-releases = Releases
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz — Music Player
home-meta-desc = Kopuz é um reprodutor de música moderno e leve construído com Rust e Dioxus. Transmita do Jellyfin ou Navidrome, navegue arquivos locais, letras sincronizadas, equalizador, temas e mais.
home-meta-keywords = Kopuz, reprodutor de música, Rust, Dioxus, Jellyfin, Navidrome, open source, Linux, macOS, Windows, MPRIS, letras
og-title = Kopuz — Music Player
og-desc = Reprodutor de música moderno e leve construído com Rust. Arquivos locais, Jellyfin, Navidrome, letras sincronizadas, equalizador, Discord RPC e mais. Grátis e open source.
og-image-alt = Reprodutor de música Kopuz
twitter-title = Kopuz — Music Player
twitter-desc = Reprodutor de música moderno e leve construído com Rust. Grátis e open source.

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
