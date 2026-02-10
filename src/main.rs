// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use dioxus_desktop::{Config, WindowCloseBehaviour};

use components::{Navbar, Battery, Dashboard, Info, Profiles, Settings};

use std::os::unix::net::{UnixStream, UnixListener};
use std::io::Write;
use std::path::PathBuf;



/// Define a components module that contains all shared components for our app.
mod components;
mod utils;

use utils::settings::{read_app_settings, read_profile_settings};
use utils::types::{AppSettings, ProfileSettings, CurrentStats};

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/icon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn socket_path() -> PathBuf {
    // XDG_RUNTIME_DIR is per-user, tmpfs, and cleaned on logout
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(runtime_dir).join("ryzone.sock")
}

fn main() {
    let sock = socket_path();

    if let Ok(mut stream) = UnixStream::connect(&sock) {
        let _ = stream.write_all(b"show");
        std::process::exit(0);
    }
    let _ = std::fs::remove_file(&sock);

    // Read settings ONCE here
    let app_settings = read_app_settings().expect("Failed to read app settings");
    let profile_settings = read_profile_settings().expect("Failed to read profile settings");

    // Use the setting to determine window close behavior
    let window_close_option = if app_settings.app.minimize_to_tray {
        WindowCloseBehaviour::WindowHides
    } else {
        WindowCloseBehaviour::WindowCloses
    };

    // Pass settings into the Dioxus app via context
    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_close_behaviour(window_close_option))
        .with_context(app_settings)
        .with_context(profile_settings)
        .launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Check if an instance is already running
    // If yes, show the already running instance
    // Runs ONCE on first render -- creates the listener thread
    let mut show_requested = use_signal_sync(|| false);

    use_hook(move || {
        std::thread::spawn(move || {
            let listener = UnixListener::bind(socket_path()).expect("Failed to bind socket");
            // Blocking .incoming() -- waits for connections, no sleep needed
            for stream in listener.incoming() {
                if stream.is_ok() {
                    show_requested.set(true);
                }
            }
        });
    });

    // Reacts to the signal change
    use_effect(move || {
        if show_requested() {
            let window = dioxus_desktop::window();
            window.set_visible(true);
            window.set_focus();
            show_requested.set(false);
        }
    });

    


    // Initialise the state
    // Retrieve settings injected from main()
    let app_settings = use_context::<AppSettings>();
    let profile_settings = use_context::<ProfileSettings>();

    use_context_provider(|| Signal::new(app_settings));
    use_context_provider(|| Signal::new(profile_settings));

    // Runtime-only UI state (not persisted) stays as individual signals
    let mut active_tab = use_signal(|| "dashboard".to_string());

    // Use memos to extract specific fields - only re-renders when that field changes
    let settings = use_context::<Signal<AppSettings>>();
    let theme_mode = use_memo(move || settings().style.theme_mode.clone());
    let theme_light = use_memo(move || settings().style.theme_light_palette.clone());
    let theme_dark = use_memo(move || settings().style.theme_dark_palette.clone());

    // Compute the active theme based on mode
    let active_theme = use_memo(move || {
        if theme_mode() == "dark" {
            theme_dark
        } else {
            theme_light
        }
    });

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "flex flex-col h-screen bg-[var(--color-base-100)] text-[var(--color-base-content)]",
            "data-theme": "{active_theme}",

            Navbar{active_tab: active_tab}

            // Main Content Area
            main { class: "flex-1 overflow-auto",
                match active_tab().as_str() {
                    "dashboard" => rsx! { Dashboard {} },
                    "battery" => rsx! { Battery {} },
                    "profiles" => rsx! { Profiles {} },
                    "settings" => rsx! { Settings {} },
                    "info" => rsx! { Info {} },
                    _ => rsx! { Dashboard {} },
                }
            }
        }
    }
}
