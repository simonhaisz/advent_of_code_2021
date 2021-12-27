use std::collections::HashMap;
use std::cmp;
use std::fmt::{self, Display};

pub struct PairInsertionRule {
    pair: String,
    element: char,
}

impl PairInsertionRule {
    pub fn from(input: &str) -> PairInsertionRule {
        let mut split = input.split("->");
        let pair = split.next().unwrap().trim();
        if pair.chars().count() != 2 {
            panic!("Elements should consist of exactly two characters - found '{}' of length {}", pair, pair.chars().count());
        }
        let pair = String::from(pair);
        let element = split.next().unwrap().trim();
        if element.chars().count() != 1 {
            panic!("Elements should consist of exactly one character - found '{}' of length {}", element, element.chars().count());
        }
        let element = element.chars().next().unwrap();
        PairInsertionRule {
            pair,
            element,
        }
    }

    pub fn split(&self) -> Vec<String> {
        let mut pair_chars = self.pair.chars();
        
        let mut left_split = String::new();
        left_split.push(pair_chars.next().unwrap());
        left_split.push(self.element);

        let mut right_split = String::new();
        right_split.push(self.element);
        right_split.push(pair_chars.next().unwrap());

        vec![left_split, right_split]
    }
}

#[derive(PartialEq, Debug)]
pub struct PolymerCounts {
    pair_counts: HashMap<String, u64>,
    element_counts: HashMap<char, u64>,
}

impl PolymerCounts {
    pub fn from(input: &str) -> PolymerCounts {
        let mut pair_counts = HashMap::new();
        let mut element_counts = HashMap::new();

        for i in 0..input.chars().count() - 1 {
            let pair = String::from(&input[i..i+2]);
            let count = pair_counts.entry(pair).or_insert(0);
            *count += 1;
        }

        for e in input.chars() {
            let count = element_counts.entry(e).or_insert(0);
            *count += 1;
        }

        PolymerCounts {
            pair_counts,
            element_counts,
        }
    }

    pub fn apply(self, rules: &Vec<PairInsertionRule>) -> PolymerCounts {
        let mut new_pair_counts = HashMap::new();
        let mut new_element_counts = self.element_counts.clone();

        for (pair, count) in self.pair_counts.into_iter() {
            let mut split_pair = false;
            for rule in rules.iter() {
                if pair == rule.pair {
                    split_pair = true;
                    for insert in rule.split() {
                        let pair_count = new_pair_counts.entry(insert).or_insert(0);
                        *pair_count += count;
                    }
                    let element_count = new_element_counts.entry(rule.element).or_insert(0);
                    *element_count += count;
                }
            }
            // no matching rule found, stays as-is
            if !split_pair {
                let pair_count = new_pair_counts.entry(pair).or_insert(0);
                *pair_count += count;
            }
        }

        PolymerCounts {
            pair_counts: new_pair_counts,
            element_counts: new_element_counts,
        }
    }

    pub fn score(&self) -> u64 {
        let mut max = u64::MIN;
        let mut min = u64::MAX;
    
        for (_, count) in self.element_counts.iter() {
            max = cmp::max(max, *count);
            min = cmp::min(min, *count);
        }

        max - min 
    }

    
}

impl Display for PolymerCounts {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut pairs: Vec<&String> = self.pair_counts.keys().collect();
        pairs.sort();

        writeln!(f, "Pair Counts:")?;
        for &pair in pairs.iter() {
            writeln!(f, "\t{}={}", pair, self.pair_counts.get(pair).unwrap())?;
        }

        let mut elements: Vec<&char> = self.element_counts.keys().collect();
        elements.sort();

        writeln!(f, "Element Counts:")?;
        for &element in elements.iter() {
            writeln!(f, "\t{}={}", element, self.element_counts.get(element).unwrap())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PolymerCounts {
        pub fn count(&self, pair: String) -> Option<u64> {
            match self.pair_counts.get(&pair) {
                Some(v) => Some(*v),
                None => None
            }
        }
    }

    #[test]
    fn test_counts_from_polymer() {
        let counts = PolymerCounts::from("NNCB");
        assert_eq!(Some(1), counts.count("NN".to_string()));
        assert_eq!(Some(1), counts.count("NC".to_string()));
        assert_eq!(Some(1), counts.count("CB".to_string()));
    }
    
    #[test]
    fn test_demo_1() {
        let rules: Vec<PairInsertionRule> = "
CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| PairInsertionRule::from(s))
        .collect();

        let mut polymer_counts = PolymerCounts::from("NNCB");

        for _ in 0..10 {
            polymer_counts = polymer_counts.apply(&rules);
        }

        assert_eq!(1588, polymer_counts.score());
    }

    #[test]
    fn test_demo_2() {
        let rules: Vec<PairInsertionRule> = "
CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| PairInsertionRule::from(s))
        .collect();

        let mut polymer_counts = PolymerCounts::from("NNCB");

        for _ in 0..40 {
            polymer_counts = polymer_counts.apply(&rules);
        }

        assert_eq!(2188189693529, polymer_counts.score());
    }
}