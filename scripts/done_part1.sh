#!/bin/bash
YEAR=2018
DAY=$1
DAY_WITH_ZERO=$(printf "%02d" $DAY)
# comment dayXX in main.rs
sed -i 's/mod day'${DAY_WITH_ZERO}'_part1;/\/\/ mod day'${DAY_WITH_ZERO}'_part1;/' ./src/main.rs
sed -i 's/day'${DAY_WITH_ZERO}'_part1::main()/\/\/ day'${DAY_WITH_ZERO}'_part1::main()/' ./src/main.rs
