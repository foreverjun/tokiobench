import json
import pathlib as lpath
import re
from pathlib import Path
import matplotlib.pyplot as plt

import params as p

def dirs() -> list[lpath.Path]:
    return list(p.CRITERION_PATH.glob("*"))

def bench_dirs(d: lpath.Path, pattern: re.Pattern[str]) -> list[lpath.Path]:
    return [f for f in d.glob("*") if f.is_dir() and pattern.match(f.name)]

Frame = dict[str, int | str, Path]

def fetch() -> list[Frame]:
    dir_pattern = re.compile(r"nruntime\((\d+)\)_nworker\((\d+)\)_nspawner\((\d+)\)_nspawn\((\d+)\)")

    data = []
    for d in dirs():
        bench_paths = [f for f in d.glob("*") if f.is_dir() and dir_pattern.match(f.name)]

        if len(bench_paths) == 0:
            continue

        for bench in bench_paths:
            estp_path = bench / "new" / "estimates.json"
            bpath_path = bench / "new" / "benchmark.json"

            estimates_json = json.load(estp_path.open())
            benchmark_json = json.load(bpath_path.open())

            thrpt = (benchmark_json["throughput"]["Elements"] / estimates_json["mean"]["point_estimate"]) * 10 ** 9
            name = benchmark_json["function_id"]

            match = re.compile(r"nruntime\((\d+)\)/nworker\((\d+)\)/nspawner\((\d+)\)/nspawn\((\d+)\)").match(name)
            data.append({
                "nruntime": int(match.group(1)),
                "nworker": int(match.group(2)),
                "nspawner": int(match.group(3)),
                "nspawn": int(match.group(4)),
                "thrpt": thrpt,
                "name": d.name
            })

    return data

FrameClasses = dict[str | int | Path, list[Frame]]

def group_by(frames: list[Frame], key) -> FrameClasses:
    uneq_frames = set(map(lambda d: d[key], frames))

    res = {}
    for uneq in uneq_frames:
        res[uneq] = []

    for frame in frames:
        res[frame[key]].append(frame)

    return res

def plot_line(frames: list[Frame]):
    trhpts = list(map(lambda f: f["thrpt"], frames))
    nspawners = list(map(lambda f: f["nspawner"], frames))

    plt.errorbar(nspawners, trhpts)

def plot(*, path: lpath.Path, frames: list[Frame]):
    plt.figure(figsize=(10, 10))
    plt.title("Syntetic multi-runtime system througput. The highest value is better.")

    legend = []

    for nspawn, frames in group_by(frames, "nspawn").items():
        assert isinstance(nspawn, int)
        print("plotting for nspawn:", nspawn)

        for nruntime, frames in sorted(group_by(frames, "nruntime").items(), key=lambda x: x[0]):
            assert isinstance(nruntime, int)
            print("plotting for nruntime:", nruntime)

            plot_line(sorted(frames, key=lambda f: f["nspawner"]))
            legend.append(f"{nruntime} runtime")

        plt.legend(legend)

        plt.xlabel("Number of spawners")
        plt.ylabel("Throughput (task / s)")

        plt.gca().ticklabel_format(axis='y', style='plain')

        plt.savefig(path / f"line_{nspawn}")
        plt.savefig(path / f"line_{nspawn}_transparent", transparent=True)
        plt.close()

def run():
    frames = fetch()

    for name, frames in group_by(frames, "name").items():
        path = p.RESULT_PATH / name
        path.mkdir(mode=0o777, parents=True, exist_ok=True)

        plot(path=path, frames=frames)
