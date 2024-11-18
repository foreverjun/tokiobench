#!/usr/bin/env python

import json
import pathlib as lpath
import matplotlib.pyplot as plt
import argparse
import itertools as it
import numpy as np

import params as p

def init_params(profile: str) -> None:
    p.N_SPAWN_GLOBAL = 100_000
    p.N_SPAWN_LOCAL = 10_000
    p.YIEDL_BOUND = 10
    p.NS_SPAWN_LOCAL = [
        50, 100, 150, 200,
        230, 240, 250, 260, 270,
        300, 320, 350, 400, 420, 450,
    ]
    p.NS_SPAWN_GLOBAL = [
        100, 200, 250, 300, 500, 750,
        1000, 2_000, 3000, 4000, 5_000,
        6_000, 7_000, 8_000, 9000, 10_000,
    ]

    match profile:
        case "default":
            p.NS_SPLIT_LOCAL = [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                11, 12
            ]
            p.NS_SPLIT_GLOBAL = [10, 50, 100, 150, 200]
            p.NS_WORKERS = [1, 2, 4, 8, 12]
        case "full":
            p.NS_SPLIT_LOCAL = [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, 22, 23, 24,
            ]
            p.NS_SPLIT_GLOBAL = [10, 50, 100, 150, 200, 250, 300, 350, 400, 450]
            p.NS_WORKERS = [1, 2, 4, 8, 12, 14, 16, 18, 20, 22, 24]

def on_path_or(p: lpath.Path, run, message: str):
    if p.exists():
        run()
    else:
        # logging lib
        print(message)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog='graphpy',
        description='Draw cringe graphs for my diploma',
        epilog="Duty dies last")

    parser.add_argument("-p", "--profile", default="default")
    parser.add_argument("-c", "--criterion", default=p.CRITERION_PATH)

    args = parser.parse_args()

    init_params(args.profile)

    import remote
    import spawner
    import workload
    import metrics

    lpath.Path(p.PLOTS_PATH).mkdir(mode=0o777, parents=True, exist_ok=True)

    plt.figure(figsize=(10,10))

    on_path_or(p.REMOTE_PATH, remote.run, "skip remote")
    on_path_or(p.WORKLOAD_PATH, workload.run, "skip workload")
    on_path_or(p.SPAWNER_PATH, spawner.run, "skip spawner")

    on_path_or(p.METRICS_PATH, metrics.run, "skip metrics")
