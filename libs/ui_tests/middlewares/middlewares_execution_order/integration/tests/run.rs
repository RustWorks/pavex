use std::future::IntoFuture;
use std::net::TcpListener;

use app::Spy;
use application::{ApplicationState, ApplicationConfig, run};

async fn spawn_test_server(spy: Spy) -> u16 {
    static TELEMETRY: std::sync::Once = std::sync::Once::new();
    TELEMETRY.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
            .init();
    });

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to listen on a random port");
    let port = listener
        .local_addr()
        .expect("Failed to get local address")
        .port();
    let incoming_stream: pavex::server::IncomingStream =
        listener.try_into().expect("Failed to convert listener");
    let server = pavex::server::Server::new().listen(incoming_stream);
    let application_state = ApplicationState::new(ApplicationConfig {}, spy).await.unwrap();
    tokio::task::spawn(run(server, application_state).into_future());
    port
}

#[tokio::test]
async fn top_level_mw_execute_in_order() {
    let state = Spy::new();
    let port = spawn_test_server(state.clone()).await;

    reqwest::get(&format!("http://localhost:{}/top_level", port))
        .await
        .expect("Failed to make request")
        .error_for_status()
        .expect("Failed to get successful response");

    let state = state.get().await;
    assert_eq!(
        state,
        vec![
            "first - start",
            "second - start",
            "first_pre",
            "second_pre",
            "handler",
            "first_post",
            "second_post",
            "second - end",
            "first - end"
        ]
    );
}

#[tokio::test]
async fn mw_registered_after_handler_does_not_wrap_handler() {
    let state = Spy::new();
    let port = spawn_test_server(state.clone()).await;

    reqwest::get(&format!("http://localhost:{}/after_handler", port))
        .await
        .expect("Failed to make request")
        .error_for_status()
        .expect("Failed to get successful response");

    let state = state.get().await;
    assert_eq!(
        state,
        vec![
            "first - start",
            "first_pre",
            "handler",
            "first_post",
            "first - end"
        ]
    );
}

#[tokio::test]
async fn order_is_preserved_with_nesting() {
    let state = Spy::new();
    let port = spawn_test_server(state.clone()).await;

    reqwest::get(&format!("http://localhost:{}/nested", port))
        .await
        .expect("Failed to make request")
        .error_for_status()
        .expect("Failed to get successful response");

    let state = state.get().await;
    assert_eq!(
        state,
        vec![
            "first - start",
            "first_pre",
            "second - start",
            "second_pre",
            "handler",
            "second_post",
            "second - end",
            "first_post",
            "first - end"
        ]
    );
}

#[tokio::test]
async fn pre_processing_mw_can_early_return_but_all_post_processing_is_invoked() {
    let state = Spy::new();
    let port = spawn_test_server(state.clone()).await;

    reqwest::get(&format!("http://localhost:{}/early_return", port))
        .await
        .expect("Failed to make request")
        .error_for_status()
        .expect("Failed to get successful response");

    let state = state.get().await;
    assert_eq!(
        state,
        vec!["first - start", "early_return_pre", "first_post", "first - end"]
    );
}

#[tokio::test]
async fn pre_processing_mw_can_fail_but_all_post_processing_is_invoked() {
    let state = Spy::new();
    let port = spawn_test_server(state.clone()).await;

    let response = reqwest::get(&format!("http://localhost:{}/failing_pre", port))
        .await
        .expect("Failed to make request");
    assert_eq!(response.status().as_u16(), 500);

    let state = state.get().await;
    assert_eq!(
        state,
        vec!["first - start", "failing_pre", "first_post", "first - end"]
    );
}
