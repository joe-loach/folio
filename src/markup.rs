pub mod home;
pub mod projects;

use maud::{DOCTYPE, Markup, PreEscaped, Render, html};

const BUTTON: &str = "inline-flex relative items-center justify-center whitespace-nowrap rounded-md text-sm font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 ";

fn format_title(name: Option<&str>) -> String {
    if let Some(name) = name {
        format!("Joe Loach - {}", name)
    } else {
        "Joe Loach".to_owned()
    }
}

/// Page layout
pub fn page_layout(title: Option<&str>, content: Markup, partial: bool) -> Markup {
    if partial {
        return html! {
            title { (format_title(title)) }
            (content)
        };
    }

    html! {
        (head(title))
        body class="mx-auto flex min-h-screen max-w-3xl flex-col px-8" {
            (header())
            main .grow {
                div #content {
                    (content)
                }
            }
            (footer())
            (toast_area())
        }
    }
}

fn head(title: Option<&str>) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1";

            meta name="author" content="Joe Loach";
            meta name="description" content="Joe Loach's website";
            link rel="canonical" href="https://joeloach.co.uk";

            // Icons
            link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon/favicon-32x32.png";
            link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon/favicon-16x16.png";

            // tailwind
            link rel="stylesheet" type="text/css" href="tailwind.css";

            // Page title
            title { (format_title(title)) }

            // HTMX
            script src="/htmx.min.js" {}

            // Theme updating
            (Javascript(include_str!("theming.js")))
            // Toasts
            (Javascript(include_str!("toasts.js")))
        }
    }
}

fn header() -> Markup {
    html! {
        header class="sticky top-0 z-40 bg-background/75 py-6 backdrop-blur-sm"{
            nav class="flex items-center justify-between" {
                (nav_links())
                div class="flex gap-0 sm:gap-4" {
                    (theme_toggle())
                }
            }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer class="flex flex-col items-center justify-center pb-16 sm:flex-row-reverse sm:justify-between" {
            div {
                (socials())
            }
            p ."text-center" ."text-xs" ."text-muted-foreground" {
                a ."hover:text-foreground" href="https://github.com/joe-loach/folio" { "Hand written html" } " by Joe Loach"
            }
        }
    }
}

fn socials() -> Markup {
    html! {
        section class="flex gap-6" {
            a class="text-muted-foreground hover:text-foreground" href="https://www.linkedin.com/in/joe-loach/" target="_blank" title="LinkedIn" {
                (PreEscaped(iconify::svg!("lucide:linkedin", width="20", height="20")))
            }
            a class="text-muted-foreground hover:text-foreground" href="https://github.com/joe-loach" target="_blank" title="GitHub" {
                (PreEscaped(iconify::svg!("lucide:github", width="20", height="20")))
            }
        }
    }
}

fn theme_toggle() -> Markup {
    html! {
        button
          class="cursor-pointer inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium hover:bg-accent hover:text-accent-foreground h-9 w-9"
          title="Toggle theme"
          aria-label="Toggle theme"
          onClick="toggleTheme()"
        {
            div class="size-4 dark:hidden text-indigo-500" {
                (PreEscaped(iconify::svg!("lucide:moon", width="16px", height="16px")))
            }
            div class="size-4 hidden dark:block text-orange-300"{
                (PreEscaped(iconify::svg!("lucide:sun", width="16px", height="16px")))
            }
        }
    }
}

fn nav_links() -> Markup {
    let link = |name: &str, href: &str| -> Markup {
        html! {
            li ."text-muted-foreground" ."hover:text-foreground" {
                a href=(href) { (name) }
            }
        }
    };

    html! {
        // HACK: prevent scrolling when clicking boosted links
        // htmx will try and move you down to the target, we don't want that...
        // so specify a handle for htmx to scroll to that doesn't exist and it wont move
        ul hx-boost="true" hx-target="#content" hx-swap="innerHTML show:no-scroll" class="flex gap-4 sm:gap-8" {
            (link("home", "/"))
            (link("projects", "/projects"))
            (link("blog", "/blog"))
        }
    }
}

fn toast_area() -> Markup {
    html! {
        aside id="toasts" class="fixed top-4 left-1/2 transform -translate-x-1/2 z-50 overflow-hidden" {
            (toast("email", html! {
                div ".w-4" ."h-4" ."mr-2" {
                    (PreEscaped(iconify::svg!("carbon:email", width="20", height="20")))
                }
                "Email Copied"
            }))
        }
    }
}

fn toast(id: &str, body: Markup) -> Markup {
    html! {
        div id=(id) class="hidden animate-fade-in-toast max-w-xs text-center bg-accent text-sm text-accent-foreground rounded-xl shadow-md" role="alert" tabindex="-1" aria-labelledby="hs-toast-solid-color-dark-label" {
            div id="hs-toast-solid-color-dark-label" class="flex p-4 items-baseline" {
                (body)
            }
        }
    }
}

fn tag(name: &str) -> Markup {
    html! {
        div class="inline-flex items-center rounded-md font-semibold focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 bg-secondary text-secondary-foreground hover:bg-secondary/80 px-1 py-0 text-[10px]" {
            (name)
        }
    }
}

pub struct Javascript(&'static str);

impl Render for Javascript {
    fn render(&self) -> Markup {
        html! {
            script type="text/javascript" {
                (PreEscaped(self.0))
            }
        }
    }
}