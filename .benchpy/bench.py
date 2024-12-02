import subprocess
from pathlib import Path
import os

import common as c
import params

if __name__ == "__main__":
    for br in params.BRANCHES:
        c.switch(br)

        profile = "-F full" if os.environ.get("FULL") else ""

        print(f"running `cargo bench` for {br} with {profile}")
        with open(c.mklogfile("bench.err"), "w") as stderr:
            c.sprun(["cargo", "bench", profile],
                    stdout=subprocess.DEVNULL,
                    stderr=stderr)

        c.save_result(src=c.TARGET / "criterion", br=br)

