use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use macroquad::prelude::*;
use macros::*;

pub trait MenuElement {
    fn update(&mut self, menu_position: Vec2);
    fn draw(&self, menu_position: Vec2);
    fn bounding_rect(&self) -> Option<Rect>;
}

pub struct Title {
    pub name: String,
    pub color: Color,
    pub font_size: f32
}
impl Title {
    ///Return the size needed to draw this ´Title´ on the screen.
    fn size(&self) -> Vec2 {
        let text_size = measure_text(&self.name, None, self.font_size as u16, 1.0);
        Vec2{
            x: text_size.width,
            y: text_size.height,
        }
    }
}

#[derive(Clone)]
pub struct Element<T: MenuElement + ?Sized> {
    id: usize,
    data: Rc<RefCell<T>>,
}
impl<T: MenuElement + ?Sized> Element<T> {
    ///Return a immutable reference to the element.
    pub fn read(&self) -> Ref<T> {
        self.data.deref().borrow()
    }
    ///Return a mutable reference to the element.
    pub fn edit(&self) -> RefMut<T> {
        self.data.deref().borrow_mut()
    }


    pub fn clone(&self) -> Element<T> {
        Element{
            id: self.id,
            data: self.data.clone(),
        }
    }
}

#[derive(BoundingRect)]
pub struct Button {
    pub title: Title,
    pub visible: bool,
    pub color: Color,
    pub position: Vec2,
    pub size: Vec2,

    pub is_pressed: bool,

    pub has_been_pressed: bool,
    pub action: Option<fn()>,

    visible_color: Color,
    // other properties specific to buttons
}
impl Button {
    ///Create a new button with the default arguments.
    pub fn new(lable: String, position: Vec2, size: Option<Vec2>) -> Button {
        let label_title = Title {
            name: lable,
            color: WHITE,
            font_size: 13.0,
        };
        Button{
            size: size.unwrap_or(label_title.size() + Vec2{ x: 10.0, y: 10.0 }),
            is_pressed: false,
            title: label_title,
            visible: true,
            color: GRAY,
            visible_color: GRAY,
            position: position,
            action: Some(|| println!("Button has been pressed")),
            has_been_pressed: false,
        }
    }
}
impl MenuElement for Button {
    fn update(&mut self, menu_position: Vec2) {
        self.is_pressed = false;
        self.has_been_pressed = false;
        self.visible_color = self.color;

        let button_position = self.position + menu_position;
        let button_rect = Rect{
            x: button_position.x,
            y: button_position.y,
            w: self.size.x,
            h: self.size.y,
        };

        let mouse_posi:Vec2;
        {
            let mouse_flot = mouse_position();
            mouse_posi = Vec2{ x: mouse_flot.0, y: mouse_flot.1 }
        }
        if button_rect.contains(mouse_posi) {
            self.visible_color = Color{
                r: self.color.r - 0.1,
                g: self.color.g - 0.1,
                b: self.color.b - 0.1,
                a: self.color.a,
            };

            if is_mouse_button_pressed(MouseButton::Left) {
                self.has_been_pressed = true;
                if let Some(action) = self.action {
                    action();
                }
            }
            if is_mouse_button_down(MouseButton::Left) {
                self.is_pressed = true;
            }
        }
    }

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
pub struct CheckBox {
    pub visible: bool,
    pub position: Vec2,
    pub size: Vec2,
    pub is_checked: bool,
    pub color: Color,

    visible_color: Color,
}
impl CheckBox {
    pub fn new(position: Vec2, size: Vec2) -> CheckBox {
        CheckBox{
            visible: true,
            position,
            size,
            is_checked: false,
            color: LIGHTGRAY,

            visible_color: LIGHTGRAY,
        }
    }
}
impl MenuElement for CheckBox {
    fn update(&mut self, menu_position: Vec2) {
        self.visible_color = self.color;
        let position = self.position + menu_position;
        let check_box_rect = Rect{
            x: position.x,
            y: position.y,
            w: self.size.x,
            h: self.size.y,
        };

        let mouse_posi:Vec2;
        {
            let mouse_flot = mouse_position();
            mouse_posi = Vec2{ x: mouse_flot.0, y: mouse_flot.1 }
        }
        if check_box_rect.contains(mouse_posi) {
            self.visible_color = Color {
                r: self.color.r - 0.1,
                g: self.color.g - 0.1,
                b: self.color.b - 0.1,
                a: self.color.a,
            };

            if is_mouse_button_pressed(MouseButton::Left) {
                self.is_checked = !self.is_checked;
            }
        }
    }

    fn draw(&self, menu_position: Vec2) {
        let position = self.position + menu_position;
        let check_box_rect = Rect{
            x: position.x,
            y: position.y,
            w: self.size.x,
            h: self.size.y,
        };
        let center = check_box_rect.center();
        let radius = {
            if self.size.x < self.size.y {
                self.size.x
            }else {
                self.size.y
            }
        };

        draw_rectangle(position.x,
                       position.y,
                       self.size.x,
                       self.size.y,
                       self.visible_color
        );
        if self.is_checked {
            draw_circle(center.x, center.y, (radius / 2.0) - (radius / 10.0), GREEN)
        }
    }

    fn bounding_rect(&self) -> Option<Rect> {
        self.bounding_rect()
    }
}

#[derive(BoundingRect)]
pub struct TextLabel {
    pub title: Title,
    pub visible: bool,
    pub position: Vec2,

    size: Vec2,
}
impl TextLabel {
    ///Create a new text label with the default arguments.
    pub fn new(lable: String, position: Vec2) -> TextLabel {
        let label_title = Title {
            name: lable,
            color: WHITE,
            font_size: 13.0,
        };
        TextLabel {
            size: label_title.size(),
            title: label_title,
            visible: true,
            position: position,
        }
    }
}
impl MenuElement for TextLabel {
    fn update(&mut self, menu_position: Vec2) {
        self.size = self.title.size();
    }

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

pub struct Menu {
    pub title: Title,
    pub visible: bool,
    pub color: Color,
    pub edge: f32,
    pub position: Vec2,
    pub size: Option<Vec2>,
    pub elements: Vec<Element<dyn MenuElement>>,

    visible_color: Color,
}
impl Menu {
    ///Create a new menu with the default arguments.
    pub fn new(name: String, position: Vec2) -> Menu {
        Menu{
            title: Title {
                name: name,
                color: WHITE,
                font_size: 25.0,
            },
            visible: true,
            color: DARKGRAY,
            edge: 20.0,
            position: position,
            size: None,
            elements: vec![],

            visible_color: DARKGRAY,
        }
    }
    ///Adds a new menu element to the **menu.elements** vector,
    ///and return a Element object that can be edited later.
    pub fn add_element<T: MenuElement + 'static>(&mut self, element: T) ->  Element<T> {
        let element_ref = Rc::new(RefCell::new(element));
        let element = Element{
            id: self.elements.len(),
            data: element_ref.clone(),
        };
        self.elements.push(Element { id: self.elements.len(), data: element_ref });

        element
    }

    fn calculate_menu_rect(&self) -> (Rect, Rect) {
        // Calculate the bounding rectangle for the menu.
        let mut elements_rect = Rect::new(0.0, 0.0, 0.0, 0.0);

        // Iterate over each element in the menu.
        for element in &self.elements {
            // Get the bounding rectangle for the element.
            if let Some(rect) = element.read().bounding_rect() {
                // Combine the element rectangle with the overall menu rectangle.
                elements_rect = elements_rect.combine_with(rect);
            }
        }

        // Get the size of the menu title text.
        let text_size = measure_text(
            &self.title.name,
            None,
            self.title.font_size as u16,
            1.0
        );

        // Create a rectangle for the menu title.
        let title_rect = Rect {
            x: elements_rect.x + elements_rect.w / 2.0,
            y: elements_rect.y,
            w: text_size.width,
            h: text_size.height,
        };

        // Combine the menu title rectangle with the overall menu rectangle.
        elements_rect = elements_rect.combine_with(title_rect);

        // If the menu has a specified size, use it to set the menu rectangle.
        if let Some(size) = self.size {
            elements_rect = Rect {
                x: elements_rect.x,
                y: elements_rect.y,
                w: size.x,
                h: size.y,
            };
        }

        // Offset the menu rectangle by the menu position and return it.
        let menu_rect = Rect {
            x: elements_rect.x + self.position.x,
            y: elements_rect.y + self.position.y,
            w: elements_rect.w,
            h: elements_rect.h,
        };

        (menu_rect, title_rect)
    }

    pub fn update(&mut self) {
        self.visible_color = self.color;

        //Remove elements out of scope
        let mut index_to_delete: Vec<usize> = vec![];
        for element_ref in &self.elements {
            if Rc::strong_count(&element_ref.data) <= 1 {
                index_to_delete.push(element_ref.id);
            }
        }
        for index in index_to_delete {
            let delet = self.elements.remove(index);
        }

        let (menu_rect, menu_tile_rect) = self.calculate_menu_rect();
        for element_ref in self.elements.iter() {
            let mut element = element_ref.edit();

            element.update(vec2(menu_rect.x, menu_rect.y + menu_tile_rect.h))
        }
    }

    ///Draw the menu
    pub fn draw(&self) {
        if !self.visible {
            return;
        }
        let (menu_rect, menu_title_rect) = self.calculate_menu_rect();

        let menu_bg_rect = Rect{
            x: menu_rect.x - self.edge,
            y: menu_rect.y - self.edge,
            w: menu_rect.w + (self.edge * 2.0),
            h: menu_rect.h + (self.edge * 2.0) + menu_title_rect.h,
        };

        // draw the menu background
        draw_rectangle(menu_bg_rect.x, menu_bg_rect.y, menu_bg_rect.w, menu_bg_rect.h, self.visible_color);

        // draw the menu name
        let name_position = vec2(menu_rect.x + (menu_rect.w - menu_title_rect.w) / 2.0, menu_rect.y );
        draw_text(&self.title.name, name_position.x, name_position.y, self.title.font_size, self.title.color);

        // draw the menu elements
        for element in &self.elements {
            element.read().draw(Vec2{ x: menu_rect.x, y: menu_rect.y + menu_title_rect.h});
        }
    }
}
impl MenuElement for Menu {
    fn update(&mut self, menu_position: Vec2) {
        self.update();
    }

    fn draw(&self, menu_position: Vec2) {
        if !self.visible {
            return;
        }
        let (mut menu_rect, mut menu_title_rect) = self.calculate_menu_rect();
        menu_rect = Rect{
            x: menu_rect.x + menu_position.x,
            y: menu_rect.y + menu_position.y,
            w: menu_rect.w,
            h: menu_rect.h,
        };
        menu_title_rect = Rect{
            x: menu_title_rect.x + menu_position.x,
            y: menu_title_rect.y + menu_position.y,
            w: menu_title_rect.w,
            h: menu_title_rect.h,
        };

        let menu_bg_rect = Rect{
            x: menu_rect.x - self.edge,
            y: menu_rect.y - self.edge,
            w: menu_rect.w + (self.edge * 2.0),
            h: menu_rect.h + (self.edge * 2.0) + menu_title_rect.h,
        };

        // draw the menu background
        draw_rectangle(menu_bg_rect.x, menu_bg_rect.y, menu_bg_rect.w, menu_bg_rect.h, self.visible_color);

        // draw the menu name
        let name_position = vec2(menu_rect.x + (menu_rect.w - menu_title_rect.w) / 2.0, menu_rect.y );
        draw_text(&self.title.name, name_position.x, name_position.y, self.title.font_size, self.title.color);

        // draw the menu elements
        for element in &self.elements {
            element.read().draw(Vec2{ x: menu_rect.x, y: menu_rect.y + menu_title_rect.h});
        }
    }

    fn bounding_rect(&self) -> Option<Rect> {
        if !self.visible {return None}
        let (menu_rect, text_rect) = self.calculate_menu_rect();

        Some(menu_rect)
    }
}