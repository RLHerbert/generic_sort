mod sort_floats;
mod sort_people;

fn generic_sort<T: Ord>(v: &mut Vec<T>) {
    // Wow. What a sort function.
    v.sort();
}

fn main() {
    use sort_people::*;
    // let mut numbers: Vec<f64> = vec![
    //     645.41, 37.59, 76.41, 5.31, -34.23, 1.11, 1.10, 23.46, 635.47, -876.32, 467.83, 62.25,
    // ];

    // generic_sort(&mut numbers);
    // This won't work. IEEE floating point numbers aren't totally ordered.
    // Thanks infinity, NaN, +- 0.0.

    // Haha! Your sort is basically guaranteed to be ~~broken~~ (jk just incorrect on edge cases)
    // on floating point numbers in other languages!
    // (Unless they/you implement IEEE 754-2019 real careful like, unlikely).

    // Create our people.
    let peep_tups = vec![
        ("Hal", 20u32),
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

    // People are sorted by name first
    let mut peep_lex: Vec<People> = peep_tups
        .clone()
        .into_iter()
        .map(|peep| People::from(peep))
        .collect();

    generic_sort(&mut peep_lex);

    /*
    People { name: "Benny", age: 28 }
    People { name: "Cindy", age: 22 }
    People { name: "Cory", age: 27 }
    People { name: "Danna", age: 20 }
    People { name: "Doretha", age: 32 }
    People { name: "Dwight", age: 19 }
    People { name: "Hal", age: 20 }
    People { name: "Juan", age: 33 }
    People { name: "Lawrence", age: 25 }
    People { name: "Mac", age: 19 }
    People { name: "Natalie", age: 25 }
    People { name: "Risa", age: 24 }
    People { name: "Romana", age: 27 }
    People { name: "Rosalyn", age: 26 }
    People { name: "Susan", age: 31 }
    People { name: "Zara", age: 23 }
        */

    // for peep in peep_lex {
    //     println!("{:?}", peep);
    // }

    println!();

    // PeopleAgeOrd are ordered by age first.
    let mut peep_agelex: Vec<PeopleAgeOrd> = peep_tups
        .clone()
        .into_iter()
        .map(|peep| PeopleAgeOrd::from(People::from(peep)))
        .collect();

    generic_sort(&mut peep_agelex);

    /*
    PeopleAgeOrd { age: 19, name: "Dwight" }
    PeopleAgeOrd { age: 19, name: "Mac" }
    PeopleAgeOrd { age: 20, name: "Danna" }
    PeopleAgeOrd { age: 20, name: "Hal" }
    PeopleAgeOrd { age: 22, name: "Cindy" }
    PeopleAgeOrd { age: 23, name: "Zara" }
    PeopleAgeOrd { age: 24, name: "Risa" }
    PeopleAgeOrd { age: 25, name: "Lawrence" }
    PeopleAgeOrd { age: 25, name: "Natalie" }
    PeopleAgeOrd { age: 26, name: "Rosalyn" }
    PeopleAgeOrd { age: 27, name: "Cory" }
    PeopleAgeOrd { age: 27, name: "Romana" }
    PeopleAgeOrd { age: 28, name: "Benny" }
    PeopleAgeOrd { age: 31, name: "Susan" }
    PeopleAgeOrd { age: 32, name: "Doretha" }
    PeopleAgeOrd { age: 33, name: "Juan" }
    */

    // for peep in peep_agelex {
    //     println!("{:?}", peep);
    // }
}
