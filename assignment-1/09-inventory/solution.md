# Solution notes: Inventory

## Approach

* `summary` (Borrowing): I used an immutable slice reference `&[(String, u32)]`. It just reads the length and sums the quantities using an iterator. Because it only borrows, the original inventory isn't moved or changed.

* `restock` (Taking Ownership): The function takes the `Vec`s by value, completely consuming them. I used a `HashMap<String, u32>` to merge duplicates. Since the function owns the data, I moved the `String`s directly into the map as keys without needing to `.clone()` them.

## The Core Constraint
The prompt required that `summary(&inv)` followed by `restock(inv, more)` compiles successfully. 

This works perfectly because `summary` only takes a temporary borrow. Once it finishes, the caller still completely owns `inv`. Therefore, `inv` can be safely passed by value (and permanently moved) into `restock` on the very next line.

## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
