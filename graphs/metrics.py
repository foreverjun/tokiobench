import pathlib as lpath
import re
import sys
import json

import matplotlib.pyplot as plt
import matplotlib
import numpy as np
import pandas as pd
import seaborn as sns
from statsmodels.nonparametric.smoothers_lowess import lowess

import params as p

NAMES = [f for f in p.METRICS_PATH.glob("*") if f.is_dir()]


# https://github.com/mwaskom/seaborn/issues/552#issuecomment-1668374877
def regplot_lowess_ci(data, x, y, ci_level, n_boot, **kwargs):
    x_ = data[x].to_numpy()
    y_ = data[y].to_numpy()
    x_grid = np.linspace(start=x_.min(), stop=x_.max(), num=1000)

    def reg_func(_x, _y):
        return lowess(exog=_x, endog=_y, xvals=x_grid, frac=0.5)

    beta_boots = sns.algorithms.bootstrap(
        x_, y_,
        func=reg_func,
        n_boot=n_boot,
    )
    err_bands = sns.utils.ci(beta_boots, ci_level, axis=0)
    y_plt = reg_func(x_, y_)

    ax = sns.lineplot(x=x_grid, y=y_plt, color='red', **kwargs)
    sns.scatterplot(x=x_, y=y_, ax=ax, **kwargs)
    ax.fill_between(x_grid, *err_bands, alpha=.15, **kwargs)
    return ax


def scatter_plot(df, labels: list[str], result_path: lpath.Path, from_i: int = 0, to_i: int = sys.maxsize):
    filtered = df[(df.iteration >= from_i) & (df.iteration <= to_i)]
    for label in labels:
        sns.relplot(
            data=filtered,
            x="time_nanos", y=label,
        ).savefig(result_path / (label+"scatter"))
        matplotlib.pyplot.close()


def lowess_plot(df, labels: list[str], result_path: lpath.Path, from_i: int = 0, to_i: int = sys.maxsize):
    filtered = df[(df.iteration >= from_i) & (df.iteration <= to_i)]
    for label in labels:
        regplot_lowess_ci(
            data=filtered,
            x="time_nanos", y=label, ci_level=95, n_boot=100
        ).figure.savefig(result_path / (label+"lowess"))
        print("saved")
        matplotlib.pyplot.close()

def merge_iters(base_dir):
    all_data = []
    if os.path.isdir(base_dir):
        for file_name in os.listdir(base_dir):
            if file_name.endswith('.csv'):
                file_path = os.path.join(base_dir, file_name)
                iteration_match = re.search(r'iter\((\d+)\)\.csv', file_name)

                if iteration_match:
                    iteration = int(iteration_match.group(1))
                    print(iteration)
                    df = pd.read_csv(file_path)
                    df['iteration'] = iteration
                    all_data.append(df)

    if all_data:
        result_df = pd.concat(all_data, ignore_index=True)
        return result_df
    else:
        return pd.DataFrame()

def process_df(df, plot_dir):
    print("runned")
    labels = ["global_queue_depth", "total_steal_count", "total_local_queue_depth", ]
    df["time_nanos"] = df.groupby('iteration')['elapsed'].cumsum()
    print("grouped")
    print(df.shape)
    lowess_plot(df, labels, plot_dir)
    print("finished")

def run_sampling():
    spawner_pattern = re.compile(r"sampling\((.*?)\)_nspawn\((\d+)\)_nworker\((\d+)\)")
    workload_pattern = re.compile(r"sampling\((.*?)\)_nspawn\((\d+)\)_nspawner\((\d+)\)")
    sns.set_theme()
    for bname in NAMES:
        print(bname.name)
        match = spawner_pattern.match(bname.name)
        if not match:
            match = workload_pattern.match(bname.name)
        if not match or not os.path.isdir(bname):
            continue
        sampling = match.group(1)
        nspawn = match.group(2)
        nworkers = match.group(3)
        print("matched")
        bdir = p.PLOTS_PATH / sampling
        bdir.mkdir(mode=0o777, parents=True, exist_ok=True)
        plot_dir = bdir / f"ns({nspawn})nw({nworkers})"
        plot_dir.mkdir(mode=0o777, parents=True, exist_ok=True)
        df = merge_iters(bname)
        process_df(df, plot_dir)

class Json:
    pass

class NSpawner:
    pass

def read_total(base_dir: lpath.Path) -> Json:
    for path in base_dir.glob("*"):
        file_name = path.name

        if file_name != "total.json":
            continue

        return json.load(path.open())

def plot_histogram(data: list[int], name: str, plot_path: lpath.Path) -> None:
    print("data", data)

    pass

# metrics per worker
def plot_total(json_data: Json, plot_dir: lpath.Path) -> None:
    metrics = ["worker_local_schedule_count", "worker_steal_operations", "worker_overflow_count"]
    lable   = ["local shedule", "steal operations", "overflow count"]

    fig, axs = plt.subplots(1, len(metrics))
    for ind, metric in enumerate(metrics):
        ax = axs[ind]

        y_points = json_data[metric]
        x_points = list(range(1, len(y_points) + 1))

        assert(len(y_points) == len(x_points))

        ax.stem(x_points, y_points)

        ax.set_xlabel("nworker value")
        ax.set_title(lable[ind])

    print("total saved in:", plot_dir)
    fig.savefig(plot_dir / "total")

# oveall metrics
def plot_sum_total(json_data: list[NSpawner, Json], plot_dir: lpath.Path) -> None:
    metrics = ["worker_local_schedule_count", "worker_steal_operations", "worker_overflow_count"]
    lable   = ["local shedule", "steal operations", "overflow count"]

    fig, axs = plt.subplots(1, len(metrics))

    json_data = sorted(json_data, key=lambda t: t[0])

    nspawn = list(map(lambda t: t[0], json_data))
    json_data = list(map(lambda t: t[1], json_data))

    for ind, metric in enumerate(metrics):
        ax = axs[ind]

        y_points = list(map(lambda j: sum(j[metric]), json_data))
        x_points = nspawn

        ax.stem(x_points, y_points)

        ax.set_xlabel("nspawn value")
        ax.set_title(lable[ind])

    print("total_sum saved in in:", plot_dir)
    fig.savefig(plot_dir / "total_sum")

def run_total():
    pattern = re.compile(r"total\((.*?)\)_nspawn\((\d+)\)_nspawner\((\d+)\)")
    sns.set_theme()

    nspawner_data: list[int, Json] = []

    for bname in NAMES:
        print("processing:", bname.name)

        match = pattern.match(bname.name)
        if not match or not bname.is_dir():
            continue

        print("matched:", bname.name)

        total_name = match.group(1)
        nspawn = match.group(2)
        nspawner = match.group(3)

        bdir = p.RESULT_PATH / total_name
        bdir.mkdir(mode=0o777, parents=True, exist_ok=True)

        plot_dir = bdir / f"npawn({nspawn})nspawner({nspawner})"
        plot_dir.mkdir(mode=0o777, parents=True, exist_ok=True)

        json_data = read_total(bname)
        plot_total(json_data, plot_dir)

        nspawner_data.append((nspawner, json_data))

    plot_sum_total(nspawner_data, p.RESULT_PATH)

if __name__== "__main__":
    run_sampling()
    run_total()


