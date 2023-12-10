use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Type {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High,
}

const TYPES: [Type; 7] = [
    Type::Five,
    Type::Four,
    Type::FullHouse,
    Type::Three,
    Type::TwoPair,
    Type::OnePair,
    Type::High,
];
const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const CARDS_PT2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub fn run(input: String) {
    println!("Day7 Pt1: {}", pt1(&input));
    println!("Day7 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let mut winnings = 0;
    let mut hands: Vec<(&str, &str)> = vec![];
    for l in input.lines() {
        let mut split = l.split(' ');
        let hand = split.next().unwrap().trim();
        let bid = split.next().unwrap().trim();
        hands.push((hand, bid));
    }
    hands.sort_by(sort_ascending_pt1);
    for (idx, h) in hands.iter().enumerate() {
        let bid = h.1.parse::<i32>().unwrap();
        winnings += bid * (idx as i32 + 1);
    }
    winnings
}

fn sort_ascending_pt1(a: &(&str, &str), b: &(&str, &str)) -> Ordering {
    let type_a = get_type(a);
    let type_b = get_type(b);
    let rank_a = get_prio(type_a);
    let rank_b = get_prio(type_b);
    if rank_a < rank_b {
        return Ordering::Greater;
    } else if rank_a > rank_b {
        return Ordering::Less;
    } else {
        let mut chars_a = a.0.chars();
        let mut chars_b = b.0.chars();
        while let Some(char_a) = chars_a.next() {
            let char_b = chars_b.next().unwrap();
            let rank_a = get_prio_card(char_a, true);
            let rank_b = get_prio_card(char_b, true);
            if rank_a < rank_b {
                return Ordering::Greater;
            } else if rank_a > rank_b {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}

fn get_type(a: &(&str, &str)) -> Type {
    let mut cards: HashMap<char, i32> = HashMap::new();
    for c in a.0.chars() {
        cards.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    let max = cards.values().reduce(|acc, card| acc.max(card)).unwrap();
    match max {
        5 => Type::Five,
        4 => Type::Four,
        3 if cards.len() == 2 => Type::FullHouse,
        3 => Type::Three,
        2 if cards.len() == 3 => Type::TwoPair,
        2 => Type::OnePair,
        _ => Type::High,
    }
}

fn get_prio(t: Type) -> usize {
    for i in 0..TYPES.len() {
        if TYPES[i] == t {
            return i;
        }
    }
    panic!();
}

fn get_prio_card(t: char, is_pt1: bool) -> usize {
    let cards = if is_pt1 { CARDS } else { CARDS_PT2 };
    for i in 0..cards.len() {
        if cards[i] == t {
            return i;
        }
    }
    panic!();
}

fn pt2(input: &String) -> i32 {
    let mut winnings = 0;
    let mut hands: Vec<(&str, &str)> = vec![];
    for l in input.lines() {
        let mut split = l.split(' ');
        let hand = split.next().unwrap().trim();
        let bid = split.next().unwrap().trim();
        hands.push((hand, bid));
    }
    hands.sort_by(sort_ascending_pt2);
    for (idx, h) in hands.iter().enumerate() {
        let bid = h.1.parse::<i32>().unwrap();
        winnings += bid * (idx as i32 + 1);
        println!("Winnings {winnings} Bid: {bid} Rank: {}", idx + 1);
    }
    winnings
}

fn get_type_pt2(a: &(&str, &str)) -> Type {
    let mut cards: HashMap<char, i32> = HashMap::new();
    for c in a.0.chars() {
        cards.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    let joker = cards.get(&'J');
    let joker = match joker {
        Some(val) => *val,
        None => 0,
    };
    for (c, val) in cards.iter_mut() {
        if *c != 'J' {
            *val += joker;
        }
    }
    let max = cards.values().reduce(|acc, card| acc.max(card)).unwrap();
    match max {
        5 => Type::Five,
        4 => Type::Four,
        3 if joker > 0 && cards.len() == 3 => Type::FullHouse,
        3 if joker == 0 && cards.len() == 2 => Type::FullHouse,
        3 => Type::Three,
        2 if joker > 0 && cards.len() == 4 => Type::TwoPair,
        2 if joker == 0 && cards.len() == 3 => Type::TwoPair,
        2 => Type::OnePair,
        1 => Type::High,
        _ => panic!("{:?}", cards),
    }
}

fn sort_ascending_pt2(a: &(&str, &str), b: &(&str, &str)) -> Ordering {
    let type_a = get_type_pt2(a);
    let type_b = get_type_pt2(b);
    let rank_a = get_prio(type_a);
    let rank_b = get_prio(type_b);
    if rank_a < rank_b {
        return Ordering::Greater;
    } else if rank_a > rank_b {
        return Ordering::Less;
    } else {
        let mut chars_a = a.0.chars();
        let mut chars_b = b.0.chars();
        while let Some(char_a) = chars_a.next() {
            let char_b = chars_b.next().unwrap();
            let rank_a = get_prio_card(char_a, false);
            let rank_b = get_prio_card(char_b, false);
            if rank_a < rank_b {
                return Ordering::Greater;
            } else if rank_a > rank_b {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();
        assert_eq!(pt1(&input), 6440);
        assert_eq!(pt2(&input), 5905);
    }
}
