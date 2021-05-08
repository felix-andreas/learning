#include <math.h>
#include <stdio.h>

float inverse_square_root_math(float number) { return 1.0f / sqrt(number); }

float inverse_square_root_quake(float number) {
    long i;
    float x2, y;
    const float threehalfs = 1.5f;

    x2 = number * 0.5f;
    y = number;
    i = *(long *)&y;
    i = 0x5f3759df - (i >> 1);
    y = *(float *)&i;
    return y * (threehalfs - (x2 * y * y));
}

int main(void) {
    FILE *file;
    file = fopen("results.csv", "w");
    for (float number = 1.0f; number < 1000.0f; number++) {
        float result_math = inverse_square_root_math(number);
        float result_quake = inverse_square_root_quake(number);
        fprintf(file, "%f %f %f\n", number, result_math, result_quake);
    }
    fclose(file);
}