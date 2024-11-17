import json
import pathlib as lpath
import matplotlib.pyplot as plt
import itertools as it

import params as p

N_ITER = 20

def plot_injection_queue_depth(path) -> None:
    plt.xlabel("spawn count")
    plt.ylabel("mean time, ms")

    metrics_path = p.TARGET_PATH / "metrics" / "spawner.json"

    for n_iter in range(N_ITER):
        metrics_path = p.TARGET_PATH / "metrics" / "spawner" / f"iter_{n_iter}.json"

        metrics = json.load(metrics_path.open())

        data_x = []
        data_y = []
        time = 0.0
        for metr in metrics:

            val = metr["injection_queue_depth"]

            secs = metr["elapsed"]["secs"]
            nanos = metr["elapsed"]["nanos"]

            time += nanos / 10 ** 6 + secs * 10 ** 3

            data_x.append(time)
            data_y.append(val)

        plt.scatter(data_x, data_y)

    plt.savefig(path)
    plt.clf()

def plot_total_steal_count(path) -> None:
    plt.xlabel("spawn count")
    plt.ylabel("mean time, ms")

    for n_iter in range(N_ITER):
        metrics_path = p.TARGET_PATH / "metrics" / "spawner" / f"iter_{n_iter}.json"

        metrics = json.load(metrics_path.open())

        data_x = []
        data_y = []
        time = 0.0
        for metr in metrics:

            val = metr["total_steal_count"]

            secs = metr["elapsed"]["secs"]
            nanos = metr["elapsed"]["nanos"]

            time += nanos / 10 ** 6 + secs * 10 ** 3

            data_x.append(time)
            data_y.append(val)

        plt.scatter(data_x, data_y)

    plt.savefig(path)
    plt.clf()

def run():
    plot_injection_queue_depth(p.PLOTS_PATH / "injection_queue_depth")
    plot_total_steal_count(p.PLOTS_PATH / "total_steal_count")