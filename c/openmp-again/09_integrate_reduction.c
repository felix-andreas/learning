#include <stdio.h>
#include <stdlib.h>

static long num_steps;
double step;

int main(int argc, char *argv[]) {
  if (argc == 1) {
    num_steps = 1000000;
  } else {
    num_steps = atoi(argv[1]);
  }

  int i;
  double pi, sum = 0.0;

  step = 1.0 / (double)num_steps;

#pragma omp parallel for reduction(+ : sum)
  for (i = 0; i < num_steps; i++) {
    double x = (i + 0.5) * step;
    sum += 4.0 / (1.0 + x * x);
  }

  pi = step * sum;
  printf("pi is approximately %f\n", pi);
}