use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, multi::separated_list1,
    IResult,
};
use std::fs;

#[derive(Debug)]
enum Tech {
    Deal,
    DealWithIncrement(usize),
    Cut(isize),
}

impl Tech {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((Self::deal, Self::deal_with_increment, Self::cut))(i)
    }

    fn deal(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("deal into new stack")(i)?;
        Ok((i, Self::Deal))
    }

    fn deal_with_increment(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("deal with increment ")(i)?;
        let (i, inc) = crate::helper::usize_val(i)?;
        Ok((i, Self::DealWithIncrement(inc)))
    }

    fn cut(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("cut ")(i)?;
        let (i, cut) = crate::helper::i32_val(i)?;
        Ok((i, Self::Cut(cut as isize)))
    }
}

fn parse_techs(i: &str) -> IResult<&str, Vec<Tech>> {
    separated_list1(newline, Tech::parse)(i)
}

type Deck = Vec<usize>;

pub fn run() {
    let input = fs::read_to_string("day22.txt").unwrap();
    println!("22:1 {}", run_1(&input));
    println!("22:2 {}", run_2(&input));
}

fn run_1(program: &str) -> usize {
    let deck = create_deck(10007);
    let (_, techs) = parse_techs(program).unwrap();
    let deck = run_with_deck(deck, &techs);

    let (idx, _) = deck
        .iter()
        .enumerate()
        .find(|(_, v)| **v == 2019usize)
        .unwrap();
    idx
}

fn run_2(program: &str) -> usize {
    let deck = create_deck(119315717514047);
    let (_, techs) = parse_techs(program).unwrap();
    let deck = run_with_deck(deck, &techs);

    let (idx, _) = deck
        .iter()
        .enumerate()
        .find(|(_, v)| **v == 2020usize)
        .unwrap();
    idx
}

fn create_deck(num_cards: usize) -> Deck {
    (0..num_cards).into_iter().collect()
}

fn cut(deck: &Deck, new_deck: &mut Deck, pos: isize) {
    let pos = if pos >= 0 {
        pos as usize
    } else {
        deck.len() - pos.abs() as usize
    };

    let (a, b) = deck.split_at(pos);
    new_deck.clear();
    new_deck.extend(b.iter());
    new_deck.extend(a.iter());
}

fn deal_with_increment(deck: &Deck, new_deck: &mut Deck, inc: usize) {
    let mut pos = 0;
    for s in deck.iter() {
        new_deck[pos] = *s;
        pos = (pos + inc) % deck.len();
    }
}

fn deal(deck: &Deck, new_deck: &mut Deck) {
    new_deck.clear();
    new_deck.extend(deck.iter().rev());
}

fn run_with_deck(mut deck_a: Deck, techs: &[Tech]) -> Deck {
    let mut deck_b = deck_a.clone();
    let mut deck_idx = 0;

    for t in techs {
        let (deck, new_deck) = if deck_idx == 0 {
            (&deck_a, &mut deck_b)
        } else {
            (&deck_b, &mut deck_a)
        };

        match t {
            Tech::Deal => deal(deck, new_deck),
            Tech::DealWithIncrement(inc) => deal_with_increment(deck, new_deck, *inc),
            Tech::Cut(pos) => cut(deck, new_deck, *pos),
        }

        deck_idx = (deck_idx + 1) % 2;
    }
    if deck_idx == 0 {
        deck_a
    } else {
        deck_b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc22_test_cut() {
        let deck = create_deck(10);
        let mut new_deck = deck.clone();
        cut(&deck, &mut new_deck, 3);
        assert_eq!(&new_deck, &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        cut(&deck, &mut new_deck, -4);
        assert_eq!(&new_deck, &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn aoc22_test_deal() {
        let deck = create_deck(10);
        let mut new_deck = deck.clone();

        deal(&deck, &mut new_deck);
        assert_eq!(&new_deck, &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

        deal_with_increment(&deck, &mut new_deck, 3);
        assert_eq!(&new_deck, &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn aoc22_run_1() {
        let (_, techs) = parse_techs(
            "deal with increment 7
deal into new stack
deal into new stack",
        )
        .unwrap();

        let deck = create_deck(10);
        let deck = run_with_deck(deck, &techs);
        assert_eq!(deck, &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        let (_, techs) = parse_techs(
            "cut 6
deal with increment 7
deal into new stack",
        )
        .unwrap();
        let deck = create_deck(10);
        let deck = run_with_deck(deck, &techs);
        assert_eq!(deck, &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let (_, techs) = parse_techs(
            "deal with increment 7
deal with increment 9
cut -2",
        )
        .unwrap();
        let deck = create_deck(10);
        let deck = run_with_deck(deck, &techs);
        assert_eq!(deck, &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let (_, techs) = parse_techs(
            "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1",
        )
        .unwrap();
        let deck = create_deck(10);
        let deck = run_with_deck(deck, &techs);
        assert_eq!(deck, &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
