# Shut the Box

Shut the box is a game where you roll 2d6 and flip down numbers depending on the
sum of the dice. You can flip down any combination of numbers that add up to the
sum. For example, if a 3 and 4 are rolled (summing to 7), you can flip down 1&6,
2&5, 3&4, or 7. The game is over when either all numbers have been flipped down
or no more numbers can be flipped down. Your score for the game is the sum of
all of the numbers you were able to flip down.

This program implements that game. Simply run `cargo run` to play.

The results of the game are stored in a file called `game_log.csv` where each
entry is a round of the game. The final score of a given game is the sum of all
the digits in the last entry of that row.
