use possibilities::Possibilities;

mod possibilities;
mod puzzle_types;

fn main() {
    let possibilities = Possibilities::new();
    if let Some(houses) = possibilities.solve() {
        for house in houses {
            println!("{house}");
        }
    } else {
        println!("no solution found")
    }
}
