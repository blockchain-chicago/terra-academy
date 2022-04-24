### Assignment
<i>
Rock, Paper, Scissors is a classic game.
<br/> <br/> 
We'll be using it to build our first real Rust smart contract.
<br/> <br/> 
Along the way, we'll learn the basics of smart contracts, tests, smart contract storage, assets, multi-party randomness, and more.
<br/> <br/> 
When we're done, two players, the "host" and the "opponent," will be able to:
<br/> <br/> 
Start a new game of Rock, Paper, Scissors with a bet (held in escrow)
Make their moves secretly and only reveal them when both players have committed to their moves
Win the bet if they win the game and move up in the leaderboard
But first, one easy step to implement: validating the opponent address.
<br/> <br/> 
For this exercise:
<br/> 
1) remove the counter functionality from the template CosmWasm smart contract.
<br/> 
2) add a single ExecuteMsg named StartGame. For now, this message only takes one field in the message: opponent, an Addr.
<br/> 
3) in the StartGame message, use addr_validate (or maybe_addr from cw0) to validate the "opponent" field.
<br/> 
4) add a test to make sure execution fails if opponent is not a valid address.
</i>
<hr/>

### Solution
Follow the instructions above
<b>go to rustlings/exercises/variables and delete the I AM NOT DONE comment to get started</b>


<br/> <br/> 
<b>Annotated code is in this dir. I used [CHANGE] by important changes. <b>
<br/> <br/> 
For seeing printouts while testing use.<br/> 
<code>cargo test -- --nocapture</code>

