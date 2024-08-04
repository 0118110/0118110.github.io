#!/bin/bash
jsonpath=storage/top.json
symbol=BTC
cargo run -- "$1" 365 10 "$jsonpath" "$symbol"
python src/plot.py "$jsonpath" storage/plot.png
python src/heatmap.py "$jsonpath" storage/heatmap.png
python src/histogram.py storage/btcchanges.json "$symbol" storage/histogram.png
