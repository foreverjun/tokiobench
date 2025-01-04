import pathlib as lpath
import argparse

import params as p

import bench
import metrics

def main():
    parser = argparse.ArgumentParser(
        prog='graphpy',
        description='Draw cringe graphs for my diploma',
        epilog="Duty dies last")

    # common
    parser.add_argument("-p", "--prefix", default=None)

    # enable benches
    parser.add_argument("-bl", "--bline", action="store_true", default=False)
    parser.add_argument("-bs", "--bscatter", action="store_true", default=False)

    # enable metrics
    parser.add_argument("-ms", "--msampling", action="store_true", default=False)
    parser.add_argument("-mt", "--mtotal", action="store_true", default=False)

    args = parser.parse_args()

    if not p.TARGET_PATH.exists():
        print("No `target` found")
        return

    if args.prefix:
        p.RESULT_PATH = p.RESULT_PATH / args.prefix

    lpath.Path(p.RESULT_PATH).mkdir(mode=0o777, parents=True, exist_ok=True)

    if args.bscatter:
        bench.nspawn_nspawner_scatter()
        bench.nspawn_nspawner_nworker_scatters()

    if args.bline:
        bench.nspawn_nspawner_nworker_line()

    if args.mtotal:
        metrics.run_sum_total()
    if args.msampling:
        metrics.run_sampling()

if __name__ == "__main__":
    main()