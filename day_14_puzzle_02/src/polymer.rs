use std::collections::HashMap;

pub struct PairInsertionRule<'input> {
    pair: &'input str,
    element: &'input str,
}

pub struct InsertionRuleIterator<'input> {
    rule: &PairInsertionRule,
    element_index: usize,
}

impl Iteratorfor InsertionRuleIterator<'input> {
    type Item = &'input str;

    fn next(&mut self) -> 
}

impl<'input> PairInsertionRule<'input> {
    pub fn from(input: &'input str) -> PairInsertionRule<'input> {
        let mut split = input.split("->");
        let pair = split.next().unwrap().trim();
        if pair.chars().count() != 2 {
            panic!("Elements should consist of exactly two characters - found '{}' of length {}", pair, pair.chars().count());
        }
        let element = split.next().unwrap().trim();
        if element.chars().count() != 1 {
            panic!("Elements should consist of exactly one character - found '{}' of length {}", element, element.chars().count());
        }
        PairInsertionRule {
            pair,
            element,
        }
    }
}

pub struct PolymerCounts<'input> {
    pair_counts: HashMap<&'input str, u64>,
    element_counts: HashMap<&'input str, u64>,
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

        for i in 0..input.chars().count() {
            let element = &input[i..i+1];
            let count = element_counts.entry(element).or_insert(0);
            *count += 1;
        }

        PolymerCounts {
            pair_counts,
            element_counts,
        }
    }

    pub fn apply(self, rules: &Vec<PairInsertionRule<'input>>) -> PolymerCounts<'input> {
        let mut pair_counts = HashMap::new();
        let mut element_counts = self.element_counts.clone();

        for rule in rules.iter() {
            if let Some(count) = pair_counts.get_mut(&rule.pair) {

            }
        }

        PolymerCounts {
            pair_counts,
            element_counts,
        }
    }
}

pub fn counts_from_polymer(input: &str) -> HashMap<&str, u64> {
    let mut polymer_counts = HashMap::new();

    for i in 0..input.len() - 1 {
        let pair = &input[i..=i+1];
        let count = polymer_counts.entry(pair).or_insert(0);
        *count += 1;
    }
    polymer_counts
}

pub fn polymerization_counts<'input>(input_polymer_counts: HashMap<&'input str,u64>, insertion_rules: &Vec<PairInsertionRule>) -> HashMap<&'input str,u64> {
    let mut polymer_counts = HashMap::new();

    polymer_counts
}

pub fn score_polymer(polymer_counts: &HashMap<&str, u64>) -> u64 {
    let mut element_counts = HashMap::new();

    for element in polymer.chars() {
        let count = element_counts.entry(element).or_insert(0);
        *count += 1;
    }

    let mut max = u32::MIN;
    let mut min = u32::MAX;

    for (_, count) in element_counts.into_iter() {
        max = cmp::max(max, count);
        min = cmp::min(min, count);
    }

    max - min
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_from_polymer() {
        let counts = counts_from_polymer("NNCB");
        assert_eq!(3, counts.len());
        assert_eq!(1, counts["NN"]);
        assert_eq!(1, counts["NC"]);
        assert_eq!(1, counts["CB"]);
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

        let mut polymer_counts = counts_from_polymer("NNCB");

        for _ in 0..40 {
            polymer_counts = polymerization_counts(polymer_counts, &rules);
        }

        assert_eq!(2188189693529, score_polymer(&polymer_counts));
    }
}