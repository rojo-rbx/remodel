#!/bin/sh

set -e

cargo build "$@"

if [ -e temp ]; then
	rm -r temp
fi

mkdir temp

for script in test-scripts/*.lua; do
	test_name=$(basename "$script" .lua)
	output_file="test-scripts/$test_name.expected"

	echo "Running $test_name"
	output=$(./target/debug/remodel run "$script" arg1 arg2 arg3 | dos2unix)

	if [ -f $output_file ]; then
		expected_output=$(cat "$output_file" | dos2unix)

		if [ "$output" = "$expected_output" ]; then
			echo "Output was correct."
		else
			echo "Test failed!"
			echo "Expected output:"
			echo "$expected_output"
			echo "Actual output:"
			echo "$output"
			exit 2
		fi
	fi
done

if [ ! -z "${REMODEL_AUTH_TESTS}" ]; then
	echo ""
	echo "Running extra tests that need network access"
	echo ""

	for script in test-scripts-extra/*.lua; do
		echo "Running $(basename "$script" .lua)"
		./target/debug/remodel run "$script"
	done
fi