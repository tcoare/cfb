#![allow(dead_code)]

mod team {
    pub mod player;
    pub mod coach;
}

mod traits;

use crate::traits::Summary;
use team::player::{Player, Year, Attributes};

fn main() {
    player_testing()
}


fn player_testing() {
    let mut quinn = Player {
        name: "Quinn Ewers".to_string(),
        age: 18,
        year: Year::Freshman,
        attributes: Attributes {
            throw_power: 12,
            throw_accuracy: 14,
        },
        redshirt: true,
        redshirting: false,
    };
    println!("{:?}", quinn);

    let mut arch = Player::new(
        String::from("Arch Manning"),
        18,
        Year::Freshman,
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

    quinn.summarise();
}
