# INJ Rebate

There are two parts.
1. [get_delegators](./get_delegators.sh) 
2. validator-rebate.

## get_delegators
You need to run this once just before the rebate starts (as a prior) and periodically during the rebate. (possibly once a day/week)

pick your own validator/API server, as some limit the responses
pass in your API endpoint, and your validator address.

## validator-rebate

1. build it. (if you need, I can supply binaries)
2. run it.

ala
``
cargo run -- prior.json out results.csv
``
the first argument is the list of delegations *prior* to the rebate program. (as the rebate only applies to new delegations)
the second argument is the directory that contains the results of the script, that you periodically ran.
the third argument is the results file the program will write.

it will be in the format of:

delegate wallet, #tokens delegated (in aINJ), and %ge of reward pool they should be alloted. 
