# Ukiyoe

A proof of concept GUI library using Pathfinder 3 and Stretch flexbox layout.

** this library is super experimental and in progress **

# Example

```rust
use ukiyoe::*;

fn main() {
    let mut window = Window::new();
    let mut b1 = Button::new("a");
    let b2 = Button::new("b");
    let mut b3 = Button::new("c");
    let b4 = Button::new("d");
    let b5 = Button::new("e");
    b1.add_child(Box::new(b2));
    b3.add_child(Box::new(b4));
    b3.add_child(Box::new(b5));
    b1.add_child(Box::new(b3));
    window.set_content(Box::new(b1));
    loop {
        let event = window.next_event();
        match event {
            WindowEvent::Exit => return,
            _ => {},
        }
        window.render();
    }
}
```
