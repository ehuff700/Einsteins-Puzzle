use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HouseColor {
    Red,
    Green,
    Blue,
    Yellow,
    White,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nationality {
    British,
    Swedish,
    Danish,
    Norwegian,
    German,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Drink {
    Tea,
    Coffee,
    Milk,
    Beer,
    Water,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pet {
    Dog,
    Bird,
    Cat,
    Horse,
    Fish,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cigarette {
    PallMall,
    Dunhill,
    Brends,
    Bluemasters,
    Prince,
}

pub const COLORS: [HouseColor; 5] = [
    HouseColor::Red,
    HouseColor::Green,
    HouseColor::Blue,
    HouseColor::Yellow,
    HouseColor::White,
];
pub const NATIONALITIES: [Nationality; 5] = [
    Nationality::British,
    Nationality::Swedish,
    Nationality::Danish,
    Nationality::Norwegian,
    Nationality::German,
];
pub const DRINKS: [Drink; 5] = [
    Drink::Tea,
    Drink::Coffee,
    Drink::Milk,
    Drink::Beer,
    Drink::Water,
];
pub const CIGARETTES: [Cigarette; 5] = [
    Cigarette::PallMall,
    Cigarette::Dunhill,
    Cigarette::Brends,
    Cigarette::Bluemasters,
    Cigarette::Prince,
];
pub const PETS: [Pet; 5] = [Pet::Dog, Pet::Bird, Pet::Cat, Pet::Horse, Pet::Fish];

impl fmt::Display for HouseColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HouseColor::Red => "Red",
            HouseColor::Green => "Green",
            HouseColor::Blue => "Blue",
            HouseColor::Yellow => "Yellow",
            HouseColor::White => "White",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Nationality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Nationality::British => "British",
            Nationality::Swedish => "Swedish",
            Nationality::Danish => "Danish",
            Nationality::Norwegian => "Norwegian",
            Nationality::German => "German",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Drink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Drink::Tea => "Tea",
            Drink::Coffee => "Coffee",
            Drink::Milk => "Milk",
            Drink::Beer => "Beer",
            Drink::Water => "Water",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Pet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Pet::Dog => "Dog",
            Pet::Bird => "Bird",
            Pet::Cat => "Cat",
            Pet::Horse => "Horse",
            Pet::Fish => "Fish",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Cigarette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cigarette::PallMall => "Pall Mall",
            Cigarette::Dunhill => "Dunhill",
            Cigarette::Brends => "Brends",
            Cigarette::Bluemasters => "Bluemasters",
            Cigarette::Prince => "Prince",
        };
        write!(f, "{}", s)
    }
}
