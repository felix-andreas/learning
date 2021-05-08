#include <stdio.h>

int main(void) {
  int c;
  while (1) {
    printf("\n\nEnter char: \n");
    c = getchar();
    printf("\n%d", c);
  }
  return 0;
}