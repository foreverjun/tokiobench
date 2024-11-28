from subprocess import run as prun
from pathlib import Path

def sprun(*args, **kars):
    prun(*args, **kars, check=True)

BRANCHES = ["iea/rev-next-task", "master"]

TARGET = Path("target")
CRITERION = TARGET / "criterion"

def cleanup() -> None:
    sprun(["rm", "-rf", str(TARGET)])

def cargo_bench(*, feats: list[str]) -> None:
    feats = [ f"-F {f}" for f in feats ]
    sprun(["cargo", "bench", *feats])

def switch(br: str):
    def switch(proj: str, br: str) -> None:
        sprun(["git", "switch", br], cwd=proj)

    def pull(proj: str) -> None:
        sprun(["git", "pull", "--rebase"], cwd=proj)

    switch("tokio", br)

    pull("tokio")
    pull("tokio-metrics")

def destination(br: str) -> Path: return TARGET / "branhes" / br

def save_result(br: str) -> None:
    sprun(["cp", "-r", str(CRITERION), str(destination(br))])

if __name__ == "__main__":
    for b in BRANCHES:
        switch(b)

        print(f"running `cargo bench` for {b}")
        cargo_bench(feats=["full"])

        save_result(b)

