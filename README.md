# What is tutorial-v1?

This is the sample code of TEA Project developer's tutorial version 1

Please follow the step by step instruction from TEA Project developer document. 

Every step in the tutorial is a branch. It starts from the initial step of cloning from boilerplace, more features are built up over each steps, finally get a full featured sample TApp.

# Steps and branches.

Please switch to the following branch as described in the tutorial document.

## init

This is the first step. We clone the code from our two boilerplate: 
- `sample-front-end`
- `sample-actor`
Please try to build the actor and front end. Load into dev-runner. Click the "say-hello" button to receive the backend actor response. 

## login, query account balance and test token faucet
Features in this step:
- Login using Metamask
- Query account balance after login
- Click to fill some test token using faucet in local dev mode.

## learn SQL CRUD by develop a decentralized **fiverr**
Features in this step:
- User A can create a task. Task is stored in SQL. 
- Task has a price tag. The creator will need to pay such price into deposit
- Worker can list all existing tasks
- Worker can claim any task. By taking the task, the worker promise he can complete this task and pay a deposit
- Worker later can click complete the task. Make the task status "done"
- Creator verify successfully and click "confirm", then both the creator's deposit and worker's deposit are paid to the worker
- If creator verify the task completion failed, the worker's deposit will be slashed and topup to the creator's deposit (the final successful worker will take all deposit)
- We can simplify the process by ignore many real business case. Just for demo purpose.

## (future task) upgrade the "Fiverr" to a Secured Oracle powered game
Features in this step:
- Replace the creator confirm step by a Secured Oracle. 
- When creator creating task, a URL and verification condition is created in the task.
- Secured Oracle decide the task is completed successful or failed. Then trigger payment.
- This can be easily converted into a gambling game.
