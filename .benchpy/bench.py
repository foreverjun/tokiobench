from subprocess import run as prun
import sys
from pathlib import Path
import os

import params

def sprun(*args, **kars):
    prun(*args, **kars, check=True)

TARGET = Path("target")
CRITERION = TARGET / "criterion"

def cleanup() -> None:
    sprun(["rm", "-rf", str(TARGET)])

def cargo_bench(*, feats: list[str]) -> None:
    sprun(["cargo", "bench", *feats])

def switch(br: str):
    def switch(proj: str, br: str) -> None:
        sprun(["git", "switch", br], cwd=proj)

    def pull(proj: str) -> None:
        if os.environ.get("PULL"):
            sprun(["git", "pull", "--rebase"], cwd=proj)

    switch("tokio", br)
    pull("tokio")

    switch("tokio-metrics", "iea/submodule")
    pull("tokio-metrics")

def destination(br: str) -> Path: return TARGET / "branhes" / br

def save_result(br: str) -> None:
    dest = destination(br)
    dest.mkdir(parents=True, exist_ok=True)
    sprun(["cp", "-r", str(CRITERION), str(dest)])

if __name__ == "__main__":
    for b in params.BRANCHES:
        switch(b)

        feats = [ f"-F {f}" for f in sys.argv[1:] ]
        print(f"running `cargo bench` for {b} with {feats}")
        sprun(["cargo", "bench", *feats])

        save_result(b)

