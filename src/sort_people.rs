// I use Rust's powerful macro system to derive a ordering on People lexicographically, field by field.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct People {
    name: String, // Using #[derive(Ord)] People is sorted by name
    age: u32,     // Then by age, which is not exactly what was asked for, but good enough.
}

// To sort by age then by name lexicographically, I simply convert to PeopleAgeOrd and order on that instead.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct PeopleAgeOrd {
    age: u32,
    name: String,
}

// For conversion purposes.
impl From<(&str, u32)> for People {
    fn from((name, age): (&str, u32)) -> Self {
        Self {
            name: String::from(name),
            age: age,
        }
    }
}

impl From<People> for PeopleAgeOrd {
    fn from(peep: People) -> Self {
        Self {
            age: peep.age,
            name: peep.name,
        }
    }
}
