#!/usr/bin/env python

import json
import pathlib as lpath
import matplotlib.pyplot as plt
import argparse
import itertools as it
import numpy as np

import params as p

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

    parser.add_argument("-c", "--criterion_path", default=p.CRITERION_PATH)
    parser.add_argument("-r", "--result_path", default=p.PLOTS_PATH)
    parser.add_argument("-b", "--bench", action="store_true", default=False)

    parser.add_argument("-mp", "--metrics_path", default=p.METRICS_PATH)
    parser.add_argument("-m", "--metrics", action="store_true", default=False)
    args = parser.parse_args()

    p.PLOTS_PATH = lpath.Path(args.result_path)
    p.CRITERION_PATH = lpath.Path(args.criterion_path)
    p.METRICS_PATH = lpath.Path(args.metrics_path)

    lpath.Path(p.PLOTS_PATH).mkdir(mode=0o777, parents=True, exist_ok=True)

    plt.figure(figsize=(10,10))

    if args.bench:
        import scatter
        scatter.run()

    if args.metrics:
        import metrics
        metrics.run()