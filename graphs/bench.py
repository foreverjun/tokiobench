import json
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

def bench_dirs(d: lpath.Path, pattern: re.Pattern[str]) -> list[lpath.Path]:
    return [f for f in d.glob("*") if f.is_dir() and pattern.match(f.name)]

def nspawn_nspawner_scatter():
    pattern = re.compile(r"nspawn\((\d+)\)_nspawner\((\d+)\)")
    name_pattern = re.compile(r"nspawn\((\d+)\)/nspawner\((\d+)\)")

    for d in dirs():
        benches = bench_dirs(d, pattern)
        print("processing:",  d.name)

        if len(benches) == 0:
            continue

        data = []
        for bench in benches:
            estp_path = bench / "new" / "estimates.json"
            bpath_path = bench / "new" / "benchmark.json"

            estp = json.load(estp_path.open())
            bpath = json.load(bpath_path.open())
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

        res_dir = p.RESULT_PATH / d
        res_dir.mkdir(777, True, True)

        plt.figure(figsize=(12, 6))
        sns.scatterplot(data=df,
                        x="nspawner",
                        y="spawn_num",
                        size="throughput",
                        sizes=(20, 200),
                        hue="throughput",
                        palette="cool",
                        legend="auto")
        plt.savefig(res_dir / f"{d.name}_scatterplot",
                    dpi=300,
                    bbox_inches='tight')
        plt.close()

def nspawn_nspawner_nworker_scatters():
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
            print("Processing:", name)
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

        res_dir = p.RESULT_PATH / d
        res_dir.mkdir(777, parents=True, exist_ok=True)

        for nworker, dict_data in sorted(list(data.items()), key=lambda t: t[0]):
            df = pd.DataFrame(dict_data)

            plt.figure(figsize=(12, 6))
            sns.scatterplot(data=df,
                            x="nspawner",
                            y="nspawn",
                            size="thrpt",
                            sizes=(20, 200),
                            hue="thrpt",
                            palette="cool",
                            legend="auto")

            plt.savefig(res_dir / f"{d.name}_scatterplot_nworker{nworker}",
                        dpi=300,
                        bbox_inches='tight')
            plt.close()

def nspawn_nspawner_nworker_line():
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

        for nspawn, dict_data in data:
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
