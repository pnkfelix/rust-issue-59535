cd rubble
set -e
cp src/main.rs.v1 src/main.rs && cargo build

echo
echo THE BUG
echo
cp src/main.rs.v2 src/main.rs && cargo build
