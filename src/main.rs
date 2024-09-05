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
    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            width: "100%",
            height: "100%",
            svg {
                width: "100",
                height: "100",
                svg_data: crab,
            }
        }
    )
}
