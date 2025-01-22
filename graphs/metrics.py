import pathlib as lpath
import re
import sys
import json

import matplotlib.pyplot as plt
import matplotlib
import pandas as pd
import seaborn as sns

import params as p

def fetch_n(name: str, path: lpath.Path, *, is_dir: bool) -> int | None:
    assert path.is_dir() is is_dir

    pattern = re.compile(fr"{name}:(\d+)")
    match = pattern.match(path.name)
    if not match:
        return None

    return int(match.group(1))

def fetch_sampling_iters() -> list[dict[str, int | str | lpath.Path]]:
    res = []
    for worker_path in (p.METRICS_PATH / "sampling").glob("*"):
        nworker= fetch_n("nworker", worker_path, is_dir=True)
        if not nworker: continue
        for nspawner_path in worker_path.glob("*"):
            nspawner = fetch_n("nspawner", nspawner_path, is_dir=True)
            if not nspawner: continue
            for nspawn_path in nspawner_path.glob("*"):
                nspawn = fetch_n("nspawn", nspawn_path, is_dir=True)
                if not nspawn: continue
                for name_path in nspawn_path.glob("*"):
                    for iter_path in name_path.glob("*"):
                        niter = fetch_n("iter", iter_path, is_dir=False)
                        res.append({
                            "nworker": nworker,
                            "nspawner": nspawner,
                            "nspawn": nspawn,
                            "niter": niter,
                            "path": iter_path,
                            "name": name_path.name,
                        })
    return res

def group_by(grouper: str, data: list[dict[str, int | str | lpath.Path]]) -> dict[int | str, list[dict[str, int | str | lpath.Path]]]:
    diffs = set(map(lambda i: i[grouper], data))

    res = { }
    for key in diffs:
        assert (not isinstance(key, lpath.Path))
        res[key] = [ d for d in data if d[grouper] == key]

    return res

def run_sampling():
    def mk_resdir(*, name: str, nworker: int, nspawner: int, nspawn: int):
        header = p.RESULT_PATH / "sampling" / name
        res_dir = header / f"nworker:{nworker}" / f"nspawner:{nspawner}" / f"nspawn:{nspawn}"
        res_dir.mkdir(mode=0o777, parents=True, exist_ok=True)

        return res_dir

    def merge_iterations(data: list[dict[str, int | str | lpath.Path]]) -> pd.DataFrame:
        all_data = []
        for d in data:
            df = pd.read_csv(d["path"])
            df['iteration'] = d["niter"]
            all_data.append(df)
        return pd.concat(all_data, ignore_index=True)

    def scatter_plot(df, result_path: lpath.Path,):
        metrics = {
            "total_steal_operations": "",
            "global_queue_depth": "global queue depth",
            "total_overflow_count": "",
            "total_local_queue_depth": "total local queue depth",
        }

        fig, axs = plt.subplots(1, len(metrics))

        fig.set_figwidth(30)
        fig.set_figheight(15)

        for n_ax, (metric, m_name) in enumerate(metrics.items()):
            xs = df["time_nanos"].to_numpy()
            ys = df[metric].to_numpy()
            ax = axs[n_ax]

            ax.set_title(metric)

            sns.scatterplot(x=xs, y=ys, ax=axs[n_ax])

        fig.savefig(result_path / "result")
        plt.close()

    data = fetch_sampling_iters()

    for name, name_data in group_by("name", data).items():
        for nworker, nworker_data in group_by("nworker", name_data).items():
            for nspawner, nspawner_data in group_by("nspawner", nworker_data).items():
                for nspawn, nspawn_data in group_by("nspawn", nspawner_data).items():
                    print(f"running sampling for {name} nworker: {nworker}, nspawner: {nspawner}, nspawn: {nspawn}")
                    df = merge_iterations(nspawn_data)
                    res_dir = mk_resdir(name=name, nworker=nworker, nspawner=nspawner, nspawn=nspawn)

                    df["time_nanos"] = df.groupby('iteration')['elapsed'].cumsum()

                    scatter_plot(df, res_dir)


def fetch_total() -> list[dict[str, int | str | lpath.Path]]:
    res = []
    for worker_path in (p.METRICS_PATH / "total").glob("*"):
        nworker= fetch_n("nworker", worker_path, is_dir=True)
        if not nworker: continue
        for nspawner_path in worker_path.glob("*"):
            nspawner = fetch_n("nspawner", nspawner_path, is_dir=True)
            if not nspawner: continue
            for nspawn_path in nspawner_path.glob("*"):
                nspawn = fetch_n("nspawn", nspawn_path, is_dir=True)
                if not nspawn: continue
                for name_path in nspawn_path.glob("*"):
                    for json_path in name_path.glob("total.json"):
                        res.append({
                            "nworker": nworker,
                            "nspawner": nspawner,
                            "nspawn": nspawn,
                            "name": name_path.name,
                            "path": json_path,
                        })
    return res

def run_sum_total():
    def plot_sum_total(*, name: str, nworker: int, nspawn: int, data: list[dict[str, int | str | lpath.Path]], res_dir: lpath.Path) -> None:
        metrics = ["remote_schedule_count", "spawned_tasks_count", "worker_steal_operations"]

        fig, axs = plt.subplots(1, len(metrics))
        data = sorted(data, key=lambda t: t["nspawner"])

        nspawner = list(map(lambda t: t["nspawner"], data))
        json_data = list(map(lambda t: json.load(t["path"].open()), data))

        for ind, metric in enumerate(metrics):
            ax = axs[ind]

            def sum_or(value):
                if isinstance(value, int):
                    return value

                return sum(value)

            y_points = list(map(lambda j: sum_or(j[metric]), json_data))
            if any(y > 0 for y in y_points):
                ax.set_yscale('log')

            x_points = nspawner

            ax.stem(x_points, y_points)

            ax.set_xlabel("number of spawners")
            ax.set_title(metric)

        print("total_sum saved in in:", res_dir)

        fig.set_figwidth(15)
        fig.set_figheight(10)

        fig.suptitle(f"Benchmark: {name} workers: {nworker} leaf tasks: {nspawn}")
        fig.savefig(res_dir / "total_sum")

    data = fetch_total()
    for name, name_data in group_by("name", data).items():
        for nworker, nworker_data in group_by("nworker", name_data).items():
            for nspawn, nspawn_data in group_by("nspawn", nworker_data).items():
                print(f"runing total for {name}: nworker: {nworker}, nspawn: {nspawn}")

                res_dir = p.RESULT_PATH / "total" / name / f"nworker:{nworker}" / f"nspawn:{nspawn}"
                res_dir.mkdir(mode=0o777, parents=True, exist_ok=True)

                plot_sum_total(name=name, nworker=nworker, nspawn=nspawn, data=nspawn_data, res_dir=res_dir)

def run_total_steal_ops():
    plt.close()
    plt.figure(figsize=(10, 10))

    data = fetch_total()
    for name, name_data in group_by("name", data).items():
        for nspawn, nspawn_data in group_by("nspawn", name_data).items():
            print(f"runing total worker for {name}: nspawn: {nspawn}")

            res_dir = p.RESULT_PATH / "total" / name / f"nspawn:{nspawn}"
            res_dir.mkdir(mode=0o777, parents=True, exist_ok=True)
            legend =[]

            for nworker, nworker_data in group_by("nworker", nspawn_data).items():

                nworker_data = sorted(nworker_data, key=lambda i: i["nspawner"])

                x_values = list(map(lambda d: d["nspawner"], nworker_data))

                jsons = list(map(lambda d: json.load(d["path"].open()), nworker_data))
                y_values = list(map(lambda d: sum(d["worker_steal_operations"]), jsons))

                plt.errorbar(x_values, y_values)
                legend.append(f"{nworker} workers")

            plt.xlabel("Number of spawners")
            plt.ylabel("Steal operations")
            plt.yscale("log")

            plt.title(f"Number of leaf tasks per spawner: {nspawn}")

            plt.legend(legend)
            plt.savefig(res_dir / f"res")
