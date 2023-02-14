# concordiumSecondTask
TASK 2: Deploy Your First Smart Contract. Concordium Hackathon - The Future Of Identity. 

Mainnet address: 3XK874y4R5QYaxHWwEoUmp9J3DqrknjBCuCD8qzD52Z5rrKvQM

Task1: https://github.com/jimytech/concordiumFirstTask

Project: hello-world-contract
Description: It is a basic smart-contract, with two string state variables, the country of origin and the greeting message, which can be updated from the functions: country and hello. This allowed me to verify all the configuration and the basic development environment of cargo-concordium, plus of course the initial syntax of the Rust programming language and the basic commands for the implementation, deployment, update, testing and visualization of the smart-contract state variables.


## deploy: 878fb4c7deadc5ba4037a13b4976ae1baedcd55fdcbdc94a51622f10a84112e3
## init: b66661bc4fb5f57e16d7ee7187758b65809ddc426647bc82b0020a382cc9244e
## update: 0bd319634425caed264cec9d3fbc337099e3c90a43d8899a0a0790107293bd47
## invoke: n/a (see screen in the end)

# Console output successfully deployed a smart contract + transaction hash in text format 

![1 1deploy](https://user-images.githubusercontent.com/39538184/218757541-fbccc459-dc22-4e50-a95e-b570c60137d2.png)

Transaction hash: 878fb4c7deadc5ba4037a13b4976ae1baedcd55fdcbdc94a51622f10a84112e3

# Console output + transaction hash of contract init in text format

![2 init](https://user-images.githubusercontent.com/39538184/218757907-4dac3c4d-af71-442d-a2d8-f27a75071583.png)

Transaction hash: b66661bc4fb5f57e16d7ee7187758b65809ddc426647bc82b0020a382cc9244e

# Console output + transaction hash of contract update in text format + parameter (if required) in JSON or binary

![3 update(country)](https://user-images.githubusercontent.com/39538184/218758334-53dd5e44-beae-48f4-89db-736e2a5eb295.png)

Transaction hash: 0bd319634425caed264cec9d3fbc337099e3c90a43d8899a0a0790107293bd47 
## Input file update-param.json
## {
##     "USA"
## }

# Console output of contract invoke (view function) + parameter (if required) in JSON or binary

![4 invoke](https://user-images.githubusercontent.com/39538184/218758521-366458a7-22b1-47d4-bb2f-4437ad929835.png)

