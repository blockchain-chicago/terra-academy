### Assignment
<i>
Our smart contract we started in the last exercise is worthless until we add some storage. 
<br/> <br/> 
1) Using cw_storage_plus::Map, create a key/value store with the host account as the key that contains the fields below in each value. (This will mean that the host can only have one game in progress.)
<br/> <br/> 
Addr host,<br/> 
Addr opponent,<br/> 
GameMove host_move, // enum GameMove { Rock, Paper, Scissors }<br/> 
GameMove opp_move,<br/> 
GameResult result   // enum GameResult { HostWins, OpponentWins, Tie }<br/> <br/> 
2) Then, extend your ExecuteMsg::StartGame so that the info.sender user, “host”, can use it to start a new game against an arbitrary “opponent” (you should be checking the address, if you completed the Addr exercise). The StartGame message should include the first move.
<br/> 
3) Add a test for the new StartGame action which makes sure everything works correctly. Attach screenshots of the test code and test result.
</i>
<hr/>

### Solution
Watching the 50 min code along vid helped a tiny bit. The docs here also helped: https://github.com/CosmWasm/cw-plus/tree/main/packages/storage-plus.


<br/> <br/> 
<b>Annotated code is in this dir. I used [CHANGE] by important changes. Im not sure if this is completed, but uh I turned it in.<b>
<br/> <br/> 
For seeing printouts while testing use.<br/> 
<code>cargo test -- --nocapture</code>


<br/> <br/> 
Also the deliverables (screen shots) are in this dir. I submitted a zip tho.
