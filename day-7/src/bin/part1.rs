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
            return self_labels.cmp(&other_labels)
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
    let mut label_counts: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    for &label in &labels {
        *label_counts.entry(label).or_insert(0) += 1;
    }
    let (_, max_count) = label_counts.iter().max_by_key(|&(_, count)| count).unwrap();
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
    let content = fs::read_to_string("test_input.txt")
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
    println!("{:?}", hands);
    let res = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| {
            println!("hand: {:?}, rank: {:?}, bid: {:?}", hand.cards, (i as u64 + 1), hand.bid);
            acc + ((i as u64 + 1) * hand.bid) 
        });
    println!("{:?}", res);
}
