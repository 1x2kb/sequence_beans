# sequence_beans
It occurs to me the prompt says these are beads... not beans. Whoops.

## Background
A collection of positive integers can be represented by rows of beads in non-increasing order, each row having the corresponding number of beads. The rows and columns of the resulting bead representation can then be flipped, to form its conjugate representation.

| Collection of positive integers | Bead representation | Conjugate bead representation | Conjugate collection of integers|
|---------------------------------|---------------------|-------------------------------|---------------------------------|
|{ 5, 3, 1 }                      | ooooo<br>ooo<br>o   | ooo<br>oo<br>oo<br>o<br>o     |{ 3, 2, 2, 1, 1 }                |
|{ 4, 2, 2 }                      | oooo<br>oo<br>oo    | ooo<br>ooo<br>o<br>o          |{ 3, 3, 1, 1 }                   |

## Problem
Write a program in your favorite programming language to obtain the conjugate of a collection of positive integers.

## Code Breakdown
It is typical in Rust code to split functionality into a main and lib, both of which can be found in `src/`. The main describing the executable binary and the lib exporting testable code for integration and e2e tests. Neither test type is contained in this repository however Rust conventions are still followed.

u8 (unsigned 8 bit) was selected as anything beyond 255 will be unreadable but it is more or less arbitrary. Should the count of an individual row exceed 2^8-1 an overflow will be observed giving an incorrect row count in its conjugate form.

### Tests
Unit tests for the 2 given examples and some others can be found in `src/lib.rs`. It is Rust convention to put unit tests in the same file as the code they're testing. This is because unlike many other languages tests are compiled as normal Rust code. Test code is decorated `(#[cfg(test)])` so that it is only compliled when running `cargo` in test mode.
