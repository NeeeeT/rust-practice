#[derive(Debug)]
enum PokerCard {
  Clubs(u8),
  Spades(u8),
  Diamonds(u8),
  Hearts(u8),
}

fn main() {
  let c1 = PokerCard::Clubs(3);
  let c2 = PokerCard::Spades(4);
  println!("{:?} {:?}", c1, c2);
}