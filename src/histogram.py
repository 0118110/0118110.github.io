import json
import sys

import pandas as pd
import seaborn as sns


def main():
    with open(sys.argv[1]) as file:
        data = json.load(file)
        frame = pd.DataFrame(
            data,
            columns=["change"],
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
        ax = sns.histplot(frame, x="change",stat="probability")
        ax.set(
            title=f"{sys.argv[2]} Daily Change Probability",
            xlabel="Daily Change",
        )
        ax.get_figure().savefig(sys.argv[3])


if __name__ == "__main__":
    main()
