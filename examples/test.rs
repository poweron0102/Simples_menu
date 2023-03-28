use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use macroquad::prelude::*;
use Simples_menu::{Button, Menu, TextLabel};

#[macroquad::main("Test")]
async fn main() {
    let mut menu1 = Menu::new("Menu 1".to_string(), Vec2{ x: 50.0, y: 50.0 });
    //menu1.size = Some(Vec2 { x: 220.0, y: 80.0 });
    let buttan1 = menu1.add_element(Button::new("Buttom 1".to_string(), Vec2{ x: 0.0, y: 120.0 }, None));
    let buttan2 = menu1.add_element(Button::new("Buttom 2".to_string(), Vec2{ x: 0.0, y: 0.0 }, None));
    let label1 = menu1.add_element(TextLabel::new("Teste de legenda: ".to_string(), Vec2{ x: 30.0, y: 50.0 }));

    let mut menu2 = Menu::new("Menu 2".to_string(), Vec2{ x: 250.0, y: 260.0 });
    //menu2.color = BLUE;
    let buttan3 = menu2.add_element(Button::new("Buttom 3".to_string(), Vec2{ x: 0.0, y: 30.0 }, None));
    let buttan4 = menu2.add_element(Button::new("Buttom 4".to_string(), Vec2{ x: 20.0, y: 0.0 }, None));
    loop {
        menu1.update();
        menu2.update();

        label1.edit().title.name = get_fps().to_string();
        if is_key_released(KeyCode::O) {
            let menu_menu = menu1.add_element(menu2);
            menu_menu.edit().color = BLUE;
            loop{
                menu1.update();
                menu1.draw();

                next_frame().await
            }
        }


        buttan1.edit().position.x += 0.1;

        menu2.draw();
        menu1.draw();
        next_frame().await
    }
}