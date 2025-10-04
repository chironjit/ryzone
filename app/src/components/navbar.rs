use dioxus::prelude::*;
use crate::AppState;

const ICON: Asset = asset!("/assets/icon.svg");
const LIGHT: Asset = asset!("/assets/light.svg");
const DARK: Asset = asset!("/assets/dark.svg");

#[component]
pub fn Navbar() -> Element {
    let app_state = use_context::<AppState>();
    let mut theme_mode = app_state.themeMode;

    let toggle_theme = move |_| {
        let new_mode = if theme_mode() == "dark" { "light" } else { "dark" };
        theme_mode.set(new_mode.to_string());
    };

    

    rsx! {
        nav {
            id: "navbar",
            class: "flex items-center justify-between px-6 py-4 shadow-md",
            style: "background-color: var(--color-base-100); color: var(--color-base-content);",

            // Left side - Icon/Logo
            div {
                class: "flex items-center",
                img {
                    src: ICON,
                    id: "icon",
                    class: "h-8 w-8",
                    alt: "Logo"
                }
            }

            // Right side - Navigation Links
            div {
                class: "flex items-center gap-6",
                p {
                    class: "transition-colors",
                    style: "color: var(--color-base-content);",
                    "Home"
                }
                p {
                    class: "transition-colors",
                    style: "color: var(--color-base-content);",
                    "Blog"
                }

                // Theme toggle button
                button {
                    class: "",
                    style: "color: var(--color-base-content);",
                    onclick: toggle_theme,
                    if theme_mode() == "dark" {
                            svg {
                                xmlns:"http://www.w3.org/2000/svg",
                                width:"24",
                                height:"24",
                                view_box: "0 0 24 24",
                                fill:"none",
                                stroke:"currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class:"lucide lucide-sun-icon lucide-sun",
                                circle { cx: "12", cy: "12", r: "4" }
                                path { d: "M12 2v2" }
                                path { d: "M12 20v2" }
                                path { d: "m4.93 4.93 1.41 1.41" }
                                path { d: "m17.66 17.66 1.41 1.41" }
                                path { d: "M2 12h2" }
                                path { d: "M20 12h2" }
                                path { d: "m6.34 17.66-1.41 1.41" }
                                path { d: "m19.07 4.93-1.41 1.41" }
                            }
                        } else {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "lucide lucide-moon-icon lucide-moon",
                                path { d: "M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401" }
                            }
                        }
                }
            }
        }
    }
}