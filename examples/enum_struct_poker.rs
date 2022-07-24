#[derive(Debug)]
enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}
#[derive(Debug)]
struct PokerCard {
  suit: PokerSuit,
  value: u8,
}

fn main () {
  let card1 = PokerCard {
    suit: PokerSuit::Clubs,
    value: 1,
  };
  let card2 = PokerCard {
    suit: PokerSuit::Diamonds,
    value: 13,
  };
  println!("{:?}, {:?}", card1, card2);
}