#!/bin/sh

set -e

cargo build

if [ -e temp ]; then
	rm -r temp
fi

mkdir temp

for script in test-scripts/*.lua; do
	echo "Running $(basename "$script" .lua)"
	./target/debug/remodel "$script"
done

if [ ! -z "${REMODEL_AUTH_TESTS}" ]; then
	echo ""
	echo "Running extra tests that need network access"
	echo ""

	for script in test-scripts-extra/*.lua; do
		echo "Running $(basename "$script" .lua)"
		./target/debug/remodel "$script"
	done
fi