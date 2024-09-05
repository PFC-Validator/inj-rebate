#!/usr/bin/env bash
files=$(ls data/*.json)

for i in $files
do
	md5=$(md5sum $i)
	target/release/validator-competition $md5 
done
