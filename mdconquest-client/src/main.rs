#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use macroquad::prelude::*;

use crate::uihelpers::{Button, PositionedObject, Text};
mod uihelpers;

enum Screen {
    MainMenu,
}

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "MDConquest".to_owned(),
        high_dpi: true,
        ..Default::default()
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        conf.fullscreen = false;
        conf.window_width = 540;
        conf.window_height = 960;
        conf.window_resizable = false;
    }

    conf
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut screen = Screen::MainMenu;

    let font = load_ttf_font("arial.ttf").await.unwrap();

    let text_params = TextParams {
        font: Some(&font),
        font_size: 25,
        color: MAGENTA,
        ..Default::default()
    };

    let sample_text = Text { 
        string: "Sample".to_owned(),
        params: text_params.clone()
    };

    let sample_button = Button { 
        size: (200., 30.),
        text_offset: (10., 25.),
        text: Some(sample_text.clone()),
        color: DARKBLUE,
        hover_color: BLUE
    };

    let start_button = PositionedObject {
        obj: Button { 
            text: Some(Text { 
                string: "Start".to_owned(),
                ..sample_text.clone()
            }),
            ..sample_button
        },
        position: (100., 60.)
    };
    let setting_button = PositionedObject {
        obj: Button { 
            text: Some(Text { 
                string: "Settings".to_owned(),
                ..sample_text
            }),
            ..sample_button
        },
        position: (100., 100.)
    };

    let mut i = 0;

    loop {
        match &screen {
            Screen::MainMenu => {
                start_button.draw();
                setting_button.draw();
                draw_text(i.to_string().as_str(), 10., 10., 10., WHITE);

                if setting_button.hovered(MouseButton::Left) {
                    i += 1;
                }

                #[cfg(not(target_arch = "wasm32"))]
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0)
                }
            }
        }

        next_frame().await
    }
}