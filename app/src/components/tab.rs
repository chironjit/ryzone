use dioxus::prelude::*;

#[component]
pub fn Tabs() -> Element {
    rsx! {
        div {
            id: "tabs",
            class: "w-full border rounded-lg shadow-sm",

            div {
                class: "sm:hidden",
                label {
                    r#for: "tabs",
                    class: "sr-only",
                    "Select tab"
                }
                select {
                    id: "tabs",
                    class: "border-0 border-b text-sm rounded-t-lg focus:ring-primary focus:border-primary block w-full p-2.5",
                    option { "Statistics" }
                    option { "Services" }
                    option { "FAQ" }
                }
            }

            ul {
                id: "fullWidthTab",
                class: "hidden text-sm font-medium text-center divide-x rounded-lg sm:flex rtl:divide-x-reverse",
                role: "tablist",
                "data-tabs-toggle": "#fullWidthTabContent",

                li {
                    class: "w-full",
                    button {
                        id: "stats-tab",
                        r#type: "button",
                        role: "tab",
                        "aria-controls": "stats",
                        "aria-selected": "true",
                        "data-tabs-target": "#stats",
                        class: "inline-block w-full p-4 rounded-ss-lg hover:bg-base-200 focus:outline-none",
                        "Statistics"
                    }
                }

                li {
                    class: "w-full",
                    button {
                        id: "about-tab",
                        r#type: "button",
                        role: "tab",
                        "aria-controls": "about",
                        "aria-selected": "false",
                        "data-tabs-target": "#about",
                        class: "inline-block w-full p-4 hover:bg-base-200 focus:outline-none",
                        "Services"
                    }
                }

                li {
                    class: "w-full",
                    button {
                        id: "faq-tab",
                        r#type: "button",
                        role: "tab",
                        "aria-controls": "faq",
                        "aria-selected": "false",
                        "data-tabs-target": "#faq",
                        class: "inline-block w-full p-4 rounded-se-lg hover:bg-base-200 focus:outline-none",
                        "FAQ"
                    }
                }
            }

            div {
                id: "fullWidthTabContent",
                class: "border-t",

                div {
                    id: "stats",
                    class: "hidden p-4 rounded-lg md:p-8",
                    role: "tabpanel",
                    "aria-labelledby": "stats-tab",

                    dl {
                        class: "grid max-w-screen-xl grid-cols-2 gap-8 p-4 mx-auto sm:grid-cols-3 xl:grid-cols-6 sm:p-8",

                        div {
                            class: "flex flex-col items-center justify-center",
                            dt { class: "mb-2 text-3xl font-extrabold", "73M+" }
                            dd { "Developers" }
                        }

                        div {
                            class: "flex flex-col items-center justify-center",
                            dt { class: "mb-2 text-3xl font-extrabold", "100M+" }
                            dd { "Public repositories" }
                        }

                        div {
                            class: "flex flex-col items-center justify-center",
                            dt { class: "mb-2 text-3xl font-extrabold", "1000s" }
                            dd { "Open source projects" }
                        }

                        div {
                            class: "flex flex-col items-center justify-center",
                            dt { class: "mb-2 text-3xl font-extrabold", "1B+" }
                            dd { "Contributors" }
                        }

                        div {
                            class: "flex flex-col items-center justify-center",
                            dt { class: "mb-2 text-3xl font-extrabold", "90+" }
                            dd { "Top Forbes companies" }
                        }

                        div {
                            class: "flex flex-col items-center justify-center",
                            dt { class: "mb-2 text-3xl font-extrabold", "4M+" }
                            dd { "Organizations" }
                        }
                    }
                }

                div {
                    id: "about",
                    class: "hidden p-4 rounded-lg md:p-8",
                    role: "tabpanel",
                    "aria-labelledby": "about-tab",

                    h2 {
                        class: "mb-5 text-2xl font-extrabold tracking-tight",
                        "We invest in the world's potential"
                    }

                    ul {
                        role: "list",
                        class: "space-y-4",

                        li {
                            class: "flex space-x-2 rtl:space-x-reverse items-center",
                            svg {
                                class: "shrink-0 w-3.5 h-3.5",
                                "aria-hidden": "true",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "currentColor",
                                "viewBox": "0 0 20 20",
                                path { d: "M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" }
                            }
                            span { class: "leading-tight", "Dynamic reports and dashboards" }
                        }

                        li {
                            class: "flex space-x-2 rtl:space-x-reverse items-center",
                            svg {
                                class: "shrink-0 w-3.5 h-3.5",
                                "aria-hidden": "true",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "currentColor",
                                "viewBox": "0 0 20 20",
                                path { d: "M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" }
                            }
                            span { class: "leading-tight", "Templates for everyone" }
                        }

                        li {
                            class: "flex space-x-2 rtl:space-x-reverse items-center",
                            svg {
                                class: "shrink-0 w-3.5 h-3.5",
                                "aria-hidden": "true",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "currentColor",
                                "viewBox": "0 0 20 20",
                                path { d: "M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" }
                            }
                            span { class: "leading-tight", "Development workflow" }
                        }

                        li {
                            class: "flex space-x-2 rtl:space-x-reverse items-center",
                            svg {
                                class: "shrink-0 w-3.5 h-3.5",
                                "aria-hidden": "true",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "currentColor",
                                "viewBox": "0 0 20 20",
                                path { d: "M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z" }
                            }
                            span { class: "leading-tight", "Limitless business automation" }
                        }
                    }
                }

                div {
                    id: "faq",
                    class: "hidden p-4 rounded-lg",
                    role: "tabpanel",
                    "aria-labelledby": "faq-tab",

                    div {
                        id: "accordion-flush",
                        "data-accordion": "collapse",
                        "data-active-classes": "bg-base-100",
                        "data-inactive-classes": "",

                        h2 {
                            id: "accordion-flush-heading-1",
                            button {
                                r#type: "button",
                                class: "flex items-center justify-between w-full py-5 font-medium text-left rtl:text-right border-b",
                                "data-accordion-target": "#accordion-flush-body-1",
                                "aria-expanded": "true",
                                "aria-controls": "accordion-flush-body-1",
                                span { "What is Flowbite?" }
                                svg {
                                    "data-accordion-icon": "",
                                    class: "w-3 h-3 rotate-180 shrink-0",
                                    "aria-hidden": "true",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "viewBox": "0 0 10 6",
                                    path {
                                        stroke: "currentColor",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M9 5 5 1 1 5"
                                    }
                                }
                            }
                        }

                        div {
                            id: "accordion-flush-body-1",
                            class: "hidden",
                            "aria-labelledby": "accordion-flush-heading-1",
                            div {
                                class: "py-5 border-b",
                                p {
                                    class: "mb-2",
                                    "Flowbite is an open-source library of interactive components built on top of Tailwind CSS including buttons, dropdowns, modals, navbars, and more."
                                }
                                p {
                                    "Check out this guide to learn how to "
                                    a {
                                        href: "/docs/getting-started/introduction/",
                                        class: "link link-primary",
                                        "get started"
                                    }
                                    " and start developing websites even faster with components on top of Tailwind CSS."
                                }
                            }
                        }

                        h2 {
                            id: "accordion-flush-heading-2",
                            button {
                                r#type: "button",
                                class: "flex items-center justify-between w-full py-5 font-medium text-left rtl:text-right border-b",
                                "data-accordion-target": "#accordion-flush-body-2",
                                "aria-expanded": "false",
                                "aria-controls": "accordion-flush-body-2",
                                span { "Is there a Figma file available?" }
                                svg {
                                    "data-accordion-icon": "",
                                    class: "w-3 h-3 rotate-180 shrink-0",
                                    "aria-hidden": "true",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "viewBox": "0 0 10 6",
                                    path {
                                        stroke: "currentColor",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M9 5 5 1 1 5"
                                    }
                                }
                            }
                        }

                        div {
                            id: "accordion-flush-body-2",
                            class: "hidden",
                            "aria-labelledby": "accordion-flush-heading-2",
                            div {
                                class: "py-5 border-b",
                                p {
                                    class: "mb-2",
                                    "Flowbite is first conceptualized and designed using the Figma software so everything you see in the library has a design equivalent in our Figma file."
                                }
                                p {
                                    "Check out the "
                                    a {
                                        href: "https://flowbite.com/figma/",
                                        class: "link link-primary",
                                        "Figma design system"
                                    }
                                    " based on the utility classes from Tailwind CSS and components from Flowbite."
                                }
                            }
                        }

                        h2 {
                            id: "accordion-flush-heading-3",
                            button {
                                r#type: "button",
                                class: "flex items-center justify-between w-full py-5 font-medium text-left rtl:text-right border-b",
                                "data-accordion-target": "#accordion-flush-body-3",
                                "aria-expanded": "false",
                                "aria-controls": "accordion-flush-body-3",
                                span { "What are the differences between Flowbite and Tailwind UI?" }
                                svg {
                                    "data-accordion-icon": "",
                                    class: "w-3 h-3 rotate-180 shrink-0",
                                    "aria-hidden": "true",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "viewBox": "0 0 10 6",
                                    path {
                                        stroke: "currentColor",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M9 5 5 1 1 5"
                                    }
                                }
                            }
                        }

                        div {
                            id: "accordion-flush-body-3",
                            class: "hidden",
                            "aria-labelledby": "accordion-flush-heading-3",
                            div {
                                class: "py-5 border-b",
                                p {
                                    class: "mb-2",
                                    "The main difference is that the core components from Flowbite are open source under the MIT license, whereas Tailwind UI is a paid product. Another difference is that Flowbite relies on smaller and standalone components, whereas Tailwind UI offers sections of pages."
                                }
                                p {
                                    class: "mb-2",
                                    "However, we actually recommend using both Flowbite, Flowbite Pro, and even Tailwind UI as there is no technical reason stopping you from using the best of two worlds."
                                }
                                p {
                                    class: "mb-2",
                                    "Learn more about these technologies:"
                                }
                                ul {
                                    class: "ps-5 list-disc",
                                    li {
                                        a {
                                            href: "https://flowbite.com/pro/",
                                            class: "link link-primary",
                                            "Flowbite Pro"
                                        }
                                    }
                                    li {
                                        a {
                                            href: "https://tailwindui.com/",
                                            rel: "nofollow",
                                            class: "link link-primary",
                                            "Tailwind UI"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}