# Deck of Cards API
Inspired by [deckofcardsapi](https://deckofcardsapi.com/)

## Changes
This is going to extend on the original version. Some ideas for improvements

- Built in Rust / [Rocket](https://rocket.rs/) / [Diesel](http://diesel.rs/) because Rust rules
- Create a **Game** object that includes **RuleSets**, which could include
  - Ace High?
  - Card values ( e.g pic cards worth 10 or 11/12/13? )
  - Win States?
  - Initial decks
  - Initial piles
  - Player count
  - Means we can do server-driven rule changes
- Game could contain piles, allowing for multiple decks
- Possible to query the state of piles/decks e.g HasCard(s)
- Possible to reset decks
- Better Card filtering using Card Selections
  - **?suits=H,D** would select all Hearts and diamonds
  - **?suits=H,D&values=A** would select AH and AD
  - **?cards=AH,AD** would select AH and AD
  - **?random=10** would select 10 random cards
  - **?top=10** would select top 10 cards ( default to 1 )
  - **?bottom=10** would select bottom 10 cards ( default to 1 )
- Entirely server-authoratitive.

## Simpler alternative

- Only ever one deck.
- Decks own their piles directly.
- Not possible to move cards between decks
- Not possible to draw cards and not put them anywhere, must go in a pile