#!/bin/sh

set -e

cargo build

if [ -e temp ]; then
	rm -r temp
fi

mkdir temp

for script in test-scripts/*.lua; do
	echo "Running $(basename "$script" .txt)"
	./target/debug/remodel "$script"
done