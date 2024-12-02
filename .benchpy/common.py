from subprocess import run as prun
import sys
from pathlib import Path
import os

TARGET = Path("target")
CRITERION = TARGET / "criterion"

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

def save_result(*, src: Path, br: str,) -> None:
    dest_prefix = TARGET  / "branhes" / br
    dest_prefix.mkdir(parents=True, exist_ok=True)
    sprun(["cp", "-r", str(src), str(dest_prefix)])

def sprun(*args, **kars):
    prun(*args, **kars, check=True)

def mklogfile(name: str) -> Path:
    res = Path(f"target/{name}")
    res.parent.mkdir(parents=True, exist_ok=True)
    return res