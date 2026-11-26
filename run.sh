#!/usr/bin/env bash
echo "========================================================"
echo " Starting E-Notes Application (Rust Web Server)"
echo " Open browser at: http://localhost:8080"
echo "========================================================"
if [ -f "./E-Notes" ]; then
    ./E-Notes
elif [ -f "./target/release/enotes" ]; then
    ./target/release/enotes
else
    cargo run --release
fi
