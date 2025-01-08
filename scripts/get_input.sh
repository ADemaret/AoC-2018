#!/bin/bash
YEAR=2018
DAY=$1
DAY_WITH_ZERO=$(printf "%02d" $DAY)
# get input file
curl -L -b ./scripts/cookie.txt https://adventofcode.com/${YEAR}/day/${DAY}/input >  ./assets/day${DAY_WITH_ZERO}_input.txt
# create empty demo file
touch ./assets/day${DAY_WITH_ZERO}_input_demo1.txt
