use std::collections::HashMap;
use std::cmp;

pub struct PairInsertionRule<'input> {
    pair: &'input str,
    element: char,
}

impl<'input> PairInsertionRule<'input> {
    pub fn from(input: &'input str) -> PairInsertionRule<'input> {
        let mut split = input.split("->");
        let pair = split.next().unwrap().trim();
        let element = split.next().unwrap().trim().chars().next().unwrap();
        PairInsertionRule {
            pair,
            element,
        }
    }
}

pub fn polymerization(input_polymer: &str, insertion_rules: &Vec<PairInsertionRule>) -> String {
    let mut todo_insertions: Vec<Insertion> = vec![];
    let mut done_insertions: Vec<Insertion> = vec![];

    for rule in insertion_rules.iter() {
        let mut start = 0;
        while let Some(m) = input_polymer[start..].find(rule.pair) {
            let index = start + m + 1;
            todo_insertions.push(Insertion { index, element: rule.element });
            start = index;
        }
    }

    let mut ouput_polymer = String::from(input_polymer);

    for todo in todo_insertions.into_iter() {
        let offset = done_insertions.iter().filter(|&d| d.index <= todo.index).count();
        ouput_polymer.insert(todo.index + offset, todo.element);
        done_insertions.push(todo);
    }

    ouput_polymer
}

pub fn score_polymer(polymer: &str) -> u32 {
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

struct Insertion {
    index: usize,
    element: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_step() {
        let template = "NNCB";

        let rules = vec![
            PairInsertionRule { pair: "NN", element: 'C' },
            PairInsertionRule { pair: "NC", element: 'B' },
            PairInsertionRule { pair: "CB", element: 'H' },
        ];

        let polymer = polymerization(&template, &rules);

        assert_eq!("NCNBCHB", polymer);
    }

    #[test]
    fn test_demo() {
        let template = "NNCB";

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

        let polymer = polymerization(&template, &rules);
        assert_eq!("NCNBCHB", polymer);

        let polymer = polymerization(&polymer, &rules);
        assert_eq!("NBCCNBBBCBHCB", polymer);

        let polymer = polymerization(&polymer, &rules);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", polymer);

        let polymer = polymerization(&polymer, &rules);
        assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB", polymer);

        let polymer = polymerization(&polymer, &rules);
        let polymer = polymerization(&polymer, &rules);
        let polymer = polymerization(&polymer, &rules);
        let polymer = polymerization(&polymer, &rules);
        let polymer = polymerization(&polymer, &rules);
        let polymer = polymerization(&polymer, &rules);

        assert_eq!(1588, score_polymer(&polymer));
    }
}