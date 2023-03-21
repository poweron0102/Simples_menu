use macroquad::prelude::*;
use Simples_menu::{Button, Menu, Text_label};

#[macroquad::main("Test")]
async fn main() {
    let mut menu = Menu::new("Menu".to_string(), Vec2{ x: 50.0, y: 50.0 });
    //menu.size = Some(Vec2 { x: 220.0, y: 80.0 });
    menu.elements.push(Box::new(Button::new("Buttom 1".to_string(), Vec2{ x: 0.0, y: 0.0 }, None)));
    menu.elements.push(Box::new(Button::new("Buttom 2".to_string(), Vec2{ x: 0.0, y: 20.0 }, None)));
    menu.elements.push(Box::new(Text_label::new("Teste de legenda: ".to_string(), Vec2{ x: 30.0, y: 50.0 })));
    loop {
        menu.draw();


        next_frame().await
    }
}