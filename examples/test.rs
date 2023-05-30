use macroquad::prelude::*;
use Simples_menu::{Button, CheckBox, Menu, TextLabel};
use Simples_menu::PositionType::{Center, TopLeft};

#[macroquad::main("Test")]
async fn main() {
    let mut menu1 = Menu::new("Menu 1".to_string(), Vec2{ x: 50.0, y: 50.0 });
    //menu1.size = Some(Vec2 { x: 220.0, y: 80.0 });
    let buttan1 = menu1.add_element(Button::new("Buttom 1".to_string(),TopLeft,Vec2{ x: 0.0, y: 120.0 }, None));
    let buttan2 = menu1.add_element(Button::new("Buttom 2".to_string(),TopLeft,Vec2{ x: 0.0, y: 0.0 }, None));
    let label1 = menu1.add_element(TextLabel::new("Teste de legenda: ".to_string(), TopLeft, Vec2{ x: 30.0, y: 50.0 }));

    let mut menu2 = Menu::new("Menu 2".to_string(), Vec2{ x: 250.0, y: 260.0 });
    //menu2.color = BLUE;
    let buttan3 = menu2.add_element(Button::new("Buttom 3".to_string(), TopLeft, Vec2{ x: 0.0, y: 30.0 }, None));
    let buttan4 = menu2.add_element(Button::new("Buttom 4".to_string(), TopLeft,Vec2{ x: 20.0, y: 0.0 }, None));
    let check_box = menu2.add_element(CheckBox::new(Center, Vec2{ x: 20.0, y: 90.0 }, Vec2{ x: 20.0, y: 20.0 }));

    let mut menu3 = Menu::new("Menu 3".to_string(), Vec2{ x: 500.0, y: 260.0 });
    let buttan5 = menu3.add_element(Button::new("Buttom 5".to_string(), Center, Vec2{ x: 0.0, y: 0.0 }, None));


    loop {
        menu1.update();
        menu2.update();
        menu3.update();

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

        if check_box.read().is_checked {buttan1.edit().position.x += 0.1}

        menu3.draw();
        menu2.draw();
        menu1.draw();
        next_frame().await
    }
}