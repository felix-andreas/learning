#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

static long num_steps;
double step;

int main(int argc, char *argv[]) {
    if (argc == 1) {
        num_steps = 1000000000;
    }
    else {
        num_steps = atoi(argv[1]);
    }


    step = 1.0 / (double) num_steps;
    double x, sum=0.0, t1;

    t1 = omp_get_wtime();
    for (int i=0; i < num_steps; ++i) {
        x = (i + 0.5) * step;
        sum += 4.0 / (1.0 + x * x);
    }

    t1 = omp_get_wtime() - t1;
    printf("Time elapsed: %fs\n", t1);

    double pi = step * sum;
    printf("Pi equals %f\n\n", pi);

}