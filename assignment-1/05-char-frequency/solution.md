# Solution notes: Character frequency, sorted

## Approach

I broke this problem down into three simple steps: Counting, Converting, and Sorting.

1. Counting the characters:
I used a HashMap to act like a tally sheet. I looped through every letter in the input string. By using Rust's `entry().or_insert(0)` trick, I told the program: "Look for this letter on the tally sheet (Hashmap). If it isn't there yet, write it down and start it at 0." Then, I just added 1 to its total count.

2. Moving to a List (Vector):
Because a HashMap stores data in a completely random order, I couldn't sort it directly. I used `.into_iter().collect()` to scoop up all my (letter, count) pairs and move them into a standard Vec (Vector). Now that they were in a normal list, I could sort them.

3. Sorting with two rules:
To sort the list, I used `.sort_by().` This part was a little tricky because the problem asked for two specific rules:

Rule 1 (Highest count first): I needed the biggest numbers at the top. I did this by comparing the counts backwards (b.1.cmp(&a.1)). This forces Rust to sort them in descending order.

Rule 2 (The Tie-breaker): If two letters appeared the exact same number of times, I used .then_with() as a backup plan. Inside that, I compared the letters normally (a.0.cmp(&b.0)) so they would naturally sort in standard alphabetical order (A to Z).


## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
