import json
import os
import sys

import pandas as pd
import seaborn as sns


def main():
    markets = None
    with open(sys.argv[1]) as file:
        row = json.load(file)
        markets = pd.DataFrame(
            row,
            columns=[
                "mean_standard_deviation_ratio",
                "measure_range",
                "start_date",
                "symbol",
            ],
        )
    frame = pd.DataFrame()
    for index, row in markets.iterrows():
        with open(f"storage{os.path.sep}{row['symbol'].lower()}changes.json") as file:
            data = json.load(file)
            frame = pd.concat(
                [frame, pd.DataFrame(data, columns=[row["symbol"]])], axis=1
            )
    frame = frame.reindex(sorted(frame.columns), axis=1)
    sns.set_theme(
        context="poster",
        palette="colorblind",
        rc={
            "axes.spines.right": False,
            "axes.spines.top": False,
            "figure.figsize": (19.2, 10.8),
        },
    )
    ax = sns.heatmap(
        frame.corr(), vmin=-1, cmap=sns.color_palette("mako", as_cmap=True)
    )
    ax.set(
        title="Daily Change Correlation",
    )
    ax.get_figure().savefig(sys.argv[2])


if __name__ == "__main__":
    main()
