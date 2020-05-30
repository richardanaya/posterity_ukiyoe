# ukiyoe

<a href="https://docs.rs/ukiyoe"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A proof of concept GUI framework.  Plugin your own renderer!

**this library is super experimental and in progress**

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

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `ukiyoe` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
