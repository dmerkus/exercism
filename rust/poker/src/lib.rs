/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
///
///
use regex::Regex;

enum Suit {
  Clubs,
  Diamonds,
  Hearts,
  Spades,
}

enum Category {
  Ace,
  King,
  Queen,
  Jack,
  Number(i8),
}

struct PlayingCard {
  category: Category,
  suit: Suit,
}

impl PlayingCard {
  fn new(card: &str) -> PlayingCard {
    let re = Regex::new(r"(?P<type>[2|3|4|5|6|7|8|9|10|J|Q|K|A])(?P<suit>[C|D|H|S])").unwrap();

    let groups = re.captures(card).unwrap();

    let category = groups.name("type").unwrap().as_str();
    let suit = groups.name("suit").unwrap().as_str();

    let suit = match suit {
      "C" => Suit::Clubs,
      "D" => Suit::Diamonds,
      "H" => Suit::Hearts,
      "S" => Suit::Spades,
      _ => panic!("Unexpected card suit"),
    };

    let category = match category {
      "A" => Category::Ace,
      "K" => Category::King,
      "Q" => Category::Queen,
      "J" => Category::Jack,
      "10" => Category::Number(10),
      "9" => Category::Number(9),
      "8" => Category::Number(8),
      "7" => Category::Number(7),
      "6" => Category::Number(6),
      "5" => Category::Number(5),
      "4" => Category::Number(4),
      "3" => Category::Number(3),
      "2" => Category::Number(2),
      _ => panic!("Unexpected card type"),
    };

    return PlayingCard { category, suit };
  }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
  for hand in hands.iter() {
    for card in hand.split(' ') {
      let card = PlayingCard::new(card);
    }
  }

  None
}
