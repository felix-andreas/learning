#include <omp.h>
#include <stdio.h>
#include <stdlib.h>

static long num_steps;

int main(int argc, char *argv[]) {
  if (argc == 1) {
    num_steps = 1000000;
  } else {
    num_steps = atoi(argv[1]);
  }

  double sum[omp_get_max_threads()];
  double step = 1.0 / (double)num_steps;
  int threads_used;

#pragma omp parallel
  {
    int id = omp_get_thread_num();
    int num_threads = omp_get_num_threads();
    if (id == 0) {
      threads_used = num_threads;
    }

    double tmp = 0;
    for (int i = id; i < num_steps; i += num_threads) {
      double x = (i + 0.5) * step;
      tmp += 4.0 / (1.0 + x * x);
    }

    sum[id] = tmp;
  }

  double pi = 0.0;
  for (int i = 0; i < threads_used; i++) {
    pi += sum[i];
  }

  pi *= step;
  printf("pi is approximately %f with threads used: %d\n", pi, threads_used);
}