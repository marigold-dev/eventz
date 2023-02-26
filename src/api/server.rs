use axum::{headers, response::Html};

use {
    axum::{
        extract::TypedHeader,
        response::sse::{Event, Sse},
        routing::get,
        Router,
    },
    futures::stream::{self, Stream},
    std::{convert::Infallible, net::SocketAddr, time::Duration},
    tokio_stream::StreamExt as _,
    tower_http::trace::TraceLayer,
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

pub async fn run() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_sse=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/sse", get(sse_handler))
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
