#!/bin/bash
jsonpath=storage/top.json
symbol=BTC
changespath=storage/changes.json
cargo run -- "$1" 365 10 "$jsonpath" "$symbol" "$changespath"
python src/plot.py "$jsonpath" storage/plot.png
python src/histogram.py "$changespath" "$symbol" storage/histogram.png
