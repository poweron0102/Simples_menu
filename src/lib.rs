use macroquad::prelude::*;
use macros::*;

pub trait Drawable {
    fn draw(&self, start_position: Vec2);
    fn bounding_rect(&self) -> Option<Rect>;
}

pub struct Title {
    name: String,
    color: Color,
    font_size: f32
}
impl Title {
    fn size(&self) -> Vec2 {
        let text_size = measure_text(&self.name, None, self.font_size as u16, 1.0);
        Vec2{
            x: text_size.width,
            y: text_size.height,
        }
    }
}

#[derive(BoundingRect)]
pub struct Button {
    pub title: Title,
    pub visible: bool,
    pub position: Vec2,
    pub size: Vec2,

    visible_color: Color,
    // other properties specific to buttons
}
impl Button {
    pub fn new(lable: String, position: Vec2, size: Option<Vec2>) -> Button {
        let label_title = Title {
            name: lable,
            color: WHITE,
            font_size: 13.0,
        };
        Button{
            size: size.unwrap_or(label_title.size() + Vec2{ x: 10.0, y: 10.0 }),
            title: label_title,
            visible: true,
            visible_color: GRAY,
            position: position,
        }
    }
}
impl Drawable for Button {
    fn draw(&self, start_position: Vec2) {
        let position = self.position + start_position;

        draw_rectangle(position.x,
                       position.y,
                       self.size.x,
                       self.size.y,
                       self.visible_color);

        let text_size = measure_text(&self.title.name, None, self.title.font_size as u16, 1.0);
        draw_text(&self.title.name,
                  position.x + (self.size.x - text_size.width) / 2.0,
                  position.y + (self.size.y + text_size.height) / 2.0,
                  self.title.font_size,
                  self.title.color)
    }
    
    fn bounding_rect(&self) -> Option<Rect>{
        self.bounding_rect()
    }
}

#[derive(BoundingRect)]
pub struct Text_label {
    pub title: Title,
    pub visible: bool,
    pub position: Vec2,

    size: Vec2,
}
impl Text_label {
    pub fn new(lable: String, position: Vec2) -> Text_label {
        let label_title = Title {
            name: lable,
            color: WHITE,
            font_size: 13.0,
        };
        Text_label{
            size: label_title.size(),
            title: label_title,
            visible: true,
            position: position,
        }
    }
}
impl Drawable for Text_label {
    fn draw(&self, start_position: Vec2) {
        let position = self.position + start_position;

        draw_text(&self.title.name,
                  position.x,
                  position.y,
                  self.title.font_size,
                  self.title.color)
    }

    fn bounding_rect(&self) -> Option<Rect> {
        self.bounding_rect()
    }
}

/*
#[derive(BoundingRect)]
struct Checkbox {
    //title: String,
    visible: bool,
    position: Vec2,
    size: Vec2,
    checked: bool,
    // other properties specific to checkboxes
}

impl Drawable for Checkbox {
    fn draw(&self, menu_position: Vec2) {
        // implementation for drawing a checkbox
    }
    fn bounding_rect(&self) -> Option<Rect>{
        self.bounding_rect()
    }
}

// other element types and their Drawable implementations...
*/

pub struct Menu {
    pub title: Title,
    pub visible: bool,
    pub color: Color,
    pub position: Vec2,
    pub size: Option<Vec2>,
    pub elements: Vec<Box<dyn Drawable>>,

    visible_color: Color,
}

impl Menu {
    pub fn new(name: String, position: Vec2) -> Menu {
        Menu{
            title: Title {
                name: name,
                color: WHITE,
                font_size: 25.0,
            },
            visible: true,
            color: DARKGRAY,
            position: position,
            size: None,
            elements: vec![],

            visible_color: DARKGRAY,
        }
    }

    pub fn add_element< 'a, T: Drawable + 'static >(&'a mut self, element: T) -> Box<T> {
        let mut element_ref = Box::new((element));
        self.elements.push(element_ref);

        element_ref
    }

    pub fn draw(&self) {
        if !self.visible {
            return;
        }

        // calculate the bounding rectangle for the menu
        let mut menu_rect = Rect::new(0.0, 0.0, 0.0, 0.0);
        for element in &self.elements {
            if let Some(rect) = element.bounding_rect() {
                menu_rect = menu_rect.combine_with(rect);
            }
        }
        let text_size = measure_text(&self.title.name, None, self.title.font_size as u16, 1.0);
        let menu_title_rect = Rect{
            x: menu_rect.x + menu_rect.w / 2.0,
            y: menu_rect.y,
            w: text_size.width,
            h: text_size.height,
        };
        menu_rect = menu_rect.combine_with(menu_title_rect);

        if let Some(size) = self.size {
            menu_rect = Rect{
                x: menu_rect.x,
                y: menu_rect.y,
                w: size.x,
                h: size.y,
            };
        }
        menu_rect = Rect{
            x: menu_rect.x + self.position.x,
            y: menu_rect.y + self.position.y,
            w: menu_rect.w,
            h: menu_rect.h,
        };

        let menu_bg_rect = Rect{
            x: menu_rect.x - 20.0,
            y: menu_rect.y - 20.0,
            w: menu_rect.w + 40.0,
            h: menu_rect.h + 40.0 + menu_title_rect.h,
        };

        // draw the menu background
        draw_rectangle(menu_bg_rect.x, menu_bg_rect.y, menu_bg_rect.w, menu_bg_rect.h, self.visible_color);

        // draw the menu name
        let name_position = vec2(menu_rect.x + (menu_rect.w - menu_title_rect.w) / 2.0, menu_rect.y );
        draw_text(&self.title.name, name_position.x, name_position.y, self.title.font_size, self.title.color);

        // draw the menu elements
        for element in &self.elements {
            element.draw(Vec2{ x: menu_rect.x, y: menu_rect.y + menu_title_rect.h});
        }
    }
}

/*
pub struct MenuManager {
    menus: HashMap<String , Menu>
}

impl MenuManager {
    pub fn add_menu(& mut self, menu: Menu) {
        self.menus.insert(&menu.title.name, menu)
    }

    pub fn draw(&self) {
        for menu in &self.menus {
            menu.draw();
        }
    }
}
*/