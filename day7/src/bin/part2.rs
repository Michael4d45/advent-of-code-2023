use std::cmp::Ordering;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

/**
 * --- Part Two ---

To make things a little more interesting, the Elf introduces one additional rule.
 Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even than 2.
The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of determining hand type;
for example, QJJQ2 is now considered four of a kind. However,
for the purpose of breaking ties between two hands of the same type,
J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

    32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
    KK677 is now the only two pair, making it the second-weakest hand.
    T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.

With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?

 */
#[derive(Debug)]
struct FullHouse {
    threes: u32,
    twos: u32,
}

#[derive(Debug)]
struct TwoPair {
    high: u32,
    low: u32,
}

#[derive(Debug)]
enum HandType {
    Five(u32),
    Four(u32),
    FullHouse(FullHouse),
    Three(u32),
    TwoPair(TwoPair),
    OnePair(u32),
    High(u32),
}

struct Hand {
    cards: Vec<u32>,
    bid: u32,
    hand_type: HandType,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare cards first
        // let type_cmp = self.cards.cmp(&other.cards);
        match self {
            HandType::Five(_) => match other {
                HandType::Five(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            HandType::Four(_) => match other {
                HandType::Five(_) => Ordering::Less,
                HandType::Four(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            HandType::FullHouse(_) => match other {
                HandType::Five(_) => Ordering::Less,
                HandType::Four(_) => Ordering::Less,
                HandType::FullHouse(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            HandType::Three(_) => match other {
                HandType::Three(_) => Ordering::Equal,
                HandType::TwoPair(_) => Ordering::Greater,
                HandType::OnePair(_) => Ordering::Greater,
                HandType::High(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            HandType::TwoPair(_) => match other {
                HandType::TwoPair(_) => Ordering::Equal,
                HandType::OnePair(_) => Ordering::Greater,
                HandType::High(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            HandType::OnePair(_) => match other {
                HandType::OnePair(_) => Ordering::Equal,
                HandType::High(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            HandType::High(_) => match other {
                HandType::High(_) => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for HandType {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare cards first
        let cards_cmp = self.hand_type.cmp(&other.hand_type);

        // If cards are equal, compare bids
        if cards_cmp == Ordering::Equal {
            self.card_cmp(&other)
        } else {
            cards_cmp
        }
    }
}

impl Hand {
    fn card_cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.cards.len() {
            if self.cards[i] > other.cards[i] {
                return Ordering::Greater;
            } else if self.cards[i] < other.cards[i] {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

fn card_to_num(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).expect("Failed to parse digit"),
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Bad parse"),
    }
}

fn get_hand_type(cards: Vec<u32>) -> HandType {
    // dbg!(&cards);
    let mut twos_1 = 0;
    let mut twos_2 = 0;
    let mut threes = 0;

    let without_j: Vec<u32> = cards.iter().filter(|&&x| x != 1).copied().collect();
    let j_count = (cards.len() - without_j.len()) as u32;

    if j_count == 5 {
        return HandType::Five(1);
    }

    for (i, card) in without_j.iter().copied().enumerate() {
        if card == twos_1 || card == twos_2 || card == threes {
            // dbg!("skipping", card);
            continue;
        }
        let mut count_same = 1;
        for j in i + 1..without_j.len() {
            // dbg!(without_j[j]);
            if card == without_j[j] {
                count_same += 1;
            }
        }
        // dbg!(card, count_same);
        if count_same == 5 - j_count {
            return HandType::Five(card);
        } else if count_same == 4 - j_count {
            return HandType::Four(card);
        } else if count_same == 3 {
            threes = card;
        } else if count_same == 2 {
            if twos_1 == 0 {
                twos_1 = card;
            } else {
                twos_2 = card;
            }
        }
    }
    let high_twos = if twos_1 > twos_2 { twos_1 } else { twos_2 };
    let low_twos = if twos_1 < twos_2 { twos_1 } else { twos_2 };

    if threes > 0 && high_twos > 0 {
        return HandType::FullHouse(FullHouse {
            threes,
            twos: high_twos,
        });
    }
    if threes > 0 {
        if j_count > 0 {
            return HandType::FullHouse(FullHouse {
                threes,
                twos: 1, // don't know
            });
        }
        return HandType::Three(threes);
    }
    if high_twos > 0 && low_twos > 0 {
        if j_count == 1 {
            return HandType::FullHouse(FullHouse {
                threes: high_twos,
                twos: low_twos,
            });
        }
        return HandType::TwoPair(TwoPair {
            high: high_twos,
            low: low_twos,
        });
    }
    if high_twos > 0 {
        if j_count == 1 {
            return HandType::Three(high_twos);
        }
        return HandType::OnePair(high_twos);
    }
    if j_count == 2 {
        return HandType::Three(1);
    } else if j_count == 1 {
        return HandType::OnePair(1);
    }

    return HandType::High(
        cards
            .iter()
            .fold(0, |acc, x| if x > &acc { *x } else { acc }),
    );
}

fn get_numbers(line: &str) -> Hand {
    let mut temp_string = "".to_string();

    let mut cards = vec![];
    let mut bid = 0;

    let mut insert_hand = true;

    for (i, c) in line.chars().enumerate() {
        if c == ' ' {
            insert_hand = false;
            continue;
        } else if insert_hand {
            cards.push(card_to_num(c));
            continue;
        }
        temp_string.push(c);
        if i == line.len() - 1 && !temp_string.is_empty() {
            bid = temp_string.parse::<u32>().expect("Bid parsed poorly.");
        }
    }

    let hand_type = get_hand_type(cards.clone());

    // dbg!(&hand_type);

    return Hand {
        cards,
        bid,
        hand_type,
    };
}

fn part2(input: &str) -> String {
    let mut hands: Vec<Hand> = input.lines().map(get_numbers).collect();

    // Sort the hands
    hands.sort();

    return hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + ((i as u32 + 1) * x.bid))
        .to_string();
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "5905".to_string());
    }
}
