# Turbin3 Vote

Repository to demostrate the most basic concepts of a anchor program for voting and downvoting. 

It's basically a PDA account with a numeric counter and bump (to reduce c.u. and improve perf)

## Note

As I used a number in the project name, it won't generate a valid idl and the test will fail. I should have avoided using numbers.

## Installation and build

`anchor build`

## Testing

Copy the idl file to turbin_3_vote.json and run:

`anchor test`

# License 
MIT