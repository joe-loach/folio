pub mod card;

use axum::{
    Router,
    response::{IntoResponse, Redirect},
    routing::get,
};
use axum_htmx::{HxBoosted, HxRequest};
use card::{CardLink, Project, featured};
use maud::{Markup, html};

use crate::page_layout;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(
                |HxRequest(hx): HxRequest, HxBoosted(boosted): HxBoosted| async move {
                    page_layout(Some("Projects"), page(), hx | boosted)
                },
            ),
        )
        .route(
            "/featured",
            get(|HxRequest(hx): HxRequest| async move {
                if !hx {
                    return Redirect::to("/projects").into_response();
                }

                featured(Some(const { [&PROJECTS[0], &PROJECTS[1]] })).into_response()
            }),
        )
}

static PROJECTS: &[Project] = &[
    Project {
        title: "Mock Manchester Bee Network App",
        img_src: "/assets/projects/bee.png",
        summary: "Functional mock-up of the Manchester Bee Network App in the browser. Keeps track of QR usage for users on a per-ticket basis to see their usage statistics. Served on the edge using cloudflare workers.",
        tags: &["Rust", "Workers", "HTML", "JS", "CSS", "SQL"],
        links: &[
            CardLink {
                dest: card::Destination::Website,
                link: "https://bee.joe-gloach.workers.dev/",
            },
            CardLink {
                dest: card::Destination::Github,
                link: "https://github.com/joe-loach/bee",
            },
        ],
    },
    Project {
        title: "Real-time black hole path tracer",
        img_src: "/assets/projects/renderer.webp",
        summary: "A native high-performance, hardware-agnostic path tracer for real-time black hole rendering. Allows users to explore the strange phyiscal effects black holes have on spacetime.",
        tags: &["Rust", "WGSL", "Vulkan", "WebGPU"],
        links: &[
            CardLink {
                dest: card::Destination::Github,
                link: "https://github.com/joe-loach/kerrbhy",
            },
        ],
    },
];

fn page() -> Markup {
    html! {
        article class="mt-8 flex flex-col gap-16 pb-16" {
            h1 ."text-5xl" ."font-semibold" { "My projects" }
            section ."grid" ."grid-cols-1" ."gap-4" ."sm:grid-cols-2" {
                @for project in PROJECTS {
                    (project)
                }
            }
        }
    }
}
