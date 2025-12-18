use macroquad::prelude::*;

#[derive(Clone)]
pub struct Text<'a> {
    pub string: String,
    pub params: TextParams<'a>,
}

#[derive(Clone)]
pub struct Button<'a> {
    pub size: (f32, f32),
    pub text_offset: (f32, f32),
    pub text: Option<Text<'a>>,
    pub color: Color,
    pub hover_color: Color
}

pub trait Hoverable {
    fn hovered(&self, x: f32, y: f32, button: MouseButton) -> bool;
}

pub trait Drawable {
    fn draw(&self, x: f32, y: f32);
}

impl Drawable for Text<'_> {
    fn draw(&self, x: f32, y: f32) {
        draw_text_ex(self.string.as_str(), x, y, self.params.clone());
    }
}

impl Drawable for Button<'_> {
    fn draw(&self, x: f32, y: f32) {
        let mouse = mouse_position();
        let rect = Rect::new(x, y, self.size.0, self.size.1);
        let hover = rect.contains(vec2(mouse.0, mouse.1));

        draw_rectangle(x, y, self.size.0, self.size.1, if hover { self.hover_color } else { self.color });

        if let Some(tx) = &self.text {
            tx.draw(x + self.text_offset.0, y + self.text_offset.1);
        }
    }
}

impl Hoverable for Button<'_> {
    fn hovered(&self, x: f32, y: f32, button: MouseButton) -> bool {
        let mouse = mouse_position();
        let rect = Rect::new(x, y, self.size.0, self.size.1);
        let hover = rect.contains(vec2(mouse.0, mouse.1));

        hover && is_mouse_button_pressed(button)
    }
}

#[derive(Clone)]
pub struct PositionedObject<T> {
    pub obj: T,
    pub position: (f32, f32)
}

impl<T: Drawable> PositionedObject<T> {
    pub fn draw(&self) {
        self.obj.draw( self.position.0, self.position.1);
    }
}

impl<T: Hoverable> PositionedObject<T> {
    pub fn hovered(&self, mb: MouseButton) -> bool {
        self.obj.hovered(self.position.0, self.position.1, mb)
    }
}

#[derive(Clone)]
pub struct Card {
    pub attack: (u8, u8, u8, u8),
    pub color: Color
}

#[derive(Clone)]
pub enum Side {
    Me,
    Enemy
}

#[derive(Clone)]
pub struct Cell {
    pub card: Option<Card>,
    pub owner: Option<Side>
}

#[derive(Clone)]
pub struct Deck {
    pub size: (usize, usize),
    pub cells: Vec<Cell>
}

#[derive(Clone)]
pub struct CardDeck {
    pub cards: Vec<Card>
}

impl Drawable for CardDeck {
    fn draw(&self, x: f32, y: f32) {
        for (i, card) in self.cards.iter().enumerate() {
            card.draw(x + i as f32 * 50., y);
        }
    }
}

impl Drawable for Deck {
    fn draw(&self, x: f32, y: f32) {
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                self.cells[i * self.size.0 + j].draw(x + (i + 1) as f32 * 60., y + (j + 1) as f32 * 60.);
            }
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Up, Right, Down, Left
}

impl Deck {
    fn get_cards(&self, card_id: usize, direction: Direction) -> Vec<(usize, Cell)> {
        let w = self.size.0;
        let h = self.size.1;
        
        let current_x = card_id % w;
        let current_y = card_id / w;

        let mut result = Vec::new();

        match direction {
            Direction::Right => {
                for x in (current_x + 1)..w {
                    let idx = current_y * w + x;
                    result.push((idx, self.cells[idx].clone()));
                }
            },
            Direction::Left => {
                for x in (0..current_x).rev() {
                    let idx = current_y * w + x;
                    result.push((idx, self.cells[idx].clone()));
                }
            },
            Direction::Down => {
                for y in (current_y + 1)..h {
                    let idx = y * w + current_x;
                    result.push((idx, self.cells[idx].clone()));
                }
            },
            Direction::Up => {
                for y in (0..current_y).rev() {
                    let idx = y * w + current_x;
                    result.push((idx, self.cells[idx].clone()));
                }
            },
        }

        result
    }

    pub fn change_side(&mut self, card: usize, side: Side) {
        self.cells[card].owner.replace(side);
    }

    pub fn tick_turn(&mut self, new_card_id: usize) {
        //все карты сверху, справа, снизу, слева
        let Some(new_card) = self.cells[new_card_id].card.clone() else { 
            return;
        };
        let Some(side) = self.cells[new_card_id].owner.clone() else {
            return;
        };

        let mut up_attack = new_card.attack.0;
        let up_cards = self.get_cards(new_card_id, Direction::Up);
        for tuple in &up_cards {
            let Some(card) = &tuple.1.card else {
                break
            };
            up_attack -= card.attack.2;
            if up_attack > 0 {
                self.change_side(tuple.0, side.clone());
            }
        }
        
        let mut right_attack = new_card.attack.1;
        let right_cards = self.get_cards(new_card_id, Direction::Right);
        for tuple in &right_cards {
            let Some(card) = &tuple.1.card else {
                break
            };
            right_attack -= card.attack.3;
            if right_attack > 0 {
                self.change_side(tuple.0, side.clone());
            }
        }
        
        let mut down_attack = new_card.attack.2;
        let down_cards = self.get_cards(new_card_id, Direction::Down);
        for tuple in &down_cards {
            let Some(card) = &tuple.1.card else {
                break
            };
            down_attack -= card.attack.0;
            if down_attack > 0 {
                self.change_side(tuple.0, side.clone());
            }
        }
        
        let mut left_attack = new_card.attack.3;
        let left_cards = self.get_cards(new_card_id, Direction::Left);
        for tuple in &left_cards {
            let Some(card) = &tuple.1.card else {
                break
            };
            left_attack -= card.attack.1;
            if left_attack > 0 {
                self.change_side(tuple.0, side.clone());
            }
        }
    }
}

impl Drawable for Card {
    fn draw(&self, x: f32, y: f32) {
        draw_rectangle(x + 5., y + 5., 40., 40., self.color);
        draw_text(self.attack.0.to_string().as_str(), x + 20., y + 15., 14., BLACK);
        draw_text(self.attack.1.to_string().as_str(), x + 30., y + 20., 14., BLACK);
        draw_text(self.attack.2.to_string().as_str(), x + 20., y + 25., 14., BLACK);
        draw_text(self.attack.3.to_string().as_str(), x + 10., y + 20., 14., BLACK);
    }
}

impl Drawable for Cell {
    fn draw(&self, x: f32, y: f32) {
        if let Some(card) = &self.card {
            card.draw(x, y)
        }
        draw_rectangle(x, y, 50., 50., match self.owner {
            None => GRAY,
            Some(Side::Me) => GREEN,
            Some(Side::Enemy) => RED
        });
    }
}