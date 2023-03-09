use {
    crate::{app_state::AppState, config::Config},
    async_stream::try_stream,
    axum::{
        extract::{
            ws::{Message, WebSocket, WebSocketUpgrade},
            ConnectInfo, State, TypedHeader,
        },
        headers,
        response::{
            sse::{Event, Sse},
            IntoResponse,
        },
        routing::get,
        Router,
    },
    futures::stream::Stream,
    std::{convert::Infallible, net::SocketAddr, path::PathBuf, sync::Arc},
    tower_http::{services::ServeDir, trace::TraceLayer},
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

pub async fn run(app_state: Arc<AppState>) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_sse=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // build our application with a route
    let app = Router::new()
        .nest_service("/", ServeDir::new(assets_dir))
        .route("/sse", get(sse_handler))
        .route("/ws", get(websocket_handler))
        .route("/config", get(config_handler))
        .layer(TraceLayer::new_for_http())
        // .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(app_state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        // .serve(app.into_make_service())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn config_handler(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    axum::response::Json(app_state.config.clone())
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    State(app_state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let mut receiver = app_state.tx.subscribe();

    // A `Stream` that receives messages from the `broadcast::Receiver`
    // Event::default().json_data("Hello, World!").unwrap();
    let stream = try_stream! {
        loop {
            match receiver.recv().await {
                Ok(i) => {
                    println!("Received: {}", i);
                    let event = Event::default().data(i);
                    yield event;
                },
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to get");
                }
            }
        }
    };

    Sse::new(stream)
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket(socket, state, addr))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn one independent tasks (for
// sending chat messages).
async fn websocket(mut stream: WebSocket, state: Arc<AppState>, who: SocketAddr) {
    // Ping the client to make sure the connection is alive.
    if stream.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {}...", who);
    } else {
        println!("Could not send ping {}!", who);
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // Subscribe to the broadcast channel to receive events.
    let mut rx = state.tx.subscribe();

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut _send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if stream.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
}
