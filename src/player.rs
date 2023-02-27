#![allow(dead_code)]

// literally everything in here is public, probably want to add getters for things
// we might not want to be

#[derive(Debug)]
pub enum Year {
    Freshman,
    Sophomore,
    Junior,
    Senior
}

impl Year {
    fn to_string(&self) -> &str {
        match self {
            Year::Freshman => "Freshman",
            Year::Sophomore => "Sophomore",
            Year::Junior => "Junior",
            Year::Senior => "Senior"
        }
    }
}


// using default so we can get an empty attributes without having to type them out
#[derive(Debug, Default)]
pub struct Attributes {
    pub throw_power: i8,
    pub throw_accuracy: i8
}

impl Attributes {
    // interesting way of manipulating types of structs?
    // or just bad way of using structs?
    pub fn as_array(&self) -> [i8; 2] {
        [self.throw_power, self.throw_accuracy]
    }

    pub fn highest_attribute(&self) -> i8 {
        // not really sure how this works yet, and if you are allwed to return references or not
        self.as_array().into_iter().max().unwrap()
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub age: i8,
    pub attributes: Attributes,
    pub year: Year,
    pub redshirt: bool,
    pub redshirting: bool
}

// current impl, we get years using year() method, this looks if the player has redshirted
// when we advance a year/season players that were redshirting are no longer and
// redshirt is given true and we dont increase year

impl Player {
    pub fn year(&self) -> String {
        if self.redshirt {
            let year = self.year.to_string();
            return String::from("Redshirt ") + year
        } else {
            return String::from(self.year.to_string()) 
        }
    }

    // Do we want this available on every player?
    pub fn advance_year(&mut self) {
        // We would also need to calculate appearances as players can redshirt
        // if they have only played 4 or less games
        if self.redshirting {
            // if they are redshirting we remove redshirt and don't update their 
            // year
            self.redshirting = false;
            self.redshirt = true;
            return;
        }

        match self.year {
            Year::Freshman => self.year = Year::Sophomore,
            Year::Sophomore => self.year = Year::Junior,
            Year::Junior => self.year = Year::Senior,
            _ => println!("Senior graduates")
        }
    }

    pub fn new(name: String, age: i8, year: Year, redshirt: bool) -> Player {
        // using field init shorthand
        Player { 
            name,
            age,
            attributes: Attributes { ..Default::default() },
            year,
            redshirt,
            redshirting: false
        }
    }
}

