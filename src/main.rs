#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::path::PathBuf;

use freya::prelude::*;
use bytes::Bytes;

static OPEN: &[u8] = include_bytes!("./open.svg");
static CRAB: &[u8] = include_bytes!("./crab.svg");
static ICON: &[u8] = include_bytes!("./gallery.png");

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_title("Resize my svg")
            .with_icon(LaunchConfig::load_icon(ICON))
            .build()
    );
}

fn app() -> Element {
    let size_unit = 2f64;
    let mut size_percentage = use_signal(|| 50.0);
    let open_file_svg = static_bytes(OPEN);
    let mut svg_bytes = use_signal::<Option<Bytes>>(|| None);
    let svg_data = svg_bytes
        .read()
        .as_ref()
        .map(|bytes| dynamic_bytes(bytes.clone()));

    let set_image_from_path = move |path: PathBuf| {
        spawn(async move {
            let file_content = tokio::fs::read(path).await;
            if let Ok(file_content) = file_content {
                svg_bytes.set(Some(Bytes::from(file_content)));
            }
        });
    };

    let open_image = move |_| {
        spawn(async move {
            let path = native_dialog::FileDialog
                ::new()
                .add_filter("Svg file", &["svg"])
                .show_open_single_file()
                .expect("failed to open file chooser");
            if let Some(path) = path {
                set_image_from_path(path);
            }
        });
    };

    use_effect(move || {
        svg_bytes.set(Some(Bytes::from_static(CRAB)));
    });

    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            width: "fill",
            height: "fill",

            if let Some(svg_data) = svg_data {
                svg {
                    width: "{size_percentage * size_unit}",
                    height: "{size_percentage * size_unit}",
                    svg_data,
                }
            } else {
                label {
                    "..."
                }
            }

            Slider {
                width: "50%",
                value: *size_percentage.read(),
                onmoved: move |s| {
                    size_percentage.set(s);
                }
            }

            Button {
                onclick: open_image,
                svg {
                    width: "50",
                    height: "50",
                    svg_data: open_file_svg ,
                }
            }
        }
    )
}
