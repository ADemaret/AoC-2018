# "just create 1" to start day01 part1

create day:
    ./scripts/set_part1.sh {{day}}
    ./scripts/get_input.sh {{day}}

done_part1 day:
    ./scripts/done_part1.sh {{day}}

part2 day:
    ./scripts/set_part2.sh {{day}}

done_part2 day:
    ./scripts/done_part2.sh {{day}}

test:
    cargo run --release
    cargo test --release
    cargo clippy

done day:
    ./scripts/done_part1.sh {{day}}
    ./scripts/done_part2.sh {{day}}

meld day:
    meld ./src/day{{day}}_part1.rs ./src/day{{day}}_part2.rs
