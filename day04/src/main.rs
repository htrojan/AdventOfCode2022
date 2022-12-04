use std::cmp::max;
use std::fmt::Display;

const INPUT: &str = include_str!("input.txt");

struct Assignment{
    start: i32,
    end: i32,
}

impl Display for Assignment{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl Assignment {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    fn parse(input: &str) -> Self {
        let mut parts = input.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        Self { start, end }
    }

    fn contains(&self, assignment: &Self) -> bool {
        self.start <= assignment.start && self.end >= assignment.end
    }
}

struct AssignmentComparison {
    assignment: Assignment,
    other: Assignment,
}

impl AssignmentComparison {
    fn new(assignment: Assignment, other: Assignment) -> Self {
        Self { assignment, other }
    }

    fn parse(input: &str) -> Self {
        let mut parts = input.split(",");
        let assignment = Assignment::parse(parts.next().unwrap());
        let other = Assignment::parse(parts.next().unwrap());
        Self { assignment, other }
    }

    fn is_overlapping(&self) -> bool {
        self.assignment.contains(&self.other) || self.other.contains(&self.assignment)
    }

    fn overlapping_segments(&self) -> u32 {
        let latest_start = self.assignment.start.max(self.other.start);
        let earliest_end = self.assignment.end.min(self.other.end);
        let overlapping = earliest_end - latest_start + 1;
        let overlapping = max(overlapping, 0) as u32;
        println!("{} overlaps with {} by {}", overlapping, self.assignment, self.other);
        overlapping
    }
}

fn main() {
    let overlap_count = INPUT.lines()
        .map(AssignmentComparison::parse)
        .filter(AssignmentComparison::is_overlapping)
        .count();
    println!("Overlapping assignments: {}", overlap_count);

    let total_overlap = INPUT.lines()
        .map(AssignmentComparison::parse)
        .map(|a| a.overlapping_segments())
        .filter(|o| *o > 0).count();
    println!("Somewhere overlapping assignments: {}", total_overlap);
}