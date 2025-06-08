#!/usr/bin/env python3
import json
import re
from pathlib import Path
import argparse

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

sns.set_theme(style="whitegrid")
plt.rcParams.update({
    "axes.titlesize": 16,
    "axes.labelsize": 16,
    "xtick.labelsize": 18,
    "ytick.labelsize": 18,
    "legend.fontsize": 18,
})

def compare_heatmap_speedup(orig_root: Path, queue_root: Path, result_root: Path):
    bench_re = re.compile(r"nspawn\((\d+)\)_nspawner\((\d+)\)_nworker\((\d+)\)")
    name_re  = re.compile(r"nspawn\((\d+)\)/nspawner\((\d+)\)/nworker\((\d+)\)")

    rows = []
    for d_orig in orig_root.iterdir():
        if not d_orig.is_dir() or "line" not in d_orig.name:
            continue

        d_queue = queue_root / d_orig.name
        if not d_queue.exists():
            continue

        for bench_orig in d_orig.iterdir():
            if not bench_orig.is_dir() or not bench_re.match(bench_orig.name):
                continue

            bench_queue = d_queue / bench_orig.name
            if not bench_queue.exists():
                continue

            est_o = json.load((bench_orig / "new" / "estimates.json").open())
            bp_o  = json.load((bench_orig / "new" / "benchmark.json").open())
            est_q = json.load((bench_queue / "new" / "estimates.json").open())
            bp_q  = json.load((bench_queue / "new" / "benchmark.json").open())

            thr_o = bp_o["throughput"]["Elements"] / est_o["mean"]["point_estimate"] * 1e9
            thr_q = bp_q["throughput"]["Elements"] / est_q["mean"]["point_estimate"] * 1e9
            speedup = thr_q / thr_o

            m = name_re.match(bp_o["function_id"])
            nspawn   = int(m.group(1))
            nspawner = int(m.group(2))
            nworker  = int(m.group(3))

            rows.append({
                "category":     d_orig.name,
                "nspawn":       nspawn,
                "nspawner":     nspawner,
                "nworker":      nworker,
                "speedup":      speedup
            })

    if not rows:
        print("No data found.")
        return

    df = pd.DataFrame(rows)

    for category, cat_df in df.groupby("category"):
        out_dir = result_root / f"compare_{queue_root.name}" / category
        out_dir.mkdir(parents=True, exist_ok=True)

        for nspawn, spawn_df in cat_df.groupby("nspawn"):

            spawn_df = spawn_df.sort_values('nworker', ascending=False)
            
            pivot_table = spawn_df.pivot(index="nworker", columns="nspawner", values="speedup")
            pivot_table = pivot_table.reindex(sorted(pivot_table.columns), axis=1)


            max_diff = max(abs(pivot_table.max().max() - 1), abs(pivot_table.min().min() - 1))
            vmin = 1 - max_diff
            vmax = 1 + max_diff
            
            plt.figure(figsize=(12, 4))
            ax = sns.heatmap(
                pivot_table,
                annot=True,
                fmt=".2f",
                cmap=sns.diverging_palette(220, 20, as_cmap=True),
                center=1.0,
                vmin=vmin,
                vmax=vmax,
                annot_kws={"size": 14, "weight": "bold"},
                linewidths=0.5,
                linecolor='black',
                cbar_kws={"label": "Ускорение отн. оригинального tokio", "shrink": 0.75}
            )
            

            ax.invert_yaxis()
            
            ax.set_xlabel("Количество задач-производителей", fontsize=16)
            ax.set_ylabel("Количество потоков", fontsize=16)
            ax.set_title(f"{queue_root.name}, задач на каждого производителя: {nspawn}\nТепловая карта ускорения", fontsize=16, pad=20)
            
            ax.tick_params(axis='x', rotation=0)
            ax.tick_params(axis='y', rotation=0)
            
            plt.tight_layout()
            plt.savefig(
                out_dir / f"heatmap_speedup_nspawn{nspawn}.png",
                dpi=300,
                bbox_inches='tight'
            )
            plt.close()

def main():
    parser = argparse.ArgumentParser(
        description="Ускорение между tokio_original и tokio_<queue_name> в виде тепловой карты"
    )
    parser.add_argument("orig",  type=Path, help="Путь к папке tokio_original")
    parser.add_argument("queue", type=Path, help="Путь к папке tokio_<queue_name>")
    parser.add_argument(
        "-o", "--out", type=Path, default=Path("results"),
        help="Путь для сохранения графиков"
    )
    args = parser.parse_args()
    compare_heatmap_speedup(args.orig, args.queue, args.out)

if __name__ == "__main__":
    main()