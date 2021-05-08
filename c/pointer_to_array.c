#include <stdio.h>

int main(void) {
    int array[][5] = {{0, 1, 2, 3, 4}};
    int(*pointer)[][5] = &array;
    printf("address of    array: %p\n", array);
    printf("address of   *array: %p\n", *array);
    printf("address of   &array: %p\n", &array);
    printf("address of &pointer: %p\n", &pointer);
}
