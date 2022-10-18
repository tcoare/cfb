#![allow(dead_code)]

#[derive(Debug)]
enum Year {
    Freshman
}

#[derive(Debug)]
struct Attributes {
}

#[derive(Debug)]
struct Player {
    name: String,
    age: i8,
    attributes: Attributes,
    year: Year
}

fn create_player(name: String, age: i8, year: Year) -> Player {
    Player { name: String::from(name), age: (age), attributes: (Attributes {  }), year: (year) }
}

fn main() {
    let quinn = String::from("Quinn Ewers");
    let first_name = &quinn[0..5];
    let last_name = &quinn[6..11];

    println!("{:?} {:?}", first_name, last_name);


    let quinn = Player { 
        name: String::from("Quinn Ewers"),
        age: 18,
        year: Year::Freshman,
        attributes: Attributes {},
    };

    println!("{:?}", quinn);
    
    let arch = create_player(String::from("Arch Manning"), 18, Year::Freshman);

    println!("{:?}", arch)
}
