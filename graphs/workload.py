import json
import pathlib as lpath
import matplotlib.pyplot as plt
import itertools as it

import params as p

NAMES = it.product(["workload", "workload_recstall"], ["Geometric", "Uniform"])

def plot(*, bench: str, path: lpath.Path, tsplit: str, nsplits: list[int]) -> None:
    plt.xlabel("nsplit")
    plt.ylabel("mean time, ms")

    legend = []

    for nwork in p.NS_WORKERS:
        data_x: list[int] = []
        data_y: list[int] = []

        for nsplit in nsplits:
            name = f"nspawn({p.N_SPAWN})_nwork({nwork})_nsplit({nsplit}, {tsplit})"
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

        # match tsplit:
            # case "Geometric":
        try:
            plot(bench=bench_name, path=path, tsplit=tsplit, nsplits=p.NS_SPLIT_GEOMERIC)
        except Exception:
            pass


        try: 
            # case "Uniform":
                plot(bench=bench_name, path=path, tsplit=tsplit, nsplits=p.NS_SPLIT_UNIFORM)
        except Exception:
            pass 