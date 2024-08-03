#!/bin/bash
jsonpath=storage/top.json
cargo run -- "$1" 365 10 "$jsonpath"
python src/plot.py "$jsonpath" storage/plot.png
