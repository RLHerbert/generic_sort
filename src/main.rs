// CECS 342 - Spring 2021 - Lab Assignment 3
// Generic sorting in Rust.
//
// In which I provide several different approaches to generically sorting in Rust.

mod people;
use people::{generic::*, simple::People};

fn main() {
    use std::cmp;

    // Numbers
    {
        let mut numbers = example_floats();
        // As it turns out, Rust's standard library, std, does not implement Ord on its implementation of IEEE floating point numbers.
        // This is because the IEEE definition of FP numbers is not totally ordered. Rust does implement PartialOrd for FP numbers.
        // We can thus sort by using a Vec's sort_by method (which I do below), or by wrapping our floats in another type and
        // implementing Ord on that.

        println!("Numbers Unsorted:");
        println!("{:?}", numbers);

        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

        println!();
        println!("Numbers Sorted:");
        println!("{:?}", numbers);
    }

    // People
    {
        let mut people = example_people();

        println!();

        println!("People Unsorted:");
        println!("{:#?}", people);

        people.sort();

        println!();
        println!("People Sorted (Lexicographically):");
        println!("{:#?}", people);

        let age_descending = |s: &People, o: &People| match s.age.cmp(&o.age) {
            cmp::Ordering::Less => cmp::Ordering::Greater,
            cmp::Ordering::Equal => s.name.cmp(&o.name),
            cmp::Ordering::Greater => cmp::Ordering::Less,
        };

        people.sort_by(age_descending);

        println!();
        println!("People Sorted (Age Descending):");
        println!("{:#?}", people);
    }

    // GenericPeople<Ordering>
    {
        let people = example_people();

        println!();

        // Ordering = Lexicographic
        {
            let mut people_lex_order: Vec<GenericPeople<Lexicographic>> = people
                .clone()
                .into_iter()
                .clone()
                .map(|p| p.into())
                .collect();

            println!();
            println!("GenericPeople<Lexicographic> Unsorted: ");
            println!("{:#?}", people_lex_order);

            people_lex_order.sort();

            println!();
            println!("GenericPeople<Lexicographic> Sorted: ");
            println!("{:#?}", people_lex_order);
        }
        // Ordering = AgeDescending
        {
            let mut people_age_order: Vec<GenericPeople<AgeDescending>> = people
                .clone()
                .into_iter()
                .clone()
                .map(|p| p.into())
                .collect();

            println!();
            println!("GenericPeople<AgeDescending> Unsorted: ");
            println!("{:#?}", people_age_order);

            people_age_order.sort();

            println!();
            println!("GenericPeople<AgeDescending> Sorted: ");
            println!("{:#?}", people_age_order);
        }
    }
}

fn example_floats() -> Vec<f64> {
    vec![
        645.41, 37.59, 76.41, 5.31, -34.23, 1.11, 1.10, 23.46, 635.47, -876.32, 467.83, 62.25,
    ]
}

fn example_people() -> Vec<People> {
    let people_tuples = vec![
        ("Hal", 20u32), // To force the rest of the ages to be 32 bit unsigned integers. i32 otherwise.
        ("Susan", 31),
        ("Dwight", 19),
        ("Lawrence", 25),
        ("Cindy", 22),
        ("Cory", 27),
        ("Mac", 19),
        ("Romana", 27),
        ("Doretha", 32),
        ("Danna", 20),
        ("Zara", 23),
        ("Rosalyn", 26),
        ("Risa", 24),
        ("Benny", 28),
        ("Juan", 33),
        ("Natalie", 25),
    ];
    people_tuples
        .clone()
        .into_iter()
        .map(|peep| peep.into())
        .collect()
}
