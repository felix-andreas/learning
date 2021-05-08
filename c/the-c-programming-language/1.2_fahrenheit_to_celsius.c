#include <stdio.h>
// #include <math.h>

int main(void) {
    double fahren, celsius;
    printf("Fahrenheit Celsius\n");
    int lower = 0, upper = 300, step = 20;
    for (fahren=0; fahren<=300; fahren+=20) {
        celsius = 5.0 / 9.0 * (fahren - 32);
        printf("%10.1f %7.1f\n", fahren, celsius);
    }
}
