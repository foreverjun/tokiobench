import json
import pathlib as lpath
import matplotlib.pyplot as plt
import itertools as it

import params as p

N_ITER = 20

SUBNAME = ["local", "current"]
NAMES = [ f"spawner_{subname}_nwork({nwork})" for nwork in p.NS_WORKERS for subname in SUBNAME ]

def plot(mname, bname: str, result_path: lpath.Path) -> None:
    plt.xlabel("mean time, ms")
    plt.ylabel(mname)

    for n_iter in range(N_ITER):
        metrics_path = p.TARGET_PATH / "metrics" / bname / f"iter_{n_iter}.json"

        metrics = json.load(metrics_path.open())

        data_x = []
        data_y = []
        time = 0.0
        for metr in metrics:

            val = metr[mname]

            secs = metr["elapsed"]["secs"]
            nanos = metr["elapsed"]["nanos"]

            time += nanos / 10 ** 6 + secs * 10 ** 3

            data_x.append(time)
            data_y.append(val)

        plt.scatter(data_x, data_y)

    plt.savefig(result_path)
    plt.clf()

def run():
    for bname in NAMES:
        for mname in ["injection_queue_depth", "total_steal_count"]:
            dir = p.PLOTS_PATH / mname

            dir.mkdir(mode=0o777, parents=True, exist_ok=True)

            plot(mname, bname, p.PLOTS_PATH / mname / bname)