#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    typedef struct {
        double x;
        int32_t a;
        int32_t b;
    } Small;

    typedef struct {
        int32_t a;
        double x;
        int32_t b;
    } Big;

    printf("Size of Small: %zu\n", sizeof(Small));
    printf("Size of Big  : %zu\n", sizeof(Big));
    printf("Size of Big  : %zu\n", sizeof(Big[2]));

    int array[5] = {[1] = 1, [3] 3};
    printf("array: [%d", array[0]);
    for (int i = 1; i < 5; i++) {
        printf(", %d", array[i]);
    }
    printf("]\n");
}
