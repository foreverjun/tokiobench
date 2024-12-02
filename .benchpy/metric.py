import params
import common as c

if __name__ == "__main__":
    for br in params.BRANCHES:
        for metr in params.METRICS:
            c.switch(br)

            c.sprun(["cargo", "run", "--release", "--bin", metr])
            c.save_result(src=c.TARGET / "metrics", br=br)
