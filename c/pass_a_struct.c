// is it possible to pass a struct ?
#include <math.h>
#include <stdio.h>

typedef struct {
    float x;
    float y;
} Vertex;

void scale_by_ref(Vertex *vertex, float scale) {
    vertex->x *= scale;
    vertex->y *= scale;
}

void scale_by_val(Vertex vertex, float scale) {
    vertex.x *= scale;
    vertex.y *= scale;
}

float length(Vertex vertex) {
    return sqrtf(vertex.x * vertex.x + vertex.y * vertex.y);
}

int main(void) {
    Vertex vertex = {3, 4};
    printf("Vertex initial length    : %.2f units\n", length(vertex));
    scale_by_val(vertex, 2.0);
    printf("After scale (pass by val): %.2f units\n", length(vertex));
    scale_by_ref(&vertex, 2.0);
    printf("After scale (pass by ref): %.2f units\n", length(vertex));
}
