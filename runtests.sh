#!/bin/sh

set -e

if [ "$1" == "--clean" ]; then
    rm -f test/*.out
    exit 0
fi

for i in test/*.bf; do
    echo "test: $i"
    ./beef $i > $i.out
    diff -u $i.out ${i}_expected
done
