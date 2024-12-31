import json
import os
import pathlib as lpath
import re
from pathlib import Path

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

import params as p

sns.set_theme()

def dirs() -> list[lpath.Path]:
    return list(p.CRITERION_PATH.glob("*"))

def nspawn_nspawner(path: lpath.Path):
    pattern = re.compile(r"nspawn\((\d+)\)_nspawner\((\d+)\)")
    name_pattern = re.compile(r"nspawn\((\d+)\)/nspawner\((\d+)\)")
    for d in dirs():
        benches = [f for f in os.scandir(d.path) if f.is_dir() and pattern.match(f.name)]
        print(d.name)
        if len(benches) == 0:
            continue
        data = []
        for bench in benches:
            estp_path = Path(bench.path) / "new" / "estimates.json"
            bpath_path = Path(bench.path) / "new" / "benchmark.json"
            sample_path = Path(bench.path) / "new" / "sample.json"

            estp = json.load(estp_path.open())
            min_time = min(json.load(sample_path.open())["times"])
            bpath = json.load(bpath_path.open())
            # thr = bpath["throughput"]["Elements"] / min_time * 10 ** 9
            thr = (bpath["throughput"]["Elements"] / estp["mean"]["point_estimate"]) * 10 ** 9
            name = bpath["function_id"]
            match = name_pattern.match(name)
            spawn_num = int(match.group(1))
            nspawner = int(match.group(2))
            data.append({
                "spawn_num": spawn_num,
                "nspawner": nspawner,
                "throughput": thr
            })

        df = pd.DataFrame(data)

        filename = path / f"{d.name}_scatterplot.png"
        plt.figure(figsize=(12, 6))
        sns.scatterplot(data=df, x="nspawner", y="spawn_num", size="throughput", sizes=(20, 200), hue="throughput",
                        palette="cool", legend="auto")
        plt.savefig(filename, dpi=300, bbox_inches='tight')
        plt.close()

def nspawn_nworker(path: lpath.Path):
    pattern = re.compile(r"nspawn\((\d+)\)_nworker\((\d+)\)")
    name_pattern = re.compile(r"nspawn\((\d+)\)/nworker\((\d+)\)")
    for d in dirs():
        benches = [f for f in os.scandir(d.path) if f.is_dir() and pattern.match(f.name)]
        print(d.name)
        if len(benches) == 0:
            continue
        data = []
        for bench in benches:
            estp_path = Path(bench.path) / "new" / "estimates.json"
            bpath_path = Path(bench.path) / "new" / "benchmark.json"

            estp = json.load(estp_path.open())
            bpath = json.load(bpath_path.open())

            thrpt = (bpath["throughput"]["Elements"] / estp["mean"]["point_estimate"]) * 10 ** 9
            name = bpath["function_id"]
            match = name_pattern.match(name)

            ntask = int(match.group(1))
            nworker = int(match.group(2))
            data.append({
                "ntask": ntask,
                "nworker": nworker,
                "thrpt": thrpt
            })

        df = pd.DataFrame(data)

        filename = path / f"{d.name}_scatterplot.png"
        plt.figure(figsize=(12, 6))
        sns.scatterplot(data=df, x="nworker", y="ntask", size="thrpt", sizes=(20, 200), hue="thrpt",
                        palette="cool", legend="auto")
        plt.savefig(filename, dpi=300, bbox_inches='tight')
        plt.close()

def nspawn_nspawner_nworker(path: lpath.Path):
    pattern = re.compile(r"nspawn\((\d+)\)_nspawner\((\d+)\)_nworker\((\d+)\)")
    name_pattern = re.compile(r"nspawn\((\d+)\)/nspawner\((\d+)\)/nworker\((\d+)\)") # nspawn(1000)/nspawner(1)/nworker(4)

    for d in dirs():
        bench_paths = [f for f in d.glob("*") if f.is_dir() and pattern.match(f.name)]

        if len(bench_paths) == 0:
            continue

        data = { }

        for bench in bench_paths:
            estp_path = bench / "new" / "estimates.json"
            bpath_path = bench / "new" / "benchmark.json"

            estimates_json = json.load(estp_path.open())
            benchmark_json = json.load(bpath_path.open())

            thrpt = (benchmark_json["throughput"]["Elements"] / estimates_json["mean"]["point_estimate"]) * 10 ** 9
            name = benchmark_json["function_id"]
            print("name:", name)
            match = name_pattern.match(name)

            nspawn = int(match.group(1))
            nspawner = int(match.group(2))
            nworker = int(match.group(3))

            data[nworker] = data.get(nworker, [])
            data[nworker].append({
                "nspawn": nspawn,
                "nspawner": nspawner,
                "thrpt": thrpt
            })

        print(data)

        plt.figure(figsize=(100, 100))
        filename = path / f"{d.name}_scatterplot.png"

        # fig, axes = plt.subplots(len(data))
        data = sorted(list(data.items()), key=lambda t: t[0])

        for ind, (nworker, dict_data) in enumerate(data):

            x_values = list(map(lambda d: d["nspawner"], dict_data))
            y_values = list(map(lambda d: d["nspawn"], dict_data))
            z_values = list(map(lambda d: d["thrpt"], dict_data))
            df = pd.DataFrame(dict_data)

            filename = path / f"{d.name}_scatterplot_nworker{nworker}.png"
            plt.figure(figsize=(12, 6))
            sns.scatterplot(data=df, x="nspawner", y="nspawn", size="thrpt", sizes=(20, 200), hue="thrpt",
                            palette="cool", legend="auto")
            plt.savefig(filename, dpi=300, bbox_inches='tight')
            plt.close()

def nspawn_nspawner_nworker_line(path: lpath.Path):
    pattern = re.compile(r"nspawn\((\d+)\)_nspawner\((\d+)\)_nworker\((\d+)\)")
    name_pattern = re.compile(r"nspawn\((\d+)\)/nspawner\((\d+)\)/nworker\((\d+)\)") # nspawn(1000)/nspawner(1)/nworker(4)

    for d in dirs():
        if not "line" in str(d):
            continue

        bench_paths = [f for f in d.glob("*") if f.is_dir() and pattern.match(f.name)]

        if len(bench_paths) == 0:
            continue

        data = { }

        for bench in bench_paths:
            estp_path = bench / "new" / "estimates.json"
            bpath_path = bench / "new" / "benchmark.json"

            estimates_json = json.load(estp_path.open())
            benchmark_json = json.load(bpath_path.open())

            thrpt = (benchmark_json["throughput"]["Elements"] / estimates_json["mean"]["point_estimate"]) * 10 ** 9
            name = benchmark_json["function_id"]
            match = name_pattern.match(name)

            nspawn = int(match.group(1))
            nspawner = int(match.group(2))
            nworker = int(match.group(3))

            print("nspawn:", nspawn)

            data[nspawn] = data.get(nspawn, {})
            data[nspawn][nworker] = data[nspawn].get(nworker, [])

            data[nspawn][nworker].append({
                "nspawner": nspawner,
                "thrpt": thrpt
            })

        plt.figure(figsize=(10, 10))

        data = sorted(list(data.items()), key=lambda t: t[0])

        res_dir = p.RESULT_PATH / d.name
        res_dir.mkdir(0o777, parents=True, exist_ok=True)

        for ind, (nspawn, dict_data) in enumerate(data):
            legend =[]

            for (nworker, w_dict_data) in sorted(dict_data.items(), key=lambda i:i[0]):
                w_dict_data = sorted(w_dict_data, key=lambda i: i["nspawner"])

                x_values = list(map(lambda d: d["nspawner"], w_dict_data))
                y_values = list(map(lambda d: d["thrpt"], w_dict_data))

                plt.errorbar(x_values, y_values)
                legend.append(f"{nworker} workers")

            plt.legend(legend)

            plt.xlabel("Number of spawners")
            plt.ylabel("Throughput (task / s)")

            if "lifo" in str(d):
                plt.title(f"Tatlin benchmark with LIFO task. Number of leaf tasks per spawner: {nspawn}")
            else:
                plt.title(f"Tatlin benchmark. Number of leaf tasks per spawner: {nspawn}")
            # break
            plt.savefig(res_dir / f"line_{nspawn}")



def run():
    common = p.RESULT_PATH / "common"
    common.mkdir(mode=0o777, parents=True, exist_ok=True)

    # nspawn_nspawner(common)
    # nspawn_nworker(common)
    # nspawn_nspawner_nworker(common)
    nspawn_nspawner_nworker_line(common)
