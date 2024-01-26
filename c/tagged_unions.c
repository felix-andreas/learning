#include <assert.h>
#include <stdio.h>

typedef struct {
  enum {
    Square,
    Rectangle,
    Circle,
  } tag;
  union {
    // Square
    struct {
      int side;
    };
    // Rectangle
    struct {
      int width, height;
    };
    // Circle
    struct {
      int radius;
    };
  };
} Shape;

Shape square(int side) {
  Shape shape = {.tag = Square, .side = side};
  return shape;
}

Shape rectangle(int width, int height) {
  Shape shape = {.tag = Rectangle, .width = width, height = height};
  return shape;
}

Shape circle(int radius) {
  Shape shape = {.tag = Circle, .radius = radius};
  return shape;
}

void print_shape(Shape shape) {
  switch (shape.tag) {
    case Square:
      printf("Square is %dx%d\n", shape.side, shape.side);
      break;
    case Rectangle:
      printf("Rectangle is %dx%d\n", shape.width, shape.height);
      break;
    case Circle:
      printf("Circle has radius of %d\n", shape.radius);
      break;
    default:
      printf("never\n");
  }
}

int main(void) {
  Shape shapes[] = {square(2), rectangle(3, 4), circle(5)};
  for (int i = 0; i < 3; i++) {
    print_shape(shapes[i]);
  }
}

