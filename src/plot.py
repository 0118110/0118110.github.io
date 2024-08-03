import json
import sys

import pandas as pd
import seaborn as sns


def main():
    with open(sys.argv[1]) as file:
        data = json.load(file)
        frame = pd.DataFrame(
            data,
            columns=[
                "mean_standard_deviation_ratio",
                "measure_range",
                "start_date",
                "symbol",
            ],
        )
        sns.set_theme(
            context="poster",
            palette="colorblind",
            rc={
                "axes.spines.right": False,
                "axes.spines.top": False,
                "figure.figsize": (19.2, 10.8),
            },
        )
        ax = sns.barplot(frame, x="symbol", y="mean_standard_deviation_ratio")
        ax.set(
            title="Top 10 Markets By Mean Standard Deviation Ratio",
            xlabel="Symbol",
            ylabel="Mean Standard Deviation Ratio",
        )
        ax.get_figure().savefig(sys.argv[2])


if __name__ == "__main__":
    main()
