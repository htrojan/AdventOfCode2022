use std::collections::BTreeSet;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Rucksack {
    compartment1: Vec<u32>,
    compartment2: Vec<u32>,
}

fn char_to_score(c: char) -> u32 {
    let upper_a = u32::from('A');
    let lower_a = u32::from('a');

    let result = match c {
        'A'..='Z' => u32::from(c) - upper_a + 1 + 26,
        'a'..='z' => u32::from(c) - lower_a + 1,
        _ => 0,
    };
    result
}

impl Rucksack {
    fn parse(input: &str) -> Result<Self, &'static str> {
        let (comp1, comp2) = input.split_at(input.len() / 2);
        let compartment1: Vec<u32> = comp1.chars().map(|c| char_to_score(c)).collect();
        let compartment2: Vec<u32> = comp2.chars().map(|c| char_to_score(c)).collect();

        if compartment1.len() != compartment2.len() {
            return Err("Invalid input. Rucksack compartments have to be the same size");
        }

        Ok(Self {
            compartment1,
            compartment2,
        })
    }

    fn calc_double_score(&self) -> u32 {
        let mut score = 0;
        let mut found = BTreeSet::new();
        for item1 in &self.compartment1 {
            for item2 in &self.compartment2 {
                if *item1 == *item2 && !found.contains(item1) {
                    score += item1;
                    found.insert(*item1);
                }
            }
        }

        score
    }

    fn get_item_set(&self) -> BTreeSet<u32> {
        let mut set = BTreeSet::new();
        for item in &self.compartment1 {
            set.insert(*item);
        }
        for item in &self.compartment2 {
            set.insert(*item);
        }
        set
    }
}

fn main() {
    let rucksacks = INPUT
        .lines()
        .map(|line| Rucksack::parse(line).unwrap())
        .collect::<Vec<_>>();
    let score1: Vec<u32> = rucksacks.iter().map(|r| r.calc_double_score()).collect();
    println!("score1: {:?}", score1.iter().sum::<u32>());

    let mut lines = INPUT.lines().peekable();
    let mut score2 = 0;
    while lines.peek().is_some() {
        let backpack1 = Rucksack::parse(lines.next().unwrap()).unwrap().get_item_set();
        let backpack2 = Rucksack::parse(lines.next().unwrap()).unwrap().get_item_set();
        let backpack3 = Rucksack::parse(lines.next().unwrap()).unwrap().get_item_set();
        let common = &(&backpack1 & &backpack2) & &backpack3;
        debug_assert!(common.len() == 1);
        let item = common.into_iter().next().unwrap();
        score2 += item;
    }
    println!("score2: {}", score2);
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        assert_eq!(1, super::char_to_score('a'));
        assert_eq!(27, super::char_to_score('A'));
        assert_eq!(38, super::char_to_score('L'));
        assert_eq!(16, super::char_to_score('p'));
        assert_eq!(19, super::char_to_score('s'));
    }
}