#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

static CRAB: &[u8] = include_bytes!("./crab.svg");

fn main() {
    launch(app);
}

fn app() -> Element {
    let crab = dynamic_bytes(CRAB);
    let size_unit = 2f64;
    let mut size_percentage = use_signal(|| 1.0);

    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            width: "100%",
            height: "100%",

            svg {
                width: "{size_percentage * size_unit}",
                height: "{size_percentage * size_unit}",
                svg_data: crab,
            }

            Slider {
                width: "50%",
                value: *size_percentage.read(),
                onmoved: move |s| {
                    size_percentage.set(s);
                }
            }
        }
    )
}
