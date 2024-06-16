use crate::variants::*;
use itertools::Itertools;

pub struct House {
    pub house_color: HouseColor,
    pub nationality: Nationality,
    pub cigarette: Cigarette,
    pub drink: Drink,
    pub pet: Pet,
}

impl std::fmt::Display for House {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The {} lives in the {} house, drinks {}, smokes {}, and has a {} for a pet.",
            self.nationality, self.house_color, self.drink, self.cigarette, self.pet
        )
    }
}

pub struct Constraints;
impl Constraints {
    pub fn new() -> Self {
        Self {}
    }

    pub fn meets_criteria<T: PartialEq + Copy, K: PartialEq + Copy>(
        &self,
        list_1: &[T],
        value_in_set_1: T,
        list_2: &[K],
        value_in_list_2: K,
    ) -> bool {
        (0..5).any(|i| list_1[i] == value_in_set_1 && list_2[i] == value_in_list_2)
    }

    pub fn next_to<T: PartialEq + Copy, K: PartialEq + Copy>(
        &self,
        list_1: &[T],
        value_in_list_1: T,
        list_2: &[K],
        value_in_list_2: K,
    ) -> bool {
        self.left_of(list_1, value_in_list_1, list_2, value_in_list_2)
            || self.right_of(list_1, value_in_list_1, list_2, value_in_list_2)
    }

    pub fn left_of<T: PartialEq + Copy, K: PartialEq + Copy>(
        &self,
        list_1: &[T],
        value_in_list_1: T,
        list_2: &[K],
        value_in_list_2: K,
    ) -> bool {
        (0..4).any(|i| list_1[i] == value_in_list_1 && list_2[i + 1] == value_in_list_2)
    }

    pub fn right_of<T: PartialEq + Copy, K: PartialEq + Copy>(
        &self,
        list_1: &[T],
        value_in_list_1: T,
        list_2: &[K],
        value_in_list_2: K,
    ) -> bool {
        (1..5).any(|i| list_1[i] == value_in_list_1 && list_2[i - 1] == value_in_list_2)
    }

    pub fn nationality_constraints(
        &self,
        nationalities: &[Nationality],
        colors: &[HouseColor],
    ) -> bool {
        nationalities[0] == Nationality::Norwegian
            && self.meets_criteria(nationalities, Nationality::British, colors, HouseColor::Red)
            && self.left_of(
                nationalities,
                Nationality::Norwegian,
                colors,
                HouseColor::Blue,
            )
    }

    pub fn pet_constraints(&self, pets: &[Pet], nationalities: &[Nationality]) -> bool {
        self.meets_criteria(pets, Pet::Dog, nationalities, Nationality::Swedish)
    }

    pub fn drink_constraints(
        &self,
        drinks: &[Drink],
        colors: &[HouseColor],
        nationalities: &[Nationality],
    ) -> bool {
        self.meets_criteria(nationalities, Nationality::Danish, drinks, Drink::Tea)
            && self.meets_criteria(colors, HouseColor::Green, drinks, Drink::Coffee)
            && drinks[2] == Drink::Milk
    }

    pub fn cigarette_constraints(
        &self,
        cigarettes: &[Cigarette],
        nationalities: &[Nationality],
        colors: &[HouseColor],
        pets: &[Pet],
        drinks: &[Drink],
    ) -> bool {
        self.meets_criteria(cigarettes, Cigarette::PallMall, pets, Pet::Bird)
            && self.meets_criteria(colors, HouseColor::Yellow, cigarettes, Cigarette::Dunhill)
            && self.next_to(cigarettes, Cigarette::Brends, pets, Pet::Cat)
            && self.next_to(pets, Pet::Horse, cigarettes, Cigarette::Dunhill)
            && self.meets_criteria(cigarettes, Cigarette::Bluemasters, drinks, Drink::Beer)
            && self.meets_criteria(
                nationalities,
                Nationality::German,
                cigarettes,
                Cigarette::Prince,
            )
            && self.next_to(cigarettes, Cigarette::Brends, drinks, Drink::Water)
    }
}

pub struct Possibilities {
    colors: Vec<Vec<HouseColor>>,
    nationalities: Vec<Vec<Nationality>>,
    cigarettes: Vec<Vec<Cigarette>>,
    drinks: Vec<Vec<Drink>>,
    pets: Vec<Vec<Pet>>,
}

impl Possibilities {
    pub fn new() -> Self {
        let colors = COLORS.into_iter().permutations(5).collect::<Vec<_>>();
        let nationalities = NATIONALITIES
            .into_iter()
            .permutations(5)
            .collect::<Vec<_>>();
        let cigarettes = CIGARETTES.into_iter().permutations(5).collect::<Vec<_>>();
        let drinks = DRINKS.into_iter().permutations(5).collect::<Vec<_>>();
        let pets = PETS.into_iter().permutations(5).collect::<Vec<_>>();
        Self {
            colors,
            nationalities,
            cigarettes,
            drinks,
            pets,
        }
    }

    pub fn solve(&self) -> Option<Vec<House>> {
        let start = std::time::Instant::now();
        let constraints = Constraints::new();
        let mut attempts = 0;

        for color_set in &self.colors {
            if !constraints.left_of(color_set, HouseColor::Green, color_set, HouseColor::White) {
                continue;
            }

            for nationality_set in &self.nationalities {
                if !constraints.nationality_constraints(nationality_set, color_set) {
                    continue;
                }

                for pet_set in &self.pets {
                    if !constraints.pet_constraints(pet_set, nationality_set) {
                        continue;
                    }

                    for drink_set in &self.drinks {
                        if !constraints.drink_constraints(drink_set, color_set, nationality_set) {
                            continue;
                        }

                        for cigar_set in &self.cigarettes {
                            attempts += 1;
                            if !constraints.cigarette_constraints(
                                cigar_set,
                                nationality_set,
                                color_set,
                                pet_set,
                                drink_set,
                            ) {
                                continue;
                            }
                            let solution: Vec<House> = (0..5)
                                .map(|i| House {
                                    nationality: nationality_set[i],
                                    house_color: color_set[i],
                                    drink: drink_set[i],
                                    cigarette: cigar_set[i],
                                    pet: pet_set[i],
                                })
                                .collect();
                            let elapsed = start.elapsed().as_nanos();
                            println!(
                                "solved in {} attempts in {} ns ({} ms)",
                                attempts,
                                elapsed,
                                elapsed / 1_000_000
                            );
                            return Some(solution);
                        }
                    }
                }
            }
        }

        None
    }
}
