# Fireball Island game

This is a dummy project for learning _Rust_ and videogame concepts in the spare 
time. Was no intended to be commercialized or anything else.  
The project is based on the game [Fireball island](https://en.wikipedia.org/wiki/Fireball_Island),
published in the 1986.  
There is a new version of the game, published in the 2018 but for now we are 
focusing on the original version, due to its simplicity. Maybe in the future, we
can considerate to implements even the new version.  

### Reference & tutorial
[Sokoban game in rust](https://sokoban.iolivia.me/)  
[Roguelike tutorial](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html)  
[Hands-on rust](https://pragprog.com/titles/hwrust/hands-on-rust/)

## Setup
For running this project, you need to:
1. install the [rust toolchain](https://www.rust-lang.org/tools/install)
2. clone the repository (`git clone`)
3. run test (when will be available)
4. go to the `fireball_island` folder and run 
`cargo run fireball_island -- bin fireball_island` 

## Project Structure (1986 version)
### Game elements
Basically, the goal of this game is to reach the jewel situated at the **center**
of the island and bring it to the **port**. For doing that, the player can use
the environment and the cards he acquires during the game to his advantage.  
Scope of the game, is to take the jewel (situated at the center of the island)
and bring it to the Dock. The first who reach the _dock_ with the jewel, wins.
Is possible, for the other players, to steal the jewel simply by passing over 
the player who own it.

#### Turn
In each turn, the player can do:
1. Roll a die and move according to the result (or play a card _move ahead_).
2. Apply the effect of the landing space
3. Play any cards he want

The first player who takes the jewel, gain the following effect:
1. Draw how many card to reach the maximum allowed (4)
2. [Roll any on fireball](#fireball)
3. Take three turns

#### Cards
There is a deck of 48 cards, all with its own name, effect and activate condition. 
Are summarized in the table below.

| Card name | Quantity | Condition | Effect |
| --------- |:--------:| --------- | ------ |
| Fireball  | 4x | In any turn | Launch a fireball (from any location) |
| Fake jewel| 2x | In the opponent turn<br>If you have the jewel | An opponent don't stole the jewel when pass you | 
| Counter   | 4x | In any turn |Cancel any card<br>except fireball card |
| Magic talisman | 4x | When a fireball is about to roll | Magic talisman stops a fireball |
| Re-roll | 4x | In any turn after the die roll | Re-roll the die or force an opponent to do it |
| Extra turn | 4x | At the end of your turn | Take another turn after this one |
| Take 1 card | 4x | In any turn<br>the target must not have the jewel and the token| Take 1 card from any opponent, at random |
| Move ahead X | 3x type | In any turn | Move ahead (4, 5 or 6 spaces)<br>instead of rolling dice.<br>Can be used to force an opponent<br>to move ahead |
| Move back X | 3x type | In the opponent turn | Move the opponent back (1,2 or 3 spaces)<br>after the roll movement|
| Double die | 4x | In any turn | Double the next die roll result 

Note that the maximum amount of cards that the user can own is **4**.

#### Rolling a dice
In each turn, the player must roll a die (or play a _Move ahead_ card) and move
according to the result.  
If the result is **1** a [fireball must be launched instead](#fireball).

#### Fireball
If the player roll an **1** or play a **fireball** card, one of the 5 fireball
must be launched. The essential thing is that the fireball must hit a player, 
even if the player that has to launch the fireball is the only available target.   
In the case a player is hit by a fireball, the following actions takes place:
1. The player is put in the _smolder pit area_, according to its original 
position  
1.1 if the player was in posses of the jewel, the jewel is placed "unguarded" in
front of the _smolder pit_ where the player is finished (also called _Rock chip
area_)
2. The player lost a turn
3. In the next turn, roll the die an start counting the spaces from the _rock 
chip_ area.