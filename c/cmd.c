#include <stdio.h>

int main(int argc, char *argv[]) {
    printf("Hello, I am the robot!\n");
    printf("you have provided %d command line arguments!\n", argc - 1);
    printf("Your arguments are:\n");
    for (int i = 1; i < argc; i++) {
	    printf("%d.) %s\n", i, argv[i]);
    }

    char c;
    while((c = getchar()) != EOF) {
        putchar(c);
    }
}

