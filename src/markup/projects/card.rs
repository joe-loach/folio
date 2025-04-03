use maud::{html, Markup, PreEscaped, Render};

use crate::markup::tag;

pub enum Destination {
    Website,
    Github,
}

pub struct CardLink {
    pub dest: Destination,
    pub link: &'static str,
}

pub struct Project {
    pub title: &'static str,
    pub img_src: &'static str,
    pub summary: &'static str,
    pub tags: &'static [&'static str],
    pub links: &'static [CardLink],
}

pub const FEATURED_CARDS_LEN: usize = 2;

pub fn featured(cards: Option<[&Project; FEATURED_CARDS_LEN]>) -> Markup {
    html! {
        @if let Some(cards) = cards {
            section ."grid" ."grid-cols-1" ."gap-4" ."sm:grid-cols-2" {
                @for card in cards {
                    (card)
                }
            }
        } @else {
            // no cards to render, instead render a "skeleton"
            // that makes an AJAX request for the real cards on load
            section ."grid" ."grid-cols-1" ."gap-4" ."sm:grid-cols-2" hx-get="/projects/featured" hx-trigger="load" hx-swap="outerHTML" {
                @for _ in 0..FEATURED_CARDS_LEN {
                    (skeleton())
                }
            }
        }
    }
}

const CARD_STYLE: &str = "rounded-xl border bg-card text-foreground shadow flex flex-col h-100";

impl Render for Project {
    fn render(&self) -> Markup {
        html! {
            div class=(CARD_STYLE) {
                header .flex ."flex-col" ."space-y-1.5" ."p-6" {
                    a href=(self.img_src) {
                        img ."h-40" ."w-full" ."object-contain" ."shadow" alt="Project image" src=(self.img_src) {}
                    }
                }
                main .flex ."grow" ."flex-col" ."p-6" ."pt-0" {
                    h3 ."font-semibold" { (self.title) }
                    p ."font-light" ."text-xs" ."text-muted-foreground" { (self.summary) }
                }
                footer .flex ."flex-col" ."p-6" ."pt-0" {
                    // tags
                    ul .flex ."flex-row" ."flex-wrap" ."items-start" ."gap-1" ."pb-2" {
                        @for tag_name in self.tags {
                            li { (tag(tag_name)) }
                        }
                    }
                    // links
                    ul .flex ."flex-row" ."flex-wrap" ."items-start" ."gap-1" {
                        @for link in self.links {
                            li { (link) }
                        }
                    }
                }
            }
        }
    }
}

impl Render for CardLink {
    fn render(&self) -> Markup {
        html! {
            a target="_blank" href=(self.link) {
                (self.dest)
            }
        }
    }
}

impl Render for Destination {
    fn render(&self) -> Markup {
        let logo = match self {
            Destination::Website => PreEscaped(iconify::svg!("lucide:globe", width = "14", height = "14")),
            Destination::Github => PreEscaped(iconify::svg!("lucide:github", width = "14", height = "14")),
        };

        let text = match self {
            Destination::Website => "Website",
            Destination::Github => "Source",
        };

        html! {
            div class="flex items-center gap-2 rounded-md font-semibold bg-primary text-primary-foreground hover:bg-primary/80 px-2 py-1 text-[10px]" {
                (logo)
                (text)
            }
        }
    }
}

fn skeleton() -> Markup {
    html! {
        div class=(CARD_STYLE) {
            header .flex ."flex-col" ."space-y-1.5" ."p-6" {
                div .flex ."flex-col" ."justify-center" ."items-center" ."animate-pulse" ."bg-gray-300" ."h-40" ."w-full" {
                    (PreEscaped(iconify::svg!("lucide:image", width="20", height="20")))
                }
            }
            main .flex ."grow" ."flex-col" ."p-6" ."pt-0" ."animate-pulse" {
                div ."block" {
                    div ."h-5" ."w-24" ."rounded-md" ."bg-gray-300" ."mb-1" {}
                    div ."h-12" ."w-48" ."rounded-md" ."bg-gray-300" {}
                }
            }
            footer .flex ."flex-col" ."p-6" ."pt-0" {
                // tags
                div ."h-4" ."w-full" ."rounded-md" ."bg-gray-300" ."mb-1" {}
                // buttons
                div ."h-4" ."w-full" ."rounded-md" ."bg-gray-300" {}
            }
        }
    }
}