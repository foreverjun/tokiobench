#!/usr/bin/env python

import json
import pathlib as lpath
import matplotlib.pyplot as plt

TARGET_PATH = lpath.Path().absolute() / "target"

CRITERION_PATH = TARGET_PATH / "criterion"
PLOTS_PATH = TARGET_PATH / "plots"

NWORKERS = [1, 2, 4, 6, 8, 10, 12]
NSPAWN = [100, 1000, 10000, 100000, 1000000, 10000000]
NSPLIT = [1, 2, 4, 6, 8, 10]

NROWS = 3
NCOLS = 2

def nwork_nspawn(*, bench: str, path: lpath.Path, show: bool = True) -> None:
    plt.xlabel("nspawn")
    plt.ylabel("mean")

    legend = []

    for nwork in NWORKERS:
        data_x: list[int] = []
        data_y: list[int] = []

        for nspawn in NSPAWN:
            name = f"nspawn({nspawn})_nwork({nwork})"
            est_path = CRITERION_PATH / bench / name / "new" / "estimates.json"

            instance = json.load(est_path.open())

            data_x.append(nspawn)

            mean = instance["mean"]["point_estimate"]
            data_y.append(mean)

        plt.plot(data_x, data_y)

        legend.append(f"{nwork} workers")

    plt.legend(legend)
    plt.savefig(path)

    if show:
        plt.show()


def nwork_nspawn_split(*, bench: str, path: lpath.Path, show: bool = True, tsplit: str, rev: bool) -> None:
    plt.xlabel("nspawn")
    plt.ylabel("mean")

    legend = []
    fig, axs = plt.subplots(ncols=NCOLS, nrows=NROWS, constrained_layout=True)

    for split_ind, nsplit in enumerate(NSPLIT):
        for nwork in NWORKERS:
            data_x: list[int] = []
            data_y: list[int] = []
            for nspawn in NSPAWN:
                rev = "true" if rev else "false"
                name = f"nspawn({nspawn})_nwork({nwork})_nsplit({nsplit}, {tsplit})_rev(false)"
                est_path = CRITERION_PATH / bench / name / "new" / "estimates.json"

                instance = json.load(est_path.open())

                data_x.append(nspawn)

                mean = instance["mean"]["point_estimate"]
                data_y.append(mean)

            row = split_ind // NCOLS
            col = split_ind % NCOLS

            print(row)
            print(col)

            axs[row, col].plot(data_x, data_y)
            axs[row, col].set_title(f"split {nsplit}")

            legend.append(f"{nwork} workers")

    plt.legend(legend)
    plt.savefig(path)

    if show:
        plt.show()


NSPAWN_NWORK = ["remote_rec", "remote_rec_stall", "spawn_current", "remote_stall", "remote_stall_rec", "spawn_current", "spawn_local" ]
NSPAWN_NWORK_NSPLIT = ["workload"]

if __name__ == "__main__":
    lpath.Path(PLOTS_PATH).mkdir(mode=0o777, parents=False, exist_ok=True)

    plt.figure(figsize=(10,10))
    bench_name = "workload"

    # for bench_name in NSPAWN_NWORK:
    # nwork_nspawn(bench=bench_name, path=PLOTS_PATH / bench_name, show=False)

    nwork_nspawn_split(bench=bench_name, path=PLOTS_PATH / bench_name, show=False, tsplit="Gradient", rev=False)

