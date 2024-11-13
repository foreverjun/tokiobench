import json
import pathlib as lpath
import matplotlib.pyplot as plt
import itertools as it

import params as p

NAMES = ["remote_rec", "remote_rec_stall", "remote_stall", "remote_stall_rec"]

def plot(*, bench: str, path: lpath.Path) -> None:
    plt.xlabel("spawn count")
    plt.ylabel("mean time, ms")

    legend = []

    for nwork in p.NS_WORKERS:
        data_x: list[int] = []
        data_y: list[int] = []

        for nspawn in p.NS_SPAWN:
            name = f"nspawn({nspawn})_nwork({nwork})"
            est_path = p.CRITERION_PATH / bench / name / "new" / "estimates.json"

            instance = json.load(est_path.open())

            data_x.append(nspawn)

            mean = instance["mean"]["point_estimate"]
            data_y.append(mean)

        plt.errorbar(data_x, data_y, label=str(nspawn))

        legend.append(f"{nwork} workers")

    plt.legend(legend)
    plt.savefig(path)

    plt.clf()

def run():
    for bname in NAMES:
        plot(bench=bname, path=p.PLOTS_PATH / bname)


