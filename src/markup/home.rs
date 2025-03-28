use maud::{Markup, PreEscaped, html};

use crate::markup::BUTTON;

pub fn page() -> Markup {
    html! {
        article class="mt-8 flex flex-col gap-16 pb-16" {
            (intro())
            (technologies())
            // (projects())
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
                    section class="flex gap-6" {
                        a class="text-muted-foreground hover:text-foreground" href="https://www.linkedin.com/in/joe-loach/" target="_blank" title="LinkedIn" {
                            (PreEscaped(iconify::svg!("lucide:linkedin", width="20", height="20")))
                        }
                        a class="text-muted-foreground hover:text-foreground" href="https://github.com/joe-loach" target="_blank" title="GitHub" {
                            (PreEscaped(iconify::svg!("lucide:github", width="20", height="20")))
                        }
                    }
                }
                section class="mt-4 flex items-center gap-4" {
                    button
                        class={(BUTTON) "active:bg-green-500"}
                        onclick="navigator.clipboard.writeText('me@joeloach.co.uk')"
                        {
                            (PreEscaped(iconify::svg!("carbon:copy", width="20", height="20")))
                            code ."ml-2" ."text-xs" {
                                "let email = format!(\"{}@joeloach.co.uk\", \"me\");"
                            }
                        }
                }
            }
        }
    }
}

fn technologies() -> Markup {
    fn item(name: &str, icon: PreEscaped<&str>) -> Markup {
        html! {
            li {
                div ."flex" ."flex-col" ."items-center" ."gap-1" {
                    button class="border border-transparent hover:border-foreground rounded-lg" title=(name) alt=(format!("{} Icon", name)) {
                        (icon)
                    }
                    div class="inline-flex items-center rounded-md font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 bg-secondary text-secondary-foreground hover:bg-secondary/80 px-1 py-0 text-[10px]" {
                        (name)
                    }
                }
            }
        }
    }

    html! {
        section class="flex flex-col gap-8" {
            div ."flex" ."flex-col" ."gap-4" {
                h1 ."text-2xl" ."font-semibold" { "Technologies I use" }
                ul .grid ."grid-flow-col" ."auto-cols-[32px]" ."gap-4" {
                    (item("Rust", PreEscaped(iconify::svg!("skill-icons:rust", width="32", height="32"))))
                    (item("C", PreEscaped(iconify::svg!("skill-icons:c", width="32", height="32"))))
                    (item("C++", PreEscaped(iconify::svg!("skill-icons:cpp", width="32", height="32"))))
                    (item("C#", PreEscaped(iconify::svg!("skill-icons:cs", width="32", height="32"))))
                    (item("SQL", PreEscaped(iconify::svg!("skill-icons:postgresql-light", width="32", height="32"))))
                    (item("Workers", PreEscaped(iconify::svg!("skill-icons:cloudflare-light", width="32", height="32"))))
                }
            }
        }
    }
}

// fn projects() -> Markup {
//     html! {
//         section class="flex flex-col gap-8" {
//             div ."flex" ."flex-col" ."gap-4" {
//                 h1 ."text-2xl" ."font-semibold" { "Featured projects" }
//             }
//         }
//     }
// }