#!/bin/bash

# Maybe I'll get clever with this at some point. For now just copy the necessaries 
# into release and do the sync with the other project manually
cp ../Rocket.toml .
cp ../target/release/cardapi .
cp ../db.sqlite .