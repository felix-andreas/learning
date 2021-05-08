#include <stdio.h>

int main(int argc, char *argv[]) {
  printf("The number of arguments is: %d\n", argc - 1);

  if (argc == 1) printf("Please provide some command line arguments!\n");
  else printf("You have entered the following arguments:\n");

  for (int i=1; i<argc; i++) {
    printf("%s\n", argv[i]);
  }

}
