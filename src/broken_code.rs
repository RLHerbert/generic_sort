// #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
// #[derive(PartialEq, Eq, PartialOrd, Default, Debug)]
// pub struct Lexicographic;
// // #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
// #[derive(PartialEq, Eq, PartialOrd, Default, Debug)]
// pub struct AgeLexicographic;

// impl Ord for People<Lexicographic> {
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         self.name.cmp(&other.name)
//     }
// }

// impl Ord for People<AgeLexicographic> {
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         match self.age.cmp(&other.age) {
//             cmp::Ordering::Less => cmp::Ordering::Less,
//             cmp::Ordering::Equal => self.name.cmp(&other.name),
//             cmp::Ordering::Greater => cmp::Ordering::Greater,
//             // gt_or_lt => gt_or_lt,
//         }
//     }
// }

// MAIN

// let to_peep_lex = |(name, age)| -> People<Lexicographic> {
//     People::<Lexicographic> {
//         name: name,
//         age: age,
//         ..Default::default()
//     }
// };

// let mut peep_lex: Vec<People<Lexicographic>> = peep_tup
//     .clone()
//     .iter()
//     .map(|(name, age)| People::<Lexicographic> {
//         name: String::from(*name),
//         age: *age,
//         ..Default::default()
//     })
//     .collect();

// let mut peep_agelex: Vec<People<AgeLexicographic>> = peep_tup
//     .clone()
//     .iter()
//     .map(|(name, age)| People::<AgeLexicographic> {
//         name: String::from(*name),
//         age: *age,
//         ..Default::default()
//     })
//     .collect();
