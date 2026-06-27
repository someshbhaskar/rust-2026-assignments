# Solution notes: Longest word slice

## Approach

1. Initialization: I use `sentence.split_whitespace()` to create an iterator over the string. This method is ideal because it automatically handles multiple spaces and yields string slices (&str) that borrow directly from the original sentence, avoiding any heap allocation.

2. Handling Empty Inputs: I pull the first word out of the iterator using a match statement on `words.next()`. If the string is empty or contains only whitespace, this immediately `returns None`. Otherwise, it initializes the longest tracking variable with the very first word.

3. Iterating: I use a standard `for loop` to go through the remaining words in the iterator.

4. Tie-breaking: Inside the loop, I compare the byte length of the current word against the longest word. By using the strict `greater-than operator (>)`, the code ensures that if two words have the exact same length, the first one encountered is preserved, perfectly satisfying the tie-breaking rule.

## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
