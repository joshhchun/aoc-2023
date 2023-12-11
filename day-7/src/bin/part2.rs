use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid:   u64
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_strength  = camel_hand_strength(self);
        let other_hand_strength = camel_hand_strength(other);
        
        // If cards are equal in strength then use char labels
        if self_hand_strength == other_hand_strength {
            let self_labels  = self.cards.chars().map(|l| label_strength(l)).collect::<Vec<usize>>();
            let other_labels = other.cards.chars().map(|l| label_strength(l)).collect::<Vec<usize>>();
            let order = self_labels
                .iter()
                .zip(other_labels.iter())
                .find_map(|(a, b)| {
                    match a.cmp(&b) {
                        std::cmp::Ordering::Equal => None,
                        order => Some(order)
                    }
                })
                .unwrap();
            return order;
        }
        return other_hand_strength.cmp(&self_hand_strength)
    }
}
 impl PartialOrd for Hand {
     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
     }
 }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn label_strength(label: char) -> usize {
    // Assign numerical values to labels based on their strength
    match label {
        '2'..='9' => label as usize - '0' as usize,
        'J' => 1,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card label"),
    }
}

fn camel_hand_strength(hand: &Hand) -> HandType {
    // Extract labels from the hand
    let labels: Vec<char> = hand.cards.chars().collect();

    // Count occurrences of each label
    let mut label_counts: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    for &label in &labels {
        *label_counts.entry(label).or_insert(0) += 1;
    }

    let non_j_highest_card = label_counts
        .iter()
        .fold(None, |mut acc, (card, count) | {
            if *card != 'J' {
                acc = std::cmp::max(acc, Some((*count, *card)));
            }
            acc
        });

    if let Some(non_j_highest_card) = non_j_highest_card {
            *label_counts.entry(non_j_highest_card.1).or_default() += *label_counts.get(&'J').unwrap_or(&0);
            label_counts.remove(&'J');
        }
    
    // Get the max matching cards
    let max_count = *label_counts.values().max().unwrap();

    // Determine hand type
    let hand_type = match max_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if label_counts.values().any(|&count| count == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if label_counts.values().filter(|&&count| count == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard
    };
 
    hand_type
}

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("should read file");

    let mut hands: Vec<Hand> = content
        .lines()
        .map(|line| {
            let mut eles = line.split_whitespace();
            let (hand, bid) = (eles.next(), eles.next().unwrap().parse());
            Hand {cards: String::from(hand.unwrap()), bid: bid.unwrap()}
        })
        .collect();

    hands.sort();
    let res: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            (i as u64 + 1) * hand.bid
        })
        .sum();

    println!("{:?}", res);
}
