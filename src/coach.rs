#![allow(dead_code)]

use crate::utils;

enum Playbook {
    Spread,
    IForm,
    Pro,
}

struct Coach {
    name: String,
    age: i8,
    playbook: Playbook
}

impl utils::Summary for Coach {
    fn summarise(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}
