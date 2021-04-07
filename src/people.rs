use std::marker::PhantomData;
use std::{cmp, fmt::Debug};

// I provide two types of the People struct. The first is a simple, non generic struct, which I use to show how Rust's Vec type
// can sort our data. The second is generic over the ordering.

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
// Rust's macro system allows for us to derive implementations of some traits without having to implement them ourselves.
// Traits fall somewhere in between Java's interfaces and Haskell's type classes.
// #[derive(Ord)] defaults to lexicographic ordering of the fields in the order in which they are written.
// See: https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html#derivable
pub struct People {
    pub name: String,
    pub age: u32,
}

impl From<(&str, u32)> for People {
    fn from((name, age): (&str, u32)) -> Self {
        Self {
            name: String::from(name),
            age,
        }
    }
}

impl Debug for People {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct GenericPeople<Order> {
    name: String,
    age: u32,
    _phantom: PhantomData<Order>,
    // I use Rust's PhantomData type to sort generically over different orderings of people, which I represent with ZSTs.
}

impl<T> Debug for GenericPeople<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

// Rust allows for "Zero Sized Types" (ZSTs), data types which take no space (https://doc.rust-lang.org/nomicon/exotic-sizes.html).
// I use this feature to encode how People should be ordered into the type itself. This use case is perhaps a bit overkill, but it
// allows me to show off some of Rust's more unique features.

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)] // Required for PartialOrd and Ord
pub struct Lexicographic;
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AgeDescending;

impl cmp::PartialOrd for GenericPeople<Lexicographic> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name) // I do not handle the equals case.
    }
}

// Rust requires correctness as much as is reasonable. To implement Ord (equivalent to the Ord type class in Haskell) you must also
// implement Eq, which requires PartialEq, and PartialOrd.
impl cmp::Ord for GenericPeople<Lexicographic> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl From<People> for GenericPeople<Lexicographic> {
    fn from(people: People) -> Self {
        Self {
            name: people.name,
            age: people.age,
            ..Default::default()
        }
    }
}

impl cmp::PartialOrd for GenericPeople<AgeDescending> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        // String and u32 both implement Ord, so I just use that instead of the partial comparators.
        Some(match self.age.cmp(&other.age) {
            cmp::Ordering::Less => cmp::Ordering::Greater,
            cmp::Ordering::Equal => self.name.cmp(&other.name), // Compare lexicographically if they have the same age.
            cmp::Ordering::Greater => cmp::Ordering::Less,
        })
    }
}

impl cmp::Ord for GenericPeople<AgeDescending> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(&other).unwrap() // Our partial_cmp always returns Some(cmp::Order), so we can safely unwrap.
    }
}

impl From<People> for GenericPeople<AgeDescending> {
    fn from(people: People) -> Self {
        Self {
            name: people.name,
            age: people.age,
            ..Default::default()
        }
    }
}
