use std::fmt;
use std::mem;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    Upkey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::Upkey(String::from("Pressed W")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed S")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed A")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed D")),
        }
    }
}

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Object {
        Object {
            width: width,
            height: height,
        }
    }

    fn show(&self) {
        println!("{} * {} = {} ", self.width, self.height, self.area());
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn main() {
    println!("Hello, world!");

    let t = (1, 1.2, false);
    let m = (2, (3, 4.56, true));

    println!("{} {} {}", t.0, t.1, t.2);
    println!("{:?}", m.1);

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));

    let ys = &xs[2..5];
    println!("{:?}", ys);

    let ss = String::from("String!");

    let slice = &ss[0..3];
    println!("{:?}", slice);

    let o = Object {
        width: 35,
        height: 20,
    };
    println!("{:#?}", o);
    println!("{:?}", o.area());

    let obj = Object::new(57, 83);
    println!("{}", obj);
    obj.show();

    let u = Direction::Up(Point { x: 0, y: 1 });
    let k = u.match_direction();

    println!("{:?}", k);
}
