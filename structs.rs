pub fn run() {
    let user = User {
        username: String::from("fabio"),
        active: true,
    };

    println!("user: {}", user.username);

    let rect = Rectangle::new(10, 5);
    println!("area: {}", rect.area());
}

struct User {
    username: String,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }
}