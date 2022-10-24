tree:
  cargo run --release --package make-tree

day DAY:
  cargo run --release --package day-{{DAY}} -- input.txt

test DAY:
  cargo test --package day-{{DAY}}

test-all:
  cargo test

clean:
  cargo clean
