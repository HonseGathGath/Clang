#!/bin/bash

for i in {1..99}; do
	residue=$((i % 2))
	if [ "$residue" -ne 0 ]; then
		echo "$i"
	fi
done
