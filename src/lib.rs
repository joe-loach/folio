mod markup;

use axum::{
    Router,
    extract::Request,
    http::{self, HeaderValue, StatusCode, header::VARY},
    middleware::Next,
    response::{Redirect, Response},
    routing::{any, get},
};
use axum_htmx::{HX_REQUEST, HxBoosted, HxRequest};
use markup::page;
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
                    page(None, markup::home::page(), hx | boosted)
                },
            ),
        )
        .route(
            "/projects",
            get(
                |HxRequest(hx): HxRequest, HxBoosted(boosted): HxBoosted| async move {
                    page(Some("Projects"), markup::projects::page(), hx | boosted)
                },
            ),
        )
        .route(
            "/blog",
            any(|| async { Redirect::permanent("https://blog.joeloach.co.uk") }),
        )
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
