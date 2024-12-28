# subnet-solver

It's a simple Rust CLI tool for solving subnetting tests for my uni's computer networking course.

## Why?

I found the subnetting test for my uni's computer networking course very easy, yet somewhat tedious to solve by hand.

You're essentially given a CIDR of the main subnet and a list of 10-15 smaller subnets you need to divide the main one into. Each subnet has a name (single uppercase letter) and a number of hosts that will be connected to it. You're supposed to assign blocks of addresses from the main pool starting with the largest subnet on the list (if multiple subnets have identical sizes, use either alphabetical or reverse alphabetical order depending on the version of the test you get).

## How does it work?

Check out [my blog post](https://maciejpedzi.ch/blog/cheesing-a-subnetting-test-with-rust) for the breakdown of the algorithm and the code itself.

## How do I use it?

You need to run this app with 3 parameters:

1. Main subnet's CIDR, eg. `12.34.56.78/9`
2. Comma-separated list of subnets with their names and numbers of hosts, eg. `"(A,12), (B,34), (C,56)"`
3. `A-Z` to order subnets with the same sizes alphabetically, or `Z-A` to use reverse alphabetical order

## License

GNU General Public License v3.0
