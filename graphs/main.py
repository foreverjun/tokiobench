#!/usr/bin/env python

import pathlib as lpath
import matplotlib.pyplot as plt
import argparse
import itertools as it

import params as p

import bench as bench
import metrics

def main():
    parser = argparse.ArgumentParser(
        prog='graphpy',
        description='Draw cringe graphs for my diploma',
        epilog="Duty dies last")

    # common
    parser.add_argument("-p", "--prefix", default=None)

    # enable benches
    parser.add_argument("-b", "--bench", action="store_true", default=False)
    # enable metrics
    parser.add_argument("-s", "--sampling", action="store_true", default=False)
    parser.add_argument("-t", "--total", action="store_true", default=False)

    args = parser.parse_args()

    if not p.TARGET_PATH.exists():
        print("No `target` found")
        return

    if args.prefix:
        p.RESULT_PATH = p.RESULT_PATH / args.prefix

    lpath.Path(p.RESULT_PATH).mkdir(mode=0o777, parents=True, exist_ok=True)

    if args.bench:
        bench.run()

    if args.total:
        metrics.run_total()
    if args.sampling:
        metrics.run_sampling()

if __name__ == "__main__":
    main()