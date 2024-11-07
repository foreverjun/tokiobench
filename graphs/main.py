#!/usr/bin/env python

import json
import pathlib as lpath
import matplotlib.pyplot as plt
import argparse
import itertools as it

TARGET_PATH = lpath.Path().absolute() / "target"

CRITERION_PATH = TARGET_PATH / "criterion"
PLOTS_PATH = TARGET_PATH / "plots"

NWORKERS: list[int] | None = None
NSPAWN: list[int] | None = None
NSPLIT: list[int] | None = None

def set_benches(proffile: str):
    global NWORKERS
    global NSPAWN
    global NSPLIT

    match proffile:
        case "full":
            NWORKERS = [1, 2, 4, 6, 8, 10, 12]
            NSPAWN = [100, 1000, 10000, 100000, 1000000, 10000000]
            NSPLIT = [1, 2, 4, 6, 8, 10]
        case "default":
            NWORKERS = [1, 4, 8]
            NSPAWN = [100, 10000, 1000000]
            NSPLIT = [1, 2, 4]
        case "maxvalonly":
            NWORKERS = [12]
            NSPAWN = [10000000]
            NSPLIT = [10]

def check_state():
    if NWORKERS is None or NSPAWN is None or NSPLIT is None:
        raise RuntimeError("Constants not init")

NROWS = 3
NCOLS = 2

def nwork_nspawn(*, bench: str, path: lpath.Path) -> None:
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

    plt.clf()


def nwork_nspawn_split(*, bench: str, path: lpath.Path, tsplit: str) -> None:
    plt.xlabel("nspawn")
    plt.ylabel("mean")

    legend = []
    fig, axs = plt.subplots(ncols=NCOLS, nrows=NROWS, constrained_layout=True)

    for split_ind, nsplit in enumerate(NSPLIT):
        for nwork in NWORKERS:
            data_x: list[int] = []
            data_y: list[int] = []
            for nspawn in NSPAWN:
                name = f"nspawn({nspawn})_nwork({nwork})_nsplit({nsplit}, {tsplit})"
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

    plt.clf()


NSPAWN_NWORK = ["remote_rec", "remote_rec_stall", "spawn_current", "remote_stall", "remote_stall_rec", "spawn_current", "spawn_local" ]
NSPAWN_NWORK_NSPLIT = it.product(["workload", "workload_recstall"], ["Gradient", "Eq"])

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog='graphpy',
        description='Draw cringe graphs for my diploma',
        epilog="Duty dies last")

    parser.add_argument("profile")

    args = parser.parse_args()

    set_benches(args.profile)
    check_state() # should be last for state verification

    lpath.Path(PLOTS_PATH).mkdir(mode=0o777, parents=False, exist_ok=True)

    plt.figure(figsize=(10,10))

    for bench_name in NSPAWN_NWORK:
        nwork_nspawn(bench=bench_name, path=PLOTS_PATH / bench_name)

    for (bench_name, tsplit) in NSPAWN_NWORK_NSPLIT:
        path = PLOTS_PATH / f"{bench_name}_{tsplit}"

        nwork_nspawn_split(bench=bench_name, path=path, tsplit=tsplit)

