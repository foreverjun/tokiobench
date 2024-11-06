#!/usr/bin/env python

import json
import pathlib as lpath
import matplotlib.pyplot as plt

TARGET_PATH = lpath.Path().absolute() / "target"

CRITERION_PATH = TARGET_PATH / "criterion"
PLOTS_PATH = TARGET_PATH / "plots"

NWORKERS = [1, 2, 4, 6, 8, 10, 12]
NSPAWN = [100, 1000, 10000, 100000, 1000000, 10000000]
NSPLIT = [1, 2, 4, 6, 8, 10, 12]

def nspawn_nwork(*, bench: str, path: lpath.Path, show: bool = True) -> None:
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


NSPAWN_NWORK = ["remote_rec", "remote_rec_stall", "spawn_current", "remote_stall", "remote_stall_rec", "spawn_current", "spawn_local" ]
NSPAWN_NWORK_NSPLIT = ["workload"]

if __name__ == "__main__":
    lpath.Path(PLOTS_PATH).mkdir(mode=0o777, parents=False, exist_ok=True)

    plt.figure(figsize=(10,10))
    bench_name = "spawn_current"

    # for bench_name in NSPAWN_NWORK:
    nspawn_nwork(bench=bench_name, path=PLOTS_PATH / bench_name, show=False)

