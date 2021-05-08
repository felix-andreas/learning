#include <stdio.h>

int main(void) {
    int c, i, nwhite, nother;
    int ndigit[10];

    nwhite = nother = 0;
    for (i = 0; i < 10; ++i) {
        ndigit[i] = 0;
    }

    while ((c = getchar()) != EOF) {
        if (c >= '0' && c <= '9') {
            ++ndigit[c-'0'];
        }
        else if (c == ' ' || c == '\t' || c == '\n') {
            ++nwhite;
        }
        else {
            ++nother;
        }
    }
    printf("\ndigits\n");
    for (int i = 0; i < 10; i++) {
        printf("%3d: %3d\n", i, ndigit[i]);
    }
    printf("whitespace: %3d, other: %3d\n", nwhite, nother);
}