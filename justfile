# Use `just work 01 1` to work on the specific binary for a specific day's problems
run day part:
    cargo run -p day-{{day}} --bin part{{part}}
work day part:
    cargo watch -w day-{{day}} -x "check -p day-{{day}}" -s "just test {{day}} {{part}}"
lint day:
    cargo clippy -p day-{{day}}
test day part:
    cargo nextest run -p day-{{day}} part{{part}} --nocapture
bench-all:
    cargo bench -q > benchmarks.txt
bench day part:
    cargo bench --bench day-{{day}} part{{part}} >> day-{{day}}.bench.txt
flamegraph day part:
    cargo flamegraph --profile flamegraph --root --package day-{{day}} --bin part{{part}} -o flamegraphs/day-{{day}}--part{{part}}.svg
dhat day part:
    cargo run --profile dhat --features dhat-heap --package day-{{day}} --bin part{{part}}
create day:
    cargo generate --path ./daily-template --name day-{{day}} -d day={{day}}
