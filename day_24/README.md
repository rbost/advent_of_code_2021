# Day 24 of the 2021 Advent of Code

For this day, I used two approaches, the brute force and the reverse-engineering.

## Brute force
I just translated the input code in Rust (essentially using search/replace and regexp), and just implemented a loop to test all the possibilities, starting by the largest one (999999999999999) and decrementing from there.

This is very coarse for a few reasons:
1. You have to hope that you will quickly find a solution. It was the case for me for the 1 part of the problem (the solution was 99799212949967, which already took me one hour of computations).
2. It is easy to improve the computation of the function and avoid recomputing the same values over and over: if two inputs share the same prefix of length `l`, they will have the same value `z` up to taking the `l+1`-th digit of the inputs. 
Also, only the value `z` is kept between rounds: `w`, `z`, and `y` get set to 0 by the `inp` instruction (for `w`) or by the `mul 0` instruction (for the rest).
So, we can reduce the state to be passed to a single variable and iterate on the state, copying it for every next digit.
3. The search task is easily parallelizable (e.g. using the `rayon` crate)

## Reverse-engineering

The key things to notice are the following:
1. `mul a 0` sets `a` to 0, so 
    ```
    mul a 0
    add a b
    ```
    just copies `b` in `a`.

2. When `x` is the result of a comparison, `eql x 0` just flips the result.

3. Let's look at the following block:
    ```
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 6 // this value depends on your input + on the round
    mul y x
    add z y
    ```
    What is does is testing if `x` and `w` are different and set `x` to 1 if they are. It that case, the following instructions can be simplified to 
    ```
    mul y 0
    add y 26
    mul z y
    mul y 0
    add y w
    add y 6 // this value depends on your input + on the round
    add z y
    ```
    or, in pseudo code, `z = 26*z + (w+6)`.
    If `x` is set to 0, things can also be simplified a lot:
    ```
    mul y 0
    add y 25
    mul y 0
    add y 1
    mul z y
    mul y 0
    add y w
    add y 6 // this value depends on your input + on the round
    mul y 0 // set y to 0
    add z y // this does nothing to z 
    ```
    which is equivalent to 
    ```
    mul y 0
    add y 1 // set y to 1
    mul z y // this does nothing to z
    ```
    so `z` is not changed.
    We are just in a kind of constant-time `if` branch.

4. For some rounds, the `x == w` test is always false: we know that `w` is a digit, between 1 and 9, and `x` is a positive integer. So when we have
    ```
    add x 13
    eql x w
    eql x 0
    ```
    we know that x is always set to 1 because `w` is always strictly less than 10. And, we can apply the corresponding `if` branch (`z = 26*z + (w+6)`).

5. It happens that the remaining cases where the check cannot be simplified are always preceded by the `div z 26` instruction, while when the ones where it can be are always preceded by the `div z 1` instruction (i.e. do nothing).

6. It is crucial to note that the
    ```
    mul x 0
    add x z
    mod x 26
    ```
    block sets `x` to `z mod 26` which is exactly the last value `y` added to `z`.
    So, when we also execute the instruction `z div 26` it puts `z` in the same state as before multiplying by 26 and adding `y`.
    In some sense, `z` behaves as a stack: multiplying by 26 and adding `y` pushes `y` to the stack, while dividing by 26 pops an element (which can be retrieved by computing `z mod 26` before popping).

7. The fact that `z` must be zero at the end means that the corresponding stack is empty. Now, notice that the rounds for which the test simplifies always correspond to pushes and the other ones pop a first element of the stack. Also note that there are as many 'pushing' rounds as there are 'popping' rounds and that, when the test of a 'popping' round is fulfilled, no new value is pushed. As a consequence, you must choose your input's digits to always satisfy the remaining conditions, which translates to relationships between digits.

8. Now it is up to you to write those relationships. A pen and a paper are of great help. And this should very quickly give you the answers by finding the largest (respectively smallest valid digits).