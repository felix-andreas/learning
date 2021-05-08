#include <omp.h>
#include <stdio.h>
const static int num = 4;

int main(void) {
  int A[num];
  omp_set_num_threads(num);

#pragma omp parallel
  {
    int id = omp_get_thread_num();
    A[id] = id;
  }
  printf("A: ");
  for (int i = 0; i < 4; i++) {
    printf("%d ", A[i]);
  }
  printf("\nall done\n");
  return 0;
}
