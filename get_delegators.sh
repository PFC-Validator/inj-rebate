#!/usr/bin/env bash
# the aim is to run this in a cronjob at a regular interval during the process.
#
api="${1:-https://rest.cosmos.directory/injective}"
_url="$api/cosmos/staking/v1beta1/validators"
validator="${2:-injvaloper15vlkdnu2c0k0gaclgycnyjm7c5f3hsde034f5p}"
date_label=$(date +"%Y-%m-%d-%H:%M")


mkdir -p out
# 8000 should be enough.. but if not, adjust accordingly
curl -s -o out/out.${date_label}.json "${_url}/${validator}/delegations?pagination.limit=8000"
