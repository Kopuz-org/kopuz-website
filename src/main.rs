
#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{extract::FromRef, routing::post, Router};
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
