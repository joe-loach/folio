mod markup;

use axum::{
    Router,
    extract::Request,
    http::{self, HeaderValue, StatusCode, header::VARY},
    middleware::Next,
    response::Response,
    routing::get,
};
use axum_htmx::{HX_REQUEST, HxBoosted, HxRequest};
use markup::page_layout;
use tower_service::Service as _;

async fn auto_vary(req: Request, next: Next) -> Result<Response, StatusCode> {
    let is_hx_request = req.headers().contains_key(HX_REQUEST);

    let mut response = next.run(req).await;

    if is_hx_request {
        response
            .headers_mut()
            .append(VARY, HeaderValue::from_name(HX_REQUEST));
    }

    Ok(response)
}

fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(
                |HxRequest(hx): HxRequest, HxBoosted(boosted): HxBoosted| async move {
                    page_layout(None, markup::home::page(), hx | boosted)
                },
            ),
        )
        .nest("/projects", markup::projects::router())
        .layer(axum::middleware::from_fn(auto_vary))
}

#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    _env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    Ok(router().call(req).await?)
}
