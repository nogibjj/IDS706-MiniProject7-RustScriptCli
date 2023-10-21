#!/bin/bash

# Record the start time
start_time=$(date +%s.%N)

# Run the Rust program 1000 times
# Run time could be changed to 10000 or 100000 or other values
for i in {1..1000}; do
    target/debug/caesar-cipher-tool "Hello World" 3 > /dev/null
done

# Record the end time
end_time=$(date +%s.%N)

# Calculate the total elapsed time
elapsed_time=$(echo "$end_time - $start_time" | bc)

echo "Total time elapsed: $elapsed_time seconds"
