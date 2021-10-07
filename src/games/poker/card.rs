use std::fmt;

use crate::games::poker;

/// The suit of a [`PlayingCard`].
///
/// [`PlayingCard`]: crate::games::poker::card::PlayingCard
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Suit
{
   /// the `Club` card type.
   Club = 0,

   /// the `Diamond` card type.
   Diamond = 1,

   /// the `Heart` card type.
   Heart = 2,

   /// the `Spade` card type.
   Spade = 3,
}

impl Suit
{
   #[doc(hidden)]
   pub const NUM: u8 = 4;

   /// The lowest possible card suit.
   pub const LOWEST: Suit = Suit::Club;

   /// The highest possible card suit.
   pub const HIGHEST: Suit = Suit::Spade;
}

/// Represents possible card values.
#[repr(u8)]
#[non_exhaustive]
#[derive(
   Debug,
   Copy, 
   Clone, 
   Eq, PartialEq,
   Ord, PartialOrd)]
pub enum Card
{
   /// the "TWO" card.
   Deux = 2,

   /// the "THREE" card.
   Trois = 3,

   /// the "FOUR" card.
   Quatre = 4,

   /// the "FIVE" card.
   Cinq = 5,

   /// the "SIX" card.
   Six = 6,

   /// the "SEVEN" card.
   Sept = 7,

   /// the "EIGHT" card.
   Huit = 8,

   /// the "NINE" card.
   Neuf = 9,

   /// the "TEN" card.
   Dix = 10,

   /// the "JACK" card.
   Jack = 11,

   /// the "QUEEN" card.
   Queen = 12,

   /// the "KING" card.
   King = 13,

   /// the "ACE" card.
   Ace = 14,
}

impl Card
{
   /// The lowest possible card value.
   pub const LOWEST: u8 = 2;

   /// The highest possible card value.
   pub const HIGHEST: u8 = 14;
}

impl fmt::Display for Card
{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
   {
      f.write_str(format!("{}", self).as_str())
   }
}

impl From<u8> for Card
{
   fn from(value: u8) -> Card
   {
      return match value {
         2  => Card::Deux,
         3  => Card::Trois,
         4  => Card::Quatre,
         5  => Card::Cinq,
         6  => Card::Six,
         7  => Card::Sept,
         8  => Card::Huit,
         9  => Card::Neuf,
         10 => Card::Dix,
         11 => Card::Jack,
         12 => Card::Queen,
         13 => Card::King,
         14 => Card::Ace,
         _  => Card::Deux,
      };
   }
}

/// Represents a playing card.
///
/// Contains a pair of `u8` values.
#[derive(Debug, Copy, Clone)]
pub struct PlayingCard(Card, Suit);

impl PlayingCard
{
   /// Create a new `PlayingCard` value pair.
   #[inline]
   pub const fn new(card: Card, suit: Suit) -> PlayingCard
   {
      return PlayingCard(card, suit);
   }

   /// Returns the value of the card.
   #[inline]
   pub const fn card(&self) -> u8 { return self.0 as u8; }

   /// Returns the suit of the card.
   #[inline]
   pub const fn suit(&self) -> u8 { return self.1 as u8; }

   /*/// Checks whether the supplied suit and the card's suit
   /// are the same.
   #[inline]
   pub const fn is_suit(&self, suit: Suit) -> bool
   {
      return self.1 == suit;
   }*/

   /// Checks whether the supplied card's value and
   /// the current card's value are the same.
   #[inline]
   pub const fn is_value(&self, card: u8) -> bool
   {
      return self.0 as u8 == card;
   }

   pub fn to_string(&self) -> String
   {
      let mut ret: String = String::new();

      ret
   }

   /*pub fn short_string(&self) -> String
   {
      let mut ret: String = String::new();

      if self.0 < 11 {
         ret = card_to_string(self.0.into(), false);
      } else if self.0 == Card::Jack {
         ret = "J".to_string();
      } else if self.0 == Card::Queen {
         ret = "Q".to_string();
      } else if self.0 == Card::King {
         ret = "K".to_string();
      } else if self.0 == Card::Ace {
         ret = "A".to_string();
      }

      match self.1 {
         Suit::Club      => { ret += "C"; },
         Suit::Diamond   => { ret += "D"; },
         Suit::Heart     => { ret += "H"; },
         Suit::Spade     => { ret += "S"; },
         _     => {},
      }


      ret
   }*/

   pub fn long_string(&self) -> String
   {
      let mut ret: String = String::new();

      ret
   }
}

/// Converts a card to a `String`.
pub fn card_to_string(card: Card, face_name: bool) -> String
{
   if face_name {
      return match card {
         Card::Deux     => "Deux".to_string(),
         Card::Trois    => "Trois".to_string(),
         Card::Quatre   => "Quatre".to_string(),
         Card::Cinq     => "Cinq".to_string(),
         Card::Six      => "Six".to_string(),
         Card::Sept     => "Sept".to_string(),
         Card::Huit     => "Huit".to_string(),
         Card::Neuf     => "Neuf".to_string(),
         Card::Dix      => "Dix".to_string(),
         Card::Jack     => "Jack".to_string(),
         Card::Queen    => "Queen".to_string(),
         Card::King     => "King".to_string(),
         Card::Ace      => "Ace".to_string(),
      };
   } else {
      return poker::utils::auto_to_string(card);
   }
}
