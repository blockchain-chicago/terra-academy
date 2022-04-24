### Assignment
<i>
Before we dive into specific aspects of development, try getting your feet wet by adding the following functionality to the template contract, depending on your level of programming expertise.
<br/><br/> 
Level 1: Currently the only query available is the current count. Add the ability to query who the current owner is. You might want to start in the msg.rs file. When you're done, make sure the contract compiles with cargo test.
<br/><br/> 
Level 2: Add a function which allows the current owner to update the owner to a new address. When you're done, make sure the contract compiles with cargo test.
<br/><br/> 
Level 3: Add tests which ensure the new functionality works. cargo test now should successfully run not only the current tests but also your custom tests for levels 1 and 2.
<br/>
Submit your working zipped code (src folder) below.

</i>
<hr/>

### Solution
Set up the base code by following these docs
1. [Download rust and others](https://docs.terra.money/docs/develop/dapp/quick-start/initial-setup.html)
2. [Deploy hello world project on test net and make your first DApp](https://docs.terra.money/docs/develop/dapp/quick-start/using-terrain-testnet.html)
    1. if you have trouble doing npm install or npm start at the end do npm install —force (after trying what the console tells you)

<br/>
you dont have to deploy anything. once you get the code on your computer you can go ahead and start working on the tasks.
<br/> <br/> 
<b>Annotated code is in src. you only have to edit msg.rs and contract.rs for this. Every change made from the boilerplates is preceeded by a commented blurb explaining the change with the prefix "[CHANGE]" (baller tip, use ctrl+f)  <b>

<br/><br/>
For seeing printouts while testing use.<br/> 
<code>cargo test -- --nocapture</code>
