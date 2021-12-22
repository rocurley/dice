use std::cmp::Ordering;
fn main() {
    let test: Vec<_> = outcomes(4)
        .filter(|(_, roll)| roll.triple.is_some())
        .collect();
    dbg!(test);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct DiceRoll {
    hundreds: u16,
    fifties: u16,
    free: u16,
    triple: Option<u16>,
}

impl DiceRoll {
    fn score(self) -> u16 {
        self.hundreds * 100
            + self.fifties * 50
            + match self.triple {
                None => 0,
                Some(1) => 1000,
                Some(n) => 100 * n,
            }
    }
}
impl PartialOrd for DiceRoll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.score().cmp(&other.score()), self.free.cmp(&other.free)) {
            (Ordering::Greater, Ordering::Less) => None,
            (Ordering::Less, Ordering::Greater) => None,
            (a, b) => Some(a.then(b)),
        }
    }
}

/*
fn selectable_rolls(roll: DiceRoll) -> impl Iterator<Item = DiceRoll> {
    (0..=roll.hundreds).into_iter().flat_map(|hundreds| {
    }
}
*/

fn make_selection(roll: DiceRoll, goal: u16) -> Option<DiceRoll> {
    unimplemented!()
}

fn outcomes(n: u16) -> impl Iterator<Item = (u16, DiceRoll)> {
    (0..=n).flat_map(move |hundreds| {
        (0..=n - hundreds).flat_map(move |fifties| {
            let free = n - hundreds - fifties;
            let base_count =
                choose(n, hundreds) * choose(n - hundreds, fifties) * 4u16.pow(free.into());
            let inner: Box<dyn Iterator<Item = (u16, DiceRoll)>> = if hundreds >= 3 {
                Box::new(std::iter::once((
                    base_count,
                    DiceRoll {
                        hundreds: hundreds - 3,
                        fifties,
                        free,
                        triple: Some(1),
                    },
                )))
            } else if fifties >= 3 {
                Box::new(std::iter::once((
                    base_count,
                    DiceRoll {
                        hundreds,
                        fifties: fifties - 3,
                        free,
                        triple: Some(5),
                    },
                )))
            } else if free >= 3 {
                let scale = choose(n, hundreds) * choose(n - hundreds, fifties);
                let triple_count = scale
                    * (3..=free)
                        .map(|n_trip| choose(free, n_trip) * 4u16.pow((free - n_trip).into()))
                        .sum::<u16>();
                Box::new(
                    [2, 3, 4, 6]
                        .into_iter()
                        .map(move |x| {
                            (
                                triple_count,
                                DiceRoll {
                                    hundreds,
                                    fifties,
                                    free: free - 3,
                                    triple: Some(x),
                                },
                            )
                        })
                        .chain(std::iter::once((
                            base_count - 4 * triple_count,
                            DiceRoll {
                                hundreds,
                                fifties,
                                free,
                                triple: None,
                            },
                        ))),
                )
            } else {
                Box::new(std::iter::once((
                    base_count,
                    DiceRoll {
                        hundreds,
                        fifties,
                        free,
                        triple: None,
                    },
                )))
            };
            inner
        })
    })
}

fn factorial(n: u16) -> u16 {
    (1..=n).product::<u16>()
}

fn choose(n: u16, k: u16) -> u16 {
    factorial(n) / factorial(k) / factorial(n - k)
}

#[test]
fn test_sums() {
    for n in 1u16..=5 {
        assert_eq!(6u16.pow(n.into()), outcomes(n).map(|(c, _)| c).sum::<u16>());
    }
}
