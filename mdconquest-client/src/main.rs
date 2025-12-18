#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::vec;

use macroquad::prelude::*;
use crate::uihelpers::{Button, Card, CardDeck, Cell, Deck, Drawable, PositionedObject, Side, Text};
mod uihelpers;

enum Screen {
    MainMenu,
    Game
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

fn new_deck() -> Deck {
    Deck {
        size: (3, 3),
        cells: vec![Cell {
            card: None,
            owner: None
        }; 9]
    }
}

fn generate_cards(count: usize) -> Vec<Card> {
    let mut result = Vec::<Card>::new();
    for _ in 0..count {
        result.push(Card {
            attack: (rand::gen_range(1, 9), rand::gen_range(1, 9), rand::gen_range(1, 9), rand::gen_range(1, 9)),
            color: Color {
                r: rand::gen_range(0.5, 1.),
                g: rand::gen_range(0.5, 1.),
                b: rand::gen_range(0.5, 1.), 
                a: 1.
            }
        });
    }
    result
}

fn generate_card_deck(count: usize) -> CardDeck {
    CardDeck { 
        cards: generate_cards(count)
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

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

    let mut current_turn = Option::<Side>::None;
    let mut deck = new_deck();
    let mut my_card_deck = generate_card_deck(6);
    let mut enemy_card_deck = generate_card_deck(6);

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
                if is_key_pressed(KeyCode::Right) {
                    screen = Screen::Game;
                }
            }
            Screen::Game => {
                enemy_card_deck.draw(100., 100.);
                deck.draw(100., 200.);
                my_card_deck.draw(100., 500.);

                if is_key_pressed(KeyCode::Left) {
                    screen = Screen::MainMenu;
                }
            }
        }

        next_frame().await
    }
}