#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

double step;
static long num_steps;

int main(int argc, char *argv[]) {
    if (argc == 1) {
        num_steps = 1000000000;
    }
    else {
        num_steps = atoi(argv[1]);
    }

    int i;
    double pi, t1, sum;
    step = 1.0 / (double) num_steps;
    t1 = omp_get_wtime();

    #pragma omp parallel for reduction(+:sum)// schedule(guided)
        for (i=0; i < num_steps; ++i) {
            double x = (i + 0.5) * step;
            sum += 4.0 / (1.0 + x * x);
        }

    t1 = omp_get_wtime() - t1;
    printf("Time elapsed: %fs\n", t1);

    pi = step * sum;
    printf("Pi equals %f\n\n", pi);

}