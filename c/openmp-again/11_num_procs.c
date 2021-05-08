#include <omp.h>
#include <stdio.h>

int main(void) {
  int num_procs = omp_get_num_procs();
  printf("num_procs: %d\n", num_procs);
}