#!/bin/bash
YEAR=2018
DAY=$1
DAY_WITH_ZERO=$(printf "%02d" $DAY)
# copy part1 code file for part2
cp -n ./src/day${DAY_WITH_ZERO}_part1.rs ./src/day${DAY_WITH_ZERO}_part2.rs
# replace "part 1" with "part 2"
sed -i 's/- Part 1 -/- Part 2 -/' ./src/day${DAY_WITH_ZERO}_part2.rs
# uncomment dayXX_part2 in main.rs
sed -i 's/\/\/ mod day'${DAY_WITH_ZERO}'_part2;/mod day'${DAY_WITH_ZERO}'_part2;/' ./src/main.rs
sed -i 's/\/\/ day'${DAY_WITH_ZERO}'_part2::main()/day'${DAY_WITH_ZERO}'_part2::main()/' ./src/main.rs
