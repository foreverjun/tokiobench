import json
import pathlib as lpath
import matplotlib.pyplot as plt
import itertools as it

import params as p
import expwrap as ew

NAMES = it.product(["workload_local"], ["Geometric", "Uniform"])

def plot(*, bench: str, path: lpath.Path, tsplit: str, nsplits: list[int]) -> None:
    plt.xlabel("nsplit")
    plt.ylabel("mean time, ms")

    legend = []

    for nwork in p.NS_WORKERS:
        data_x: list[int] = []
        data_y: list[int] = []

        for nsplit in nsplits:
            name = f"nspawn({p.N_SPAWN_GLOBAL})_nwork({nwork})_nsplit({nsplit}, {tsplit})"
            est_path = p.CRITERION_PATH / bench / name / "new" / "estimates.json"

            instance = json.load(est_path.open())

            data_x.append(nsplit)

            mean = instance["mean"]["point_estimate"]
            data_y.append(mean)

        plt.errorbar(data_x, data_y, label=str(nsplit))

        legend.append(f"{nwork} workers")

    plt.legend(legend)
    plt.savefig(path)

    plt.clf()

def run():
    for (bench_name, tsplit) in NAMES:
        path = p.PLOTS_PATH / f"{bench_name}_{tsplit}"

        nsplit = p.NS_SPLIT_LOCAL if "local" in bench_name else p.NS_SPLIT_GLOBAL

        ew.trylog(lambda: plot(bench=bench_name, path=path, tsplit=tsplit, nsplits=nsplit))
