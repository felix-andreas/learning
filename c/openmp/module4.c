#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

static long num_steps;
double step;
#define NUM_THREADS 12// omp_get_max_threads()

int main(int argc, char *argv[]) {
    if (argc == 1) {
        num_steps = 1000000000;
    }
    else {
        num_steps = atoi(argv[1]);
    }


    double pi, t1, sum=0.0 ;
    step = 1.0 / (double) num_steps;
    omp_set_num_threads(NUM_THREADS);
    t1 = omp_get_wtime();

    #pragma omp parallel
    {
        double x, sum_p = 0.0;
        int i, thread_id = omp_get_thread_num();
        for (i=thread_id = 0.0; i < num_steps; i = i + NUM_THREADS) {
            x = (i + 0.5) * step;
            sum_p += 4.0 / (1.0 + x * x);
        }

        #pragma omp atomic
             sum += sum_p;
    }

    t1 = omp_get_wtime() - t1;
    printf("Time elapsed: %fs\n", t1);
    printf("Threads used: %d\n", NUM_THREADS);

    pi = step * sum;
    printf("Pi equals %f\n\n", pi);

}