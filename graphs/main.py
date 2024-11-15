#!/usr/bin/env python

import json
import pathlib as lpath
import matplotlib.pyplot as plt
import argparse
import itertools as it
import numpy as np

import params as p

import remote as remote
import spawner as spawner
import workload as workload

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog='graphpy',
        description='Draw cringe graphs for my diploma',
        epilog="Duty dies last")

    parser.add_argument("profile")

    args = parser.parse_args()

    lpath.Path(p.PLOTS_PATH).mkdir(mode=0o777, parents=True, exist_ok=True)

    plt.figure(figsize=(10,10))

    remote.run()
    spawner.run()
    workload.run()
