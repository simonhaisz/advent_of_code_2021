use std::collections::HashMap;
use std::cmp;

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

pub struct PolymerCounts<'input> {
    pair_counts: HashMap<&'input str, u64>,
    element_counts: HashMap<char, u64>,
}

impl<'input> PolymerCounts<'input> {
    pub fn from(input: &'input str) -> PolymerCounts<'input> {
        let mut pair_counts = HashMap::new();
        let mut element_counts = HashMap::new();

        for i in 0..input.chars().count() - 1 {
            let pair = &input[i..i+2];
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

    pub fn apply(mut self, rules: &Vec<PairInsertionRule>) -> PolymerCounts<'input> {
        let mut new_pair_counts = HashMap::new();
        let mut new_element_counts = self.element_counts.clone();

        for rule in rules.iter() {
            if let Some(count) = self.pair_counts.get_mut(rule.pair.as_str()) {
                for insert in rule.split() {
                    new_pair_counts.insert(insert.as_str(), *count);
                }
                let count = new_element_counts.entry(rule.element).or_insert(0);
                *count + 1;
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

    pub fn count(&self, pair: &str) -> Option<u64> {
        match self.pair_counts.get(&pair) {
            Some(v) => Some(*v),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_from_polymer() {
        let counts = PolymerCounts::from("NNCB");
        assert_eq!(1, counts.count("NN"));
        assert_eq!(1, counts.count("NC"));
        assert_eq!(1, counts.count("CB"));
    }

    #[test]
    fn test_demo() {
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

        let polymer_counts = PolymerCounts::from("NNCB");

        assert_eq!(2188189693529, polymer_counts.score());
    }
}