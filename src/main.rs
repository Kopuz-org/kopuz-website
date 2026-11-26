#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body,
        extract::{FromRef, Request},
        http::{header, HeaderValue, StatusCode},
        middleware::{self, Next},
        response::Response,
        routing::post,
        Router,
    };
    use kopuz_website::app::*;
    use kopuz_website::sponsors::{self, SponsorsState, WebhookConfig};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::path::PathBuf;
    use std::sync::Arc;

    #[derive(Clone)]
    struct AppState {
        leptos_options: LeptosOptions,
        sponsors: SponsorsState,
        webhook: WebhookConfig,
    }

    impl FromRef<AppState> for LeptosOptions {
        fn from_ref(state: &AppState) -> Self {
            state.leptos_options.clone()
        }
    }

    impl FromRef<AppState> for SponsorsState {
        fn from_ref(state: &AppState) -> Self {
            state.sponsors.clone()
        }
    }

    impl FromRef<AppState> for WebhookConfig {
        fn from_ref(state: &AppState) -> Self {
            state.webhook.clone()
        }
    }

    // The js/wasm bundle keeps a fixed filename across deploys, so browsers
    // must revalidate it each visit or they hydrate fresh HTML with a stale
    // bundle (which silently breaks all client-side interactivity).
    async fn pkg_no_cache(req: Request, next: Next) -> Response {
        let is_pkg = req.uri().path().starts_with("/pkg/");
        let mut res = next.run(req).await;
        if is_pkg {
            res.headers_mut()
                .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        }
        res
    }

    // Routes are declared without a trailing slash and would 404 with one, so
    // send `/features/` to `/features` instead of leaving a dead duplicate.
    async fn strip_trailing_slash(req: Request, next: Next) -> Response {
        let path = req.uri().path();
        // `//host/` would trim to a protocol-relative redirect off the site.
        if path.len() > 1 && path.ends_with('/') && !path.starts_with("//") {
            let mut location = path.trim_end_matches('/').to_owned();
            if let Some(query) = req.uri().query() {
                location.push('?');
                location.push_str(query);
            }
            if let Ok(location) = HeaderValue::from_str(&location) {
                return Response::builder()
                    .status(StatusCode::MOVED_PERMANENTLY)
                    .header(header::LOCATION, location)
                    .body(Body::empty())
                    .unwrap_or_default();
            }
        }
        next.run(req).await
    }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let state_path: PathBuf = std::env::var("SPONSORS_STATE_PATH")
        .unwrap_or_else(|_| "sponsors_state.json".to_string())
        .into();
    let webhook_secret = std::env::var("GITHUB_SPONSORS_WEBHOOK_SECRET").unwrap_or_default();
    if webhook_secret.is_empty() {
        log!("GITHUB_SPONSORS_WEBHOOK_SECRET is not set; the sponsors webhook endpoint will reject all events");
    }

    let sponsors_state = sponsors::load_or_bootstrap(&state_path).await;

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        sponsors: sponsors_state.clone(),
        webhook: WebhookConfig {
            secret: Arc::new(webhook_secret),
            state_path: Arc::new(state_path),
        },
    };

    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let sponsors_state = sponsors_state.clone();
                move || provide_context(sponsors_state.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .route("/webhooks/github-sponsors", post(sponsors::webhook_handler))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(middleware::from_fn(pkg_no_cache))
        .layer(middleware::from_fn(strip_trailing_slash))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
