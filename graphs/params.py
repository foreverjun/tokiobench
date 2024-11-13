import pathlib as lpath

TARGET_PATH = lpath.Path().absolute() / "target"

CRITERION_PATH = TARGET_PATH / "criterion"
PLOTS_PATH = TARGET_PATH / "plots"

NS_SPAWN = [100, 200, 250, 300, 500, 1000, 5000, 10_000, 50_000, 100_000]
NS_WORKERS = [1, 2, 4, 8, 12]
NS_SPLIT = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]

N_SPAWN = 100000
