## Language

## Nav
nav-features = Recursos
nav-download = Baixar
nav-lang-label = Idioma

## Hero
hero-cta-download = Baixar
hero-cta-github = Ver no GitHub
hero-screenshot-alt = Tela inicial do Kopuz

## Features
features-title = Recursos
features-source-local = Arquivos locais
features-source-jellyfin = Jellyfin
features-source-navidrome = Navidrome
features-source-subsonic = Subsonic API
features-source-nextcloud = Nextcloud
features-source-ytmusic = YouTube Music
features-source-applemusic = Apple Music
features-source-soundcloud = SoundCloud

feat-theming-title = Temas
feat-native-title = Integração nativa
feat-android-title = Android
feat-android-desc = Uma versão ARM64 adaptada para celulares, com gestos de deslizar, controles de reprodução aleatória e repetição na notificação de mídia e modo AMOLED, para Android 7.0 ou mais recente.
feat-lyrics-title = Letras sincronizadas
feat-lyrics-desc = Letras sincronizadas por linha, palavra por palavra ou sem marcação de tempo, com pausas instrumentais, linha ativa centralizada, desfoque de profundidade e ajuste manual ou automático da sincronização.
feat-eq-title = Equalizador
feat-eq-desc = Equalizador de 5 bandas com presets integrados e controle total sobre seu som.
feat-fav-title = Favoritos e playlists
feat-scrobble-title = Scrobbling
feat-scrobble-desc = Scrobbling ListenBrainz integrado. Usuários do Jellyfin também podem usar o plugin listenbrainz.
feat-radio-title = Rádio na internet
feat-radio-desc = Ouça estações da internet por meio de diretórios de rádios, com metadados da transmissão e da faixa em reprodução.
feat-offline-title = Downloads para ouvir offline
feat-offline-desc = Baixe faixas remotas compatíveis para ouvir sem conexão com a internet.
feat-fonts-title = Fontes personalizadas da interface
feat-fonts-desc = Escolha sua própria fonte para a interface junto com os temas integrados.
feat-discord-title = Discord RPC
feat-discord-desc = Mostre aos amigos o que você está ouvindo com Discord Rich Presence embarcado. Sem configuração.
feat-search-title = Busca
feat-search-desc = Busca em tempo real entre artistas, álbuns e faixas. Resultados instantâneos enquanto digita.
feat-genre-title = Navegação por gênero
feat-genre-desc = Navegue por toda a sua biblioteca por gênero. Funciona com arquivos locais e músicas de servidores.
feat-logs-title = Logs de escuta
feat-logs-desc = Contagens de reprodução rastreadas localmente. Veja o que você realmente escuta mais ao longo do tempo.
feat-i18n-title = i18n
feat-channels-title = Modos de canal
feat-channels-desc = Estéreo, Mono, Só-Esquerdo, Só-Direito e troca L/R. Controle fino dos canais de áudio.
feat-youtube-title = YouTube Music
feat-applemusic-title = Apple Music
feat-applemusic-desc = Navegue pelo catálogo e pela sua biblioteca, sincronize favoritos, adicione ou remova faixas de playlists, inicie rádios, baixe faixas e use letras com marcação por palavra. A reprodução no desktop requer Widevine.
feat-nextcloud-title = Nextcloud
feat-nextcloud-desc = Conecte-se via WebDAV, escolha pastas da biblioteca para cada servidor, transmita ou baixe faixas, armazene capas em cache e determine a duração pelos cabeçalhos dos arquivos.
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
feat-badges-desc = Faixas locais exibem um pequeno selo com o formato, como MP3, FLAC ou WAV, diretamente na linha da faixa.

## Novidades

## Escolha de download

## Performance
perf-skip-label = Pular já indexados
perf-skip-desc = Reescaneamentos só processam arquivos novos. 10k faixas + 5 novas = só 5 lidas.
perf-parallel-label = Início paralelo
perf-art-label = Cache de capas
perf-art-desc = Capas extraídas uma vez, salvas em disco. Nunca decodificadas novamente em visualizações repetidas.
perf-lazy-label = Carregamento preguiçoso de imagens
perf-lazy-desc = Centenas de capas podem aparecer nos resultados de busca, mas só são carregadas quando ficam visíveis.
perf-io-label = I/O não-bloqueante
perf-http-label = Cache HTTP de capas
perf-sort-label = Ordenação mais inteligente

## Install
install-nix-title = Nix / NixOS
install-nix-run = Executar sem instalar:
install-nix-profile = Ou adicionar ao seu perfil:
install-nix-note = Flake NixOS suportado com cache binário Cachix.
install-aur-title = AUR (Arch Linux)
install-aur-desc = Instale com seu helper preferido:
install-flatpak-title = Flatpak
install-flatpak-desc = Instale o Kopuz a partir do Flathub:
install-flatpak-note = As atualizações chegam pelo fluxo normal de atualização do Flatpak.
install-appimage-title = AppImage
install-appimage-desc-1 = Baixe do GitHub Releases e execute diretamente. Requer
install-appimage-desc-2 = { " " }e
install-appimage-desc-3 = { " " }no seu sistema.
install-appimage-note-1 = Usuários Arch: se travar com erro WebKitNetworkProcess, execute com prefixo
install-appimage-note-2 = .
install-macos-title = macOS
install-macos-desc-1 = Baixe o
install-macos-desc-2 = { " " }do GitHub Releases. Se o macOS bloquear, limpe a flag de quarentena:

## Platforms
platforms-title = Baixar Kopuz
platforms-windows = Windows
platforms-macos = macOS
platforms-macos-note = Apenas Apple Silicon
platforms-linux = Linux

## Support
support-title = Apoiar Kopuz
support-subtitle = O Kopuz é grátis e de código aberto. As doações apoiam o desenvolvimento.
support-gh = GitHub Sponsors
support-bmc = Buy Me a Coffee
support-usdt-note = (rede Solana)


## Nextcloud
nextcloud-subtitle = Adicione em Configurações → Servidores de mídia → Adicionar → Nextcloud.
nextcloud-connect-title = Use uma senha de aplicativo
nextcloud-connect-desc = Insira a URL base do Nextcloud, seu nome de usuário e uma senha de aplicativo revogável criada em Configurações do Nextcloud → Segurança.
nextcloud-folders-title = Escolha as pastas da biblioteca
nextcloud-folders-desc = Escolha uma ou mais pastas para cada fonte. O Kopuz lê a árvore de pastas do WebDAV, armazena as capas disponíveis em cache e verifica os cabeçalhos dos arquivos para determinar a duração das faixas.
nextcloud-webdav-title = Funcionamento com WebDAV puro
nextcloud-webdav-desc = A biblioteca segue a estrutura de pastas e não oferece playlists nem rádio. Se o Nextcloud Music disponibilizar o Subsonic, use essa fonte para obter metadados mais completos e playlists.
nextcloud-playback-title = Transmita ou baixe
nextcloud-playback-desc = Reproduza os arquivos diretamente do seu servidor ou baixe-os para ouvir offline, sem que o áudio passe pela infraestrutura do Kopuz.

## YouTube Music
ytmusic-subtitle = Adicione em Configurações → Servidores de mídia → Adicionar → YouTube Music.
ytmusic-token-title = Sem helper necessário
ytmusic-signin-title = Faça login com um navegador
ytmusic-signin-desc = O Kopuz abre o login do Google em um perfil de navegador isolado e extrai os cookies da sessão. O login libera sua biblioteca, Músicas Curtidas, playlists e artistas seguidos.
ytmusic-anon-title = Modo anônimo
ytmusic-anon-desc = Sem login, sem cookies. Navegue, busque, abra páginas de artistas, álbuns e playlists, inicie a rádio mix e reproduza faixas públicas. Curtir e as visualizações da biblioteca ficam desativados.

## Apple Music
applemusic-subtitle = Adicione em Configurações → Servidores de mídia → Adicionar → Apple Music.
applemusic-signin-title = Faça login ou cole um token
applemusic-signin-desc = No desktop, o Kopuz abre o Apple Music em um perfil isolado de um navegador da família Chromium. Como alternativa, você pode colar manualmente um media-user-token.
applemusic-playback-title = Widevine no desktop
applemusic-playback-desc = A reprodução do Apple Music requer um CDM Widevine de um navegador instalado ou do serviço de plugins da Mozilla. O Kopuz não inclui o CDM.
applemusic-features-title = O que você ganha
applemusic-features-desc = Navegação pelo catálogo e pela biblioteca, sincronização de favoritos, adição e remoção de faixas em playlists, rádio de faixas e playlists, downloads e letras com marcação por palavra.
applemusic-android-title = Situação no Android
applemusic-android-desc = O login usa uma WebView integrada ao app, mas a reprodução do Apple Music ainda não é suportada no Android.

## SoundCloud
soundcloud-subtitle = Adicione em Configurações → Servidores de mídia → Adicionar → SoundCloud.
soundcloud-signin-title = Login único pelo navegador
soundcloud-features-title = O que você ganha
soundcloud-features-desc = Busca, reprodução de faixas (MP3 progressivo mais streams AAC/HLS do Go+), suas faixas Curtidas como favoritos, playlists somente leitura e curtir/descurtir. Remover a fonte limpa seu perfil isolado.

## Guias de recursos
guides-title = Guias
## Privacidade e armazenamento

## Requisitos

## Comunidade
community-title = Contribua com o Kopuz

## Sponsors
sponsors-cta = Tornar-se patrocinador

## Gallery

## Footer
footer-license = Licença EUPL v1.2. Grátis e de código aberto.
footer-github = GitHub
footer-releases = Releases
footer-issues = Issues
footer-discord = Discord

## Page meta
home-title = Kopuz: player de música gratuito e de código aberto
og-title = Kopuz: player de música gratuito e de código aberto
og-image-alt = Reprodutor de música Kopuz
twitter-title = Kopuz: player de música gratuito e de código aberto
## User-friendly aliases (synced with EN updates)
perf-parallel-desc = { perf-parallel-desc-1 } { perf-parallel-desc-2 }
perf-io-desc = { perf-io-desc-1 } { perf-io-desc-2 }
perf-http-desc = { perf-http-desc-1 } { perf-http-desc-2 }
perf-sort-desc = { perf-sort-desc-1 } { perf-sort-desc-2 }

## Deep link (/j)
join-title = A abrir o Kopuz…
join-opening = A abrir o Kopuz…
join-fallback = O Kopuz não abriu. Ainda não o tens?
join-no-payload = Esta ligação não traz nenhuma fila.
join-download = Transferir o Kopuz

## Privacy policy
footer-privacy = Privacidade
privacy-english-note = Esta política é mantida em inglês para que a sua redação continue exata. As traduções do resto do site não alteram o que ela diz.
privacy-back = Voltar ao início

## Added strings
hero-title-1 = Um player para
hero-title-2 = toda a sua música.
hero-desc = O Kopuz reúne seus arquivos locais, seu NAS, seu próprio servidor e suas contas de streaming em uma única biblioteca. É gratuito e de código aberto.
features-source-spotify = Spotify
feat-local-title = Pastas e servidores
feat-local-desc = Adicione pastas locais ou conecte um servidor Jellyfin ou Subsonic, como o Navidrome. As faixas de cada fonte que você adiciona aparecem em uma única biblioteca, e o Subsonic mantém sua fila sincronizada entre os clientes.
feat-theming-desc = Escolha um dos 24 temas claros e escuros incluídos, ou crie o seu e defina cada cor.
feat-native-desc = Controle a reprodução pelas teclas e pelos controles de mídia do sistema: MPRIS no Linux, Now Playing no macOS e a sobreposição de mídia no Windows.
feat-fav-desc = Guarde seus favoritos no dispositivo ou sincronize com o servidor. Crie playlists, adicione álbuns inteiros de uma vez e sincronize as playlists com o servidor.
feat-i18n-desc = Use o Kopuz em qualquer um dos 28 idiomas.
feat-ytdlp-title = Downloader do YouTube
feat-ytdlp-desc = Cole um link do YouTube ou do YouTube Music de uma música, álbum ou playlist. O Kopuz baixa com seu próprio cliente do YouTube, adiciona título, artista, álbum e capa e organiza por álbum. A melhor qualidade de áudio mantém o stream original e não exige nada instalado. Para converter para MP3, FLAC ou WAV, instale o ffmpeg.
feat-crossfade-title = Crossfade e reprodução sem pausas
feat-crossfade-desc = Faça uma transição suave entre as faixas na reprodução no desktop, ou desative o crossfade para álbuns sem pausas.
feat-youtube-desc = Navegue, pesquise e inicie um rádio mix a partir de qualquer faixa. O Descobrir sugere músicas, álbuns, artistas e estilos, e as páginas de artista mostram as principais músicas, lançamentos e artistas relacionados. Entre para ver sua biblioteca, as Músicas curtidas e suas playlists, ou navegue sem conta.
feat-spotify-title = Spotify
feat-spotify-desc = Navegue por suas faixas salvas, álbuns, playlists e pelo Descobrir, pesquise e faça scrobble. Reproduza pelo Web Playback SDK do Spotify ou em qualquer um dos seus dispositivos Spotify Connect. Você precisa do Spotify Premium para reproduzir música.
new-title = Novidades do Kopuz
install-cargo-title = Cargo (crates.io)
install-cargo-desc = Instale a versão mais recente com o Cargo:
install-macos-homebrew = Instale a versão para Apple silicon pelo tap do Homebrew:
install-android-title = Android
install-android-desc = Baixe o Kopuz no Google Play ou baixe o APK arm64-v8a assinado no GitHub Releases. Você precisa do Android 7.0 (API 24) ou posterior.
install-android-note = Se o APK instalado veio de outra fonte, desinstale-o antes de instalar este.
ytmusic-token-desc = Você pode reproduzir sem entrar e não precisa instalar nenhum programa auxiliar.
ytmusic-signin-note = O Kopuz usa seu navegador padrão. Você pode escolher Firefox, Chrome, Chromium, Brave, Edge, Vivaldi, Helium, LibreWolf, Zen ou Floorp.
ytmusic-premium-title = Qualidade Premium
ytmusic-premium-desc = Com uma conta Music Premium, o Kopuz reproduz e baixa os streams de maior qualidade com seu próprio cliente do YouTube. O modo anônimo não reproduz faixas exclusivas do Premium.
soundcloud-signin-desc = Você não precisa de URL, senha nem token. O Kopuz abre o login do SoundCloud em um perfil de navegador separado e exclui esse perfil quando você remove a fonte.
spotify-guide-subtitle = O Kopuz usa a Web API oficial do Spotify e a reprodução no navegador. Ele nunca vê sua senha nem retransmite o áudio.
spotify-step-1-title = Crie um aplicativo do Spotify
spotify-step-1-desc = No painel de desenvolvedores do Spotify, crie um aplicativo e ative Web API e Web Playback SDK. Em User Management, adicione cada conta que vai ouvir (até cinco no Development Mode). Depois, defina exatamente este URI de redirecionamento:
spotify-step-2-title = Adicione seu Client ID
spotify-step-2-desc = Em Configurações → Servidores de mídia → Adicionar → Spotify, cole o Client ID e aprove o acesso. Você não precisa de um Client Secret. A porta 8898 precisa estar livre enquanto você entra.
spotify-step-3-title = Escolha onde reproduzir
spotify-step-3-desc = Mantenha aberta a guia do player do aplicativo ou escolha um dos seus dispositivos Spotify Connect no menu de dispositivos.
spotify-requirement = Você precisa do Spotify Premium para reproduzir música. Use Chrome, Edge, Brave, Chromium, Vivaldi, Helium ou Safari no macOS. O Firefox não funciona.
spotify-full-guide = Leia o guia completo de configuração e solução de problemas do Spotify
privacy-local-title = Um banco de dados local
privacy-local-desc = O Kopuz guarda sua biblioteca, playlists, favoritos, contagens de reprodução e dados das fontes em kopuz.db, e suas preferências ao lado, em settings.toml.
privacy-accounts-title = Logins separados
privacy-accounts-desc = Cada fonte guarda seu próprio login e seus próprios favoritos. Os logins pelo navegador usam perfis separados, não a sua sessão habitual do navegador.
privacy-files-title = Cache e logs
privacy-files-desc = Capas de álbum e faixas offline ficam na pasta de cache do sistema. Você pode abrir ou exportar os logs nas Configurações.
requirements-title = Requisitos e limitações
about-title = O que é um kopuz?
about-desc-1 = O kopuz é um antigo instrumento de cordas turcomano que bardos e xamãs tocavam. Acredita-se que seja ancestral de vários alaúdes da Ásia Central.
about-desc-2 = Na lenda turca, ele pertence a Dede Korkut, um bardo. O komuz quirguiz e o kobyz cazaque são instrumentos aparentados. O xomus, apesar do nome parecido, não é.
community-issues-title = Issues
community-issues-desc = Relate um bug ou assuma uma tarefa aberta.
community-discussions-title = Discussões
community-discussions-desc = Faça perguntas e sugira mudanças.
community-discord-desc = Converse com usuários e colaboradores.
community-contribute-title = Contribuir
community-contribute-desc = Obtenha o código-fonte, configure um ambiente de desenvolvimento ou ajude a traduzir.
platforms-android = Android
platforms-android-note = ARM64 · Android 7.0+ · API 24+
home-meta-desc = O Kopuz é um player de música gratuito para desktop e Android. Toque arquivos locais, SMB, Jellyfin, YouTube Music, Spotify e mais em uma só biblioteca.
home-meta-keywords = Kopuz, player de música, Jellyfin, Navidrome, Subsonic, SMB, NAS, Clippsly, Nextcloud, YouTube Music, Apple Music, SoundCloud, Spotify, código aberto, Android, Linux, macOS, Windows, letras, equalizador
og-desc = Um player de música gratuito para arquivos locais, servidores e serviços de streaming no desktop e no Android, com letras sincronizadas, temas e equalizador de 10 bandas.
twitter-desc = Reproduza arquivos locais, compartilhamentos SMB, Jellyfin, Subsonic, YouTube Music, SoundCloud, Spotify e mais em uma única biblioteca.
nav-home = Início
footer-license-line = EUPL 1.2, gratuito e de código aberto
shelf-link-to-us = Faça um link para nós com um botão 88×31
hero-release = Versão mais recente
hero-cta-download-for = Baixar para
hero-cta-all = Todas as plataformas
home-sources-title = Reproduza música de
source-local = Pastas locais
source-navidrome = Navidrome e Subsonic
source-radio = Rádio na internet
moment-lyrics-title = Letras que acompanham o ritmo
moment-lyrics-alt = O Kopuz reproduzindo uma faixa com letras sincronizadas em tela cheia
moment-library-title = Uma biblioteca para todas as fontes
moment-library-alt = A biblioteca do Kopuz com álbuns de várias fontes
moment-themes-title = Suas cores, ou as do seu papel de parede
moment-themes-alt = O editor de temas do Kopuz
moment-themes-extra = Ou deixe o Kopuz criar um tema a partir do papel de parede da sua área de trabalho.
theme-default = Gruvbox dark soft
theme-amoled = AMOLED
theme-wallpaper = Papel de parede
wallpaper-sunset = Pôr do sol
wallpaper-lake = Lago
home-see-all-features = Ver todos os recursos
home-get-title = Obtenha o Kopuz
home-get-download = Baixar
home-also-on = Você também pode instalar o Kopuz com Cargo, Nix, AUR, Flatpak ou Homebrew.
home-sponsors-title = Patrocinadores
home-sponsor-cta = Patrocinar no GitHub
copy = Copiar
copied = Copiado
copy-address = Copiar endereço de { $coin }
nav-primary-aria = Navegação principal
nav-menu-open = Abrir menu
nav-menu-close = Fechar menu
footer-nav-aria = Rodapé
theme-use-light = Usar tema claro
theme-use-dark = Usar tema escuro
theme-leave-moe = Sair do modo moe
mode-simple = Site simples
mode-full = Site completo
webbutton-alt = Botão web do Kopuz 88x31
features-page-title = Recursos do Kopuz: fontes, letras, temas e reprodução
download-page-title = Baixar o Kopuz para Windows, macOS, Linux e Android
guides-page-title = Guias de configuração do Kopuz para servidores e streaming
support-page-title = Apoie o Kopuz: patrocinadores, doações e comunidade
notfound-page-title = Página não encontrada | Kopuz
guide-title-suffix = guia de configuração | Kopuz
notfound-title = Página não encontrada.
notfound-desc = Nada está tocando neste endereço.
notfound-home = Ir para a página inicial
features-group-sources = Fontes
features-group-playback = Reprodução
features-group-library = Biblioteca
features-group-desktop = Desktop
features-group-android = Android
features-group-integrations = Integrações
features-group-performance = Desempenho
features-group-privacy = Privacidade
features-jump-aria = Grupos de recursos
features-privacy-link = Leia a política de privacidade
install-other-title = Outras formas de instalar
req-spotify = Você precisa do Premium, do seu próprio Client ID e de um navegador compatível para reproduzir música.
req-applemusic = A reprodução no desktop exige um Widevine CDM. Você pode entrar no Android, mas a reprodução ainda não funciona lá.
req-nextcloud = O WebDAV simples não oferece suporte a playlists nem a rádio. Se o Nextcloud Music oferecer um endpoint Subsonic, use esse.
req-android = Os APKs de versão exigem um celular arm64-v8a com Android 7.0 (API 24) ou posterior. O status do Discord e a bandeja do sistema não estão disponíveis.
req-appimage = Exige webkit2gtk-4.1 e GTK 3. A bandeja também exige uma biblioteca appindicator.
req-ytmusic = O modo anônimo não reproduz faixas exclusivas do Premium. Entre com uma conta Music Premium para reproduzi-las.
req-crossfade = O crossfade e a reprodução sem pausas funcionam com a reprodução nativa no desktop, não com o áudio do Spotify reproduzido no navegador.
req-spotify-limits-label = Limites do Spotify
req-spotify-limits = No Development Mode, um aplicativo permite até cinco ouvintes, a pesquisa retorna até dez resultados por tipo, as playlists são somente leitura, e downloads, rádio, edição de tags, equalizador e crossfade ficam desativados.
support-note = Sou o Temidaradev e desenvolvo o Kopuz. Sou estudante e não tenho renda fixa, então os patrocinadores são o que me permite continuar trabalhando no Kopuz enquanto estudo.
support-goal-label = Meta do GitHub Sponsors
support-goal-amount = ${ $current } de ${ $goal } por mês
support-goal-aria = Progresso da meta do GitHub Sponsors
support-crypto-summary = Endereços de criptomoedas
sponsors-tier-special = Especial
sponsors-tier-monthly = Mensal
sponsors-tier-one-time = Único
features-source-smb = SMB e NAS
features-source-clippsly = Clippsly
feat-smb-title = SMB e NAS
feat-smb-desc = Adicione um compartilhamento SMB 2 ou 3, ou uma pasta dentro de um, como biblioteca. O Kopuz lê as tags incorporadas e as capas e reproduz direto do compartilhamento, com busca na faixa, então você não precisa montá-lo.
feat-clippsly-title = Clippsly
feat-clippsly-desc = Entre com seu nome de usuário do Clippsly e uma senha de aplicativo de clippsly.app. Faça streaming e download do seu locker, envie arquivos de áudio e confira o uso do seu armazenamento. As playlists remotas ainda não são compatíveis.
feat-updates-title = Atualizações automáticas
feat-updates-desc = No desktop, um banner avisa quando há uma versão nova, e o Kopuz baixa e instala para você.
feat-cli-title = Daemon e linha de comando
feat-cli-desc = O player roda como um daemon, o kopuzd, ao qual a janela se conecta. Use o kopuzctl para controlar a reprodução, configurar fontes e iniciar varreduras da biblioteca em um terminal ou script, ou fale direto com a API gRPC.
install-android-play = Baixe no Google Play
footer-made-with = Site feito com
footer-love = amor
footer-made-by = por
source-smb = SMB e NAS
source-clippsly = Clippsly
player-prev-label = Seção anterior
player-next-label = Próxima seção
player-seek-label = Posição na página
player-queue-label = Seções desta página
player-queue-title = Nesta página
dl-file-exe = Instalador (.exe)
dl-file-msi = Instalador (.msi)
dl-file-dmg = Imagem de disco (.dmg)
dl-file-appimage = AppImage
dl-file-deb = Debian e Ubuntu (.deb)
dl-file-rpm = Fedora e RHEL (.rpm)
dl-file-apk = APK
dl-release-page = Página da versão
dl-this-device = Este dispositivo
dl-dialog-ready = Versão mais recente: { $version }. Escolha o instalador para o seu dispositivo.
dl-dialog-close = Fechar
dl-dialog-release = Ver a versão no GitHub
guide-nextcloud-summary = Conecte por WebDAV com uma senha de aplicativo e escolha as pastas da sua biblioteca.
guide-ytmusic-summary = Entre pelo navegador ou ouça sem conta.
guide-applemusic-summary = Entre, faça o Widevine funcionar no desktop e veja o que funciona no Android.
guide-soundcloud-summary = Entre uma vez no navegador e veja o que a fonte oferece.
guide-spotify-summary = Crie um aplicativo de desenvolvedor, adicione o Client ID e escolha onde reproduzir.
chapter-home = Início
chapter-sources = Fontes
chapter-lyrics = Letras
chapter-library = Biblioteca
chapter-themes = Temas
chapter-download = Baixar
chapter-sponsors = Patrocinadores
chapter-whats-new = Novidades
chapter-name = O nome
chapter-not-found = Não encontrado
chapter-privacy = Privacidade
chapter-install = Instalar
chapter-requirements = Requisitos
chapter-support = Apoio
chapter-community = Comunidade
chapter-features = Recursos
chapter-guides = Guias
guide-smb-summary = Aponte o Kopuz para um compartilhamento do seu NAS e entre com a conta do NAS.
smb-subtitle = Escolha SMB / NAS no seletor de fontes. Você não precisa montar o compartilhamento antes.
smb-share-title = Aponte para um compartilhamento
smb-share-desc = Digite a URL de um compartilhamento, ou de uma pasta dentro de um, por exemplo:
smb-share-port = Se o seu NAS usa uma porta diferente da padrão, adicione-a à URL:
smb-signin-title = Entre com a conta do NAS
smb-signin-desc = Use o nome de usuário e a senha do seu NAS. Para uma conta de domínio, digite DOMAIN\username. O Kopuz exige SMB 2 ou 3 e uma conta: ele não procura compartilhamentos na rede nem entra como convidado.
smb-library-title = Como a biblioteca funciona
smb-library-desc = O Kopuz varre o compartilhamento e suas subpastas em busca de arquivos de áudio e lê as tags incorporadas e as capas. Ele reproduz direto do compartilhamento, com busca na faixa, também no Android. Seus favoritos ficam guardados no Kopuz.
smb-limits-title = O compartilhamento fica somente leitura
smb-limits-desc = O Kopuz não altera nada no compartilhamento, então você não pode editar tags nele, manter playlists nele nem baixar as faixas para ouvir offline.
guide-clippsly-summary = Entre com uma senha de aplicativo, faça streaming do seu locker e envie músicas.
clippsly-subtitle = Adicione o Clippsly nas configurações de fontes.
clippsly-signin-title = Entre com uma senha de aplicativo
clippsly-signin-desc = Crie uma senha de aplicativo em clippsly.app. Depois, entre no Kopuz com seu nome de usuário do Clippsly e essa senha de aplicativo.
clippsly-locker-title = Seu locker
clippsly-locker-desc = Navegue e faça streaming das músicas do seu locker do Clippsly, e baixe faixas para ouvir offline.
clippsly-upload-title = Envie músicas
clippsly-upload-desc = Envie arquivos de áudio pelo cartão de configurações da fonte. O Kopuz atualiza sua biblioteca quando o lote inteiro termina. O cartão também mostra quanto armazenamento você usa; o Clippsly define sua cota.
clippsly-bonus = Vincule o Kopuz à sua conta do Clippsly e o Clippsly adiciona 3 GB de armazenamento de graça.
clippsly-limits-title = Playlists
clippsly-limits-desc = As playlists que você guarda no Clippsly ainda não aparecem no Kopuz.
clippsly-site = Ir para clippsly.app
