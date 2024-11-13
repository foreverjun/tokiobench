# tokiobench

The repository presents a set of custom benchmarks for tokio asynchronous runtime. For all subsequent results, the **shorter** the execution time, the **better**.

The parameters of all benchmarks can be found [here](src/params.rs).

## Workload 

This benchmark is aimed at evaluating the scenario of generating a large number of tasks from the small number of producers tasks. And it is implemented in two versions.

- Eq. `nsplit` tasks produce an equal number of subtasks

    - With simulation of cpu bound work in tasks

        ![image info](./plots/workload_recstall_Eq.png)

   - Empty async lambdas

        ![image info](./plots/workload_Eq.png)

- Gradient. Each next task produces `nsplit` times less than the previous one, let me remind you that the number of tasks is fixed.

    - With simulation of cpu bound work in tasks

        ![image info](./plots/workload_recstall_Gradient.png)

   - Empty async lambdas

        ![image info](./plots/workload_Gradient.png)

The source code can be found [here](benches/workload.rs)

## Spawner

This benchmark represents a scenario for the production of a large number of tasks from a single producer's task in two varants.

- Spawn from `block_on` async lambda

    ![image info](./plots/spawn_current.png)

- Spawn from local async task

    ![image info](./plots/spawn_local.png)

The source code can be found [here](benches/spawner.rs)

## License. 

The work was performed under an MIT license, the text of which can be found [here](LICENSE).

## Contribution

Feel free to suggest any changes and criticize the current implementation!








