#include <stdio.h>

int main(void) {
    char *name = "Emaunel";
    char *template = "Hello, %s!";
    char buffer[100];
    sprintf(buffer, template, name);
    puts(buffer);
}
