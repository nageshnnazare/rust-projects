/*struct Color {
    red: u8,
    blue: u8,
    green: u8,
}
*/

// tuple struct
/*struct Color(u8, u8, u8);
*/

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    fn name_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    /*let mut color = Color {
        red: 255,
        blue: 0,
        green: 0,
    };

    color.red = 200;
    println!("{:?}", (color.red, color.blue, color.green));
    */
    /*
    let mut c = Color(255, 0, 0);
    c.0 = 100;
    println!("{:?}", (c.0, c.1, c.2));*/

    let p = Person::new("John", "Doe");
    println!("{:?}", p);

    println!("Full name : {:?}", p.full_name());

    let mut m = Person::new("Mary", "Doe");
    m.set_last_name("Williams");
    println!("Full name : {:?}", m.full_name());
    println!("Full name : {:?}", m.name_tuple());

}
