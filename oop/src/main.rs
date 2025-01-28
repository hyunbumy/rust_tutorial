use oop::{Draw, Screen, Button};

struct SelectionBox {
    width: u32,
    height: u32,
}

impl Draw for SelectionBox {
    fn draw(&self) {
        println!("select");
    }
}

fn main() {
    // Instantiate a screen object with structs that implement `Draw`
    let screen = Screen {
        components: vec![
            Box::new(SelectionBox {
                width: 75,
                height: 10,
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // Type inference is a bit more tricky with traits.
    // `let componenets = vec![Box::new(SelectionBox{})];`
    // does not work anymore since compiler doesn't know what it's supposed to
    // conform to.
    // Instead, be explicit:
    /*
    let components = vec![
        Box::new(SelectBox { /* .. */ }) as Box<dyn Draw>,
        Box::new(Button { /* .. */ }),
    ];

    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(SelectBox { /* .. */ }),
        Box::new(Button { /* .. */ }),
    ];
     */
    // Note that using traits incur dynamic dispatch where Rust uses the
    // pointers inside the trait object to know which method to call at runtime,
    // which incurs some cost in addition to other optimization prevention.
}
