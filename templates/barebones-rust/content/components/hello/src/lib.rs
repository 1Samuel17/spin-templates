use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A starter Spin HTTP component.
///
/// This handler responds to all requests on the `/hello/...` route.
/// Replace the body and logic below with your own implementation.
#[http_component]
fn handle_hello(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello World!")
        .build())
}
