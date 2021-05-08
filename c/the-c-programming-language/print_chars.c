#include <stdio.h>

int main(void) {
    for (char i=0; i<127; i++){
        printf("%c: %3d\n", i, i);
    }
    char c = 127;
    printf("%c: %3d\n", c, c);
}