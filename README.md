# Simple_menu Rust Library

Simple_menu is a rust library for creating interactive menus with buttons, check boxes, and text labels. The library provides an easy-to-use API for creating and managing menus, allowing you to quickly add interactive elements to your rust applications.

## Usage

To use Simple_menu in your rust project, simply add the following line to your `Cargo.toml` file:

```toml
Simple_menu = "*"
```

Then, in your rust code, you can import the Simple_menu library using:

```rust
use simple_menu::*;
```

To create a new menu, you can use the Menu::new function:

```rust
let mut menu = Menu::new("Menu 1".to_string(), Vec2{ x: 50.0, y: 50.0 });
```

To add elements to the menu, you can use the add_element function:

```rust
let buttan = menu1.add_element(Button::new("Buttom 1".to_string(), Vec2{ x: 0.0, y: 120.0 }, None));
let label = menu1.add_element(TextLabel::new("Teste de legenda: ".to_string(), Vec2{ x: 30.0, y: 50.0 }));
```

You can update and draw the menu in a loop using menu.update() and menu.draw():

```rust
loop{ // Your game loop
    menu.update()

    /*
        Your code
    */

    menu.draw()
    next_frame().await // next_frame() from MacroQuad
}
```
