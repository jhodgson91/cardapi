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

## Routes

### GET /api
Returns link to/reroute to a starter page explaining the api

### GET /api/game/\<id\>
id: ID of the game you are querying, new/unspecified will return a new game
Returns a new Game with a fresh shuffled deck

### GET /api/game/\<id\>/deck
id: ID of the game
Returns the current deck with all cards

### GET /api/game/\<id\>/deck/draw/?\<to\>&\<cardfilter\>
id: ID of the game
to: ID of a pile to draw to 
cardfilter: see above list of query params

Returns the deck after the changes

### GET /api/game/\<id\>/\<pile\>/
id: ID of the game
pile: Name of a pile to query

Returns the pile in it's current state

### GET /api/game/\<id\>/\<pile\>/draw/?\<to\>&\<cardfilter\>
id: ID of the game
pile: name of pile
to: Destination to draw to ( either deck or another pile )
cardfilter: see above

Returns the pile after changes

# Notes
- Filtering should probably be PUT requests and JSON based, rather than query params on the URL
- Instead of filtering what you can take, maybe just say where from ( top bottom or random ) and where to
- Filtering could be used during game setup only, e.g want a game with just clubs/diamonds, or certain piles