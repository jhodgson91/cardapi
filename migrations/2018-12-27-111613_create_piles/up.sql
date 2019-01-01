-- Your SQL goes here
CREATE TABLE piles (
    id VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    deck_id VARCHAR(12) NOT NULL,
    cards VARCHAR NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (deck_id) REFERENCES DECKS(id)
)