#!/bin/bash
YEAR=2018
DAY=$1
DAY_WITH_ZERO=$(printf "%02d" $DAY)
# copy template code file for part1
cp -n ./src/dayXX_part1_template.rs ./src/day${DAY_WITH_ZERO}_part1.rs
# replace "XX" with day number
sed -i 's/XX/'${DAY_WITH_ZERO}'/g' ./src/day${DAY_WITH_ZERO}_part1.rs
# uncomment day01 in main.rs
sed -i 's/\/\/ mod day'${DAY_WITH_ZERO}'_part1;/mod day'${DAY_WITH_ZERO}'_part1;/' ./src/main.rs
sed -i 's/\/\/ day'${DAY_WITH_ZERO}'_part1::main()/day'${DAY_WITH_ZERO}'_part1::main()/' ./src/main.rs
