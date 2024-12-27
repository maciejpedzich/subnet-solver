# subnet-solver

## Why?

I found subnetting tests for my uni's computer networking course very easy, but somewhat tedious to solve by hand.

You're essentially given a CIDR of the main subnet and a list of about 10 smaller subnets you need to divide the main one into. You're given their names (single uppercase letters) and the minimal number of hosts they need to accomodate. You have to start assigning the addresses from the largest subnet, and if two subnets have the same sizes, you have to use either alphabetical or reverse alphabetical order.

So I came up with this simple program. Run it without arguments for more details. Enjoy!
