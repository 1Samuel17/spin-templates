use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

const INDEX_HTML: &str = include_str!("../../../assets/index.html");

#[http_component]
fn handle_api_explorer(_req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(INDEX_HTML)
        .build())
}
