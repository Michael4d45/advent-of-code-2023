use std::cmp::Ordering;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

/**
--- Day 7: Camel Cards ---

Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship.
 (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

"Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for;
you're here to figure out why the sand stopped.

"The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon.
The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there.
 Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island
 recently stopped receiving the parts they need to fix the machines.

You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the game of Camel Cards.
Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand.
A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

    Five of a kind, where all five cards have the same label: AAAAA
    Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High card, where all cards' labels are distinct: 23456

Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect.
Start by comparing the first card in each hand. If these cards are different,
the hand with the stronger first card is considered stronger.
If the first card in each hand have the same label, however,
then move on to considering the second card in each hand.
If they differ, the hand with the higher second card wins;
otherwise, continue with the third card in each hand, then the fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands,
but 33332 is stronger because its first card is stronger.
Similarly, 77888 and 77788 are both a full house,
but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).

To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank,
 where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands
 in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.

So, the first step is to put the hands in order of strength:

    32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
    KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T),
     so KTJJT gets rank 2 and KK677 gets rank 3.
    T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.

Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each
hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

Find the rank of every hand in your set. What are the total winnings?

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
            HandType::Five(val_self) => match other {
                HandType::Five(val_other) => val_self.cmp(val_other),
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
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Bad parse"),
    }
}

fn get_hand_type(cards: Vec<u32>) -> HandType {
    dbg!(&cards);
    let mut twos_1 = 0;
    let mut twos_2 = 0;
    let mut threes = 0;
    for i in 0..cards.len() - 1 {
        if cards[i] == twos_1 || cards[i] == twos_2 || cards[i] == threes {
            dbg!("skipping", cards[i]);
            continue;
        }
        let mut count_same = 1;
        for j in i + 1..cards.len() {
            dbg!(cards[j]);
            if cards[i] == cards[j] {
                count_same += 1;
            }
        }
        dbg!(cards[i], count_same);
        if count_same == 5 {
            return HandType::Five(cards[i]);
        } else if count_same == 4 {
            return HandType::Four(cards[i]);
        } else if count_same == 3 {
            threes = cards[i];
        } else if count_same == 2 {
            if twos_1 == 0 {
                twos_1 = cards[i];
            } else {
                twos_2 = cards[i];
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
        return HandType::Three(threes);
    }
    if high_twos > 0 && low_twos > 0 {
        return HandType::TwoPair(TwoPair {
            high: high_twos,
            low: low_twos,
        });
    }
    if high_twos > 0 {
        return HandType::OnePair(high_twos);
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

    dbg!(&hand_type);

    return Hand {
        cards,
        bid,
        hand_type,
    };
}

fn part1(input: &str) -> String {
    let mut hands: Vec<Hand> = input.lines().map(get_numbers).collect();

    // Sort the hands
    hands.sort();

    println!("Sorted");
    for hand in &hands {
        dbg!(&hand.hand_type);
    }

    return hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + ((i as u32 + 1) * x.bid))
        .to_string();
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "6440".to_string());
    }
}
