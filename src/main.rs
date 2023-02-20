#![allow(dead_code)]

mod player;

fn main() {
    let mut quinn = player::Player {
        name: "Quinn Ewers".to_string(),
        age: 18,
        year: player::Year::Freshman,
        attributes: player::Attributes {
            throw_power: 12,
            throw_accuracy: 14,
        },
        redshirt: true,
        redshirting: false,
    };
    println!("{:?}", quinn);

    let mut arch = player::create_player(
        String::from("Arch Manning"),
        18,
        player::Year::Freshman,
        false,
    );
    println!("{:?}", arch);
    println!("{:?}", arch.year());

    let x = quinn.attributes.highest_attribute();
    println!("{:?}", quinn.year());
    println!("{:?}", x);

    quinn.advance_year();
    arch.advance_year();
    println!("{:?}", quinn.year());
    println!("{:?}", arch.year());
}
