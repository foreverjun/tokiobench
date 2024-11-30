from subprocess import run as prun
import os

import params

if __name__ == "__main__":
    for br in params.BRANCHES:
        prun(["uv", "run", "main.py",
              "-r", f"../target/{br}",
              "-c", f"../target/branhes/{br}/criterion",
              "-p", "full" if os.environ.get("FULL") else "default" ],
              cwd="graphs")