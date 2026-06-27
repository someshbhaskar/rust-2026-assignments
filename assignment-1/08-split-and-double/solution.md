# Solution notes: Split and double

## Approach

The core of this problem is navigating Rust's strict borrowing rules. Rust's borrow checker prevents you from creating two mutable references to the same data structure at the same time. If you try to manually slice the vector (e.g., `&mut xs[..mid]` and `&mut xs[mid..]`), the compiler will reject it because it cannot mathematically prove at compile-time that those two slices don't overlap. 

To solve this, I used the standard library method `split_at_mut(mid)`. This method safely consumes the single mutable borrow of the `Vec` and returns a tuple of two guaranteed disjoint mutable slices `(&mut [i32], &mut [i32])`. 

Once the slices are successfully split, I used two simple `for` loops with `.iter_mut()` to iterate over each half. By dereferencing the values (`*val`), I doubled the integers in place. Finally, the function simply returns the tuple containing both modified slices.

## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
