use maud::{Markup, PreEscaped, html};

use crate::markup::{socials, projects::card::featured, tag, BUTTON};

pub fn page() -> Markup {
    html! {
        article class="mt-8 flex flex-col gap-16 pb-16" {
            (intro())
            (technologies())
            (projects())
            (blog_posts())
        }
    }
}

fn intro() -> Markup {
    html! {
        section class="flex flex-col items-start gap-8 md:flex-row-reverse md:items-center md:justify-between" {
            img ."rounded-lg" fetchpriority="high" alt="Joe with Food" width="175" height="175" src="/assets/pics/joe.webp" {}
            div .flex ."flex-col" {
                h1 ."text-5xl" ."font-semibold" {
                    "Hi I'm Joe 👋"
                }
                p ."mt-4" ."font-light" {
                    "A 21-year-old software developer from the UK."
                }
                p ."mt-2" ."font-light" {
                    "I like to write Rust, cook amazing food and play guitar."
                }
                section class="mt-4 flex items-center gap-8" {
                    a target="_blank" href="/resume.pdf" {
                        button class=(BUTTON) {
                            span ."font-semibold" { "Resume" }
                            div ."ml-2" {
                                (PreEscaped(iconify::svg!("lucide:file-down", width="20", height="20")))
                            }
                        }
                    }
                    (socials())
                }
                section class="mt-4 flex items-center gap-4" {
                    button class={(BUTTON) "group active:bg-green-500 active:border-green-500"}
                    onclick="navigator.clipboard.writeText(`${'me'}@joeloach.co.uk`); toast('email');"
                    {
                        div ."h-5" ."w-5" {
                            (PreEscaped(iconify::svg!("carbon:copy", width="20", height="20")))
                        }
                        code ."ml-2" ."text-xs" ."font-normal" ."text-muted-foreground/75" ."group-active:text-foreground" {
                            // let email = 
                            span ."hidden" ."xs:inline-block" ."mr-1" {
                                span { "let " }
                                span ."text-sky-500" ."group-active:text-foreground" ."font-bold" { "email " }
                                span { "=" }
                            }
                            @let string = "text-green-500 dark:text-green-400 group-active:text-foreground font-bold";
                            // format!("{}@joeloach.co.uk", "me")
                            span ."inline-block" ."whitespace-normal" {
                                span { "format!(" }
                                (PreEscaped("<wbr />"))
                                span class=(string) { "\"" }
                                span { "{}" }
                                span class=(string) { "@joeloach.co.uk\"" }
                                ", "
                                (PreEscaped("<wbr />"))
                                span class=(string) { "\"me\"" }
                                (PreEscaped("<wbr />"))
                                ");"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn technologies() -> Markup {
    fn item(long: &str, short: &str, icon: PreEscaped<&str>, dark_icon: Option<PreEscaped<&str>>) -> Markup {
        html! {
            li ."w-[50px] h-auto" {
                div ."flex" ."flex-col" ."items-center" ."gap-1" {
                    button
                    class="border-2 not-hover:!border-transparent hover:border-foreground rounded-xl not-hover:black-white hover:no-filter"
                    title=(long) {
                        @if let Some(dark_icon) = dark_icon {
                            div ."dark:hidden" { (icon) }
                            div ."not-dark:hidden" { (dark_icon) }
                        } @else {
                            div { (icon) }
                        }
                    }
                    (tag(short))
                }
            }
        }
    }

    html! {
        section class="flex flex-col gap-8" {
            div ."flex" ."flex-col" ."gap-4" {
                h1 ."text-2xl" ."font-semibold" { "Technologies I use" }
                ul .flex ."flex-wrap" ."gap-3" {
                    (item("Rust", "Rust", PreEscaped(iconify::svg!("skill-icons:rust", width="42", height="42")), None))
                    (item(
                        "Cloudflare Workers",
                        "Workers",
                        PreEscaped(iconify::svg!("skill-icons:cloudflare-light", width="42", height="42")),
                        Some(PreEscaped(iconify::svg!("skill-icons:cloudflare-dark", width="42", height="42"))),
                    ))
                    (item("C#", "C#", PreEscaped(iconify::svg!("skill-icons:cs", width="42", height="42")), None))
                    (item("C++", "C++", PreEscaped(iconify::svg!("skill-icons:cpp", width="42", height="42")), None))
                    (item("JavaScript", "JS", PreEscaped(iconify::svg!("skill-icons:javascript", width="42", height="42")), None))
                    (item("TypeScript", "TS", PreEscaped(iconify::svg!("skill-icons:typescript", width="42", height="42")), None))
                    (item("HTML", "HTML", PreEscaped(iconify::svg!("skill-icons:html", width="42", height="42")), None))
                    (item("CSS", "CSS", PreEscaped(iconify::svg!("skill-icons:css", width="42", height="42")), None))
                    (item(
                        "Tailwind CSS",
                        "Tailwind",
                        PreEscaped(iconify::svg!("skill-icons:tailwindcss-light", width="42", height="42")),
                        Some(PreEscaped(iconify::svg!("skill-icons:tailwindcss-dark", width="42", height="42")))
                    ))
                    (item(
                        "Postgresql",
                        "SQL",
                        PreEscaped(iconify::svg!("skill-icons:postgresql-light", width="42", height="42")),
                        Some(PreEscaped(iconify::svg!("skill-icons:postgresql-dark", width="42", height="42")))
                    ))
                    (item("System Verilog", "Verilog", PreEscaped(iconify::svg!("vscode-icons:file-type-verilog", width="42", height="42")), None))
                }
            }
        }
    }
}

fn projects() -> Markup {
    html! {
        section class="flex flex-col gap-8" {
            div ."flex" ."justify-between" {
                h2 ."text-2xl" ."font-semibold" { "Featured projects" }
                a ."text-muted-foreground" ."hover:text-foreground" ."flex" ."items-center" ."gap-2" ."font-light" href="/projects" hx-boost="true" hx-target="#content" {
                    span { "view more" }
                    (PreEscaped(iconify::svg!("solar:arrow-right-linear", width="20", height="20")))
                }
            }
            (featured(None))
        }
    }
}

fn blog_posts() -> Markup {
    html! {
        section class="flex flex-col gap-8" {
            div ."flex" ."justify-between" {
                h2 ."text-2xl" ."font-semibold" { "Recent Posts" }
                a ."text-muted-foreground" ."hover:text-foreground" ."flex" ."items-center" ."gap-2" ."font-light" href="https://blog.joeloach.co.uk" {
                    span { "view more" }
                    (PreEscaped(iconify::svg!("solar:arrow-right-linear", width="20", height="20")))
                }
            }
            div hx-get="https://blog.joeloach.co.uk/post/latest" hx-trigger="load" {}
        }
    }
}
