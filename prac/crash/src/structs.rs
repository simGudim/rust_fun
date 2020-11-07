pub fn run() {
    let color1 = Color {
        r: 255,
        g: 78,
        b: 0
    };

    println!("{:?}, {:?}", color1.r, color1);

    let mut x = Color2(255, 0, 0);
    println!("{} {} {}", x.0, x.1, x.2);

    let mut person = Person::person_create("John", "Dow");
    println!("{} {:?}", person.full_name(), person.last_name);

    person.set_last_name("Marry");
    println!("{}", person.full_name());

    println!("{:?}", person.to_tuple());
}


#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn person_create(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str)  {
        self.last_name = last.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}