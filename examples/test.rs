use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use macroquad::prelude::*;
use Simples_menu::{Button, Menu, Text_label};

#[macroquad::main("Test")]
async fn main() {
    let mut menu = Menu::new("Menu".to_string(), Vec2{ x: 50.0, y: 50.0 });
    //menu.size = Some(Vec2 { x: 220.0, y: 80.0 });

    //menu.elements.push(Rc::new(Button::new("Buttom 1".to_string(), Vec2{ x: 0.0, y: 0.0 }, None)));
    //menu.elements.push(Rc::new(Button::new("Buttom 2".to_string(), Vec2{ x: 0.0, y: 20.0 }, None)));
    //menu.elements.push(Rc::new(Text_label::new("Teste de legenda: ".to_string(), Vec2{ x: 30.0, y: 50.0 })));


    let buttan1 = menu.add_element(Button::new("Buttom 1".to_string(), Vec2{ x: 0.0, y: 0.0 }, None));
    let label1 = menu.add_element(Text_label::new("Teste de legenda: ".to_string(), Vec2{ x: 30.0, y: 50.0 }));
    loop {
        menu.draw();

        label1.edit().title.name = get_fps().to_string();
        buttan1.edit().position.x += 0.1;

        next_frame().await
    }
}