use macroquad::prelude::*;

#[derive(Clone)]
pub struct Text<'a> {
    pub string: String,
    pub params: TextParams<'a>
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

pub fn btn(text: &str, y: f32) -> bool {
    let mouse = mouse_position();
    let rect = Rect::new(20.0, y, 400.0, 40.0);
    let hover = rect.contains(vec2(mouse.0, mouse.1));
    
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, if hover { DARKGRAY } else { LIGHTGRAY });
    draw_text(text, rect.x + 10.0, rect.y + 30.0, 30.0, BLACK);
    
    hover && is_mouse_button_pressed(MouseButton::Left)
}