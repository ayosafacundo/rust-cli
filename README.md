# Rust CLI  

CLI tool made in Rust to do all sorts of things, this tool has stuff like:

## Games

CLI Games such as:

### Poker Game

Poker game, with these helper functions
- `make_deck(cards)`
    Initialize the deck of cards as a list of tuples, with pairs like (1, Hearts).
- `draw(cards, n)`
    Draw n cards from the top of a set of cards.
- `shuffle(cards)`
    Shuffle the cards: rearrange them to be in a random order.
- `pull_high_card(cards)`
    Return the highest number value card in the deck.
- `pull_low_card(cards)`
    Return the lowest number value card in the deck.
- `sort_cards(cards)`
    Sort the cards by value and then by suit.
- `num_spades(cards)`
    Calculate the number of Spades in a set of cards.
- `num_diamonds(cards)`
    Calculate the number of Diamonds in a set of cards.
- `num_hearts(cards)`
    Calculate the number of Hearts in a set of cards.
- `num_clubs(cards)`
    Calculate the number of Clubs in a set of cards.
- `most_common_suit(cards)`
    Return the suit that has the most cards in it within a set of cards.
- `sum_cards(cards)`
    Sum up the values of all the cards.
- `has_straight(cards)`
    Determine if a set of cards has a straight, or 5 cards that have consecutive values (like 3,4,5,6,7).
- `has_pair(cards)`
    Determine if a set of cards has a matching pair (like 2 of Hearts and 2 of Diamonds).
- `has_two_pair(cards)`
    Determine if a set of cards has a two pair (like 2 of Hearts, 2 of Diamonds, 3 of Spades, 3 of Clubs).
- `has_triple(cards)`
    Determine if a set of cards has a matching triple (like 2 of Hearts, 2 of Spades, and 2 of Diamonds).
- `has_quadruple(cards)`
    Determine if a set of cards has 4 in a row (like 2 of Hearts, 2 of Spades, 2 of Clubs, 2 of Diamonds).
- `has_flush(cards)`
    Determine if a set of cards has a flush (5 cards of the same suit).
- `has_straight_flush(cards)`
    Determine if a set of cards has a straight flush (5 consecutive cards of the same suit).
- `has_full_house(cards)`
    Determine if a set of cards has a full house (one pair and one triple).
- `red_count(cards)`
    Count the number of red cards (Hearts or Diamonds) in a set of cards.
- `black_count(cards)`
    Count the number of black cards (Spades or Clubs) in a set of cards.
- `red_percentage(cards)`
    Determine what percentage of a set of cards is red (Hearts or Diamonds).
- `black_percentage(cards)`
    Determine what percentage of a set of cards is black (Hearts or Diamonds).
- `probability_next_black(cards)`
    Determine the probability that the next card pulled from a card is black (Hearts or Diamonds).
- `probability_next_red(cards)`
    Determine the probability that the next card pulled from a card is red (Hearts or Diamonds).
- `num_even(cards)`
    Count the number of even-valued cards in a set of cards.
- `num_odd(cards)`
    Count the number of odd-valued cards in a set of cards



### Prison's Dillema
A game where you fight the PC to get the most points, for this you have to either `split` or `steal` the points. If both split, both you and the PC gets **3** points. If one splits and one steals, the one stealing gets **5** points, and if both steal, each receives 1 point.

## ACSL Challenges

American Computer Science League challenges

### "String Stats"

Given a sentence of up to 1024 characters long, output the following:

- Number of different letters.
- Number of vowels.
- Number of Uppercase letters.
- Number of times that the most frequent letter appears without distinction of uppercase or lowercase.
- longest word in the sentence, if there's a tie print the first one sorted alphabetically without regard for uppercase or lowercase.

### "Strings"

Write Code Algorithms that replicate the ACSL string functions listed below:

- `char_split(str, n, char)`
`char_split` divides a given string into substrings of length `n` separated by the given `char`.
- `strrem(stra, strb)`
Removes all occurrences of the string `strb`.
- `strchr(stra, strb)`
Returns the characters of `stra` before the first occurrence of the `strb`.
- `strtok(str, char)`
Splits the given string `stra` into substrings each time the character `char` is found, but doesn't remove it from the string.
- `wordwrap(stra, n, char)`
Returns all substrings of `str` that are `n` characters in length or that begin with the character `char`.
