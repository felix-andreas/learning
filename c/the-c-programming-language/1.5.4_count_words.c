#include <stdio.h>
#include <string.h>

const int IN = 1;
const int OUT= 0;

int main (int argc, char * argv[]) {
    int c, nl, nw, nc, state;

    state = OUT;
    nl = nw = nc = 0;
    int len = strlen(argv[1]);
    for (int i = 0; i < len; ++i) {
        c = argv[1][i];
        ++nc;
        if (c == '\n') {
            ++nl;
        }
        if (c == ' ' || c == '\n' || c == '\t') {
            state = OUT;
        } 
        else if (state == OUT) {
            state = IN;
            ++nw;
        }
    }
    printf("\nNumber of lines: %d\nNumber of words: %d\nNumber of chars: %d\n", nl, nw, nc);
}


