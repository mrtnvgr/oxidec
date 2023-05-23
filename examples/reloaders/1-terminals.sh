#!/bin/sh

for term in /dev/pts/*; do
    if [[ "$term" =~ [0-9]+$ ]] then
        printf "\\033]111\\007" > $term
    fi
done
