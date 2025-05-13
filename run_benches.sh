#!/bin/bash
COMMITS=("d32268acf3e33a42a202a0ebf18c3a20980d8936" "288679b0aaa0fd7c9a9210653837249eca148343")
set -e

if [ "$(id -u)" -ne 0 ]; then
    echo "sudo -E env "PATH=\$PATH" ./run_benches.sh"
    exit 1
fi

TOTAL_RUNS=${#COMMITS[@]}

RESULTS_DIR="./bench_results"
mkdir -p "$RESULTS_DIR"

for i in "${!COMMITS[@]}"; do
    COMMIT="${COMMITS[$i]}"
    RUN_NUMBER=$((i + 1))
    
    echo "=========================================="
    echo "Запуск $RUN_NUMBER из $TOTAL_RUNS. Коммит: $COMMIT"
    echo "=========================================="

	cd tokio
    git checkout --force "$COMMIT" || { echo "Не удалось перейти на коммит $COMMIT"; exit 1; }
	cd ..

    cargo clean

    START_TIME=$(date +%s)
    
    nice -n -20 cargo bench

    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))

    echo "Запуск $RUN_NUMBER занял $DURATION секунд."

    COMMIT_SHORT=$(git rev-parse --short "$COMMIT")
    RESULT_ARCHIVE="$RESULTS_DIR/criterion_results_$COMMIT_SHORT.tar.gz"
    tar -czf "$RESULT_ARCHIVE" -C target criterion

    echo "Результаты сохранены в $RESULT_ARCHIVE"
done

echo "Все $TOTAL_RUNS запусков завершены."
