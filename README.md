# INJ Rebate

There are two parts.
1. [get_delegators](./get_delegators.sh) 
2. validator-rebate.

## get_delegators
You need to run this once just before the rebate starts (as a prior) and periodically during the rebate. (possibly once a day/week)

pick your own validator/API server, as some limit the responses
pass in your API endpoint, and your validator address.
as setup, it will only download the first 8000 delegators, if you have more.. adjust it.

## validator-rebate

1. build it. (if you need, I can supply binaries)
2. run it.

ala
``
cargo run -- whitelist.csv prior.json out results.csv
``
the 1st argument are the whitelisted wallets. (the ones with the coupons, one per line)
the 2nd argument is the list of delegations *prior* to the rebate program. (as the rebate only applies to new delegations)
the 3rd argument is the directory that contains the results of the script, that you periodically ran.
the 4th argument is the results file the program will write.

it will be in the format of:

delegate wallet, #tokens delegated (in aINJ), and %ge of reward pool they should be alloted. 
