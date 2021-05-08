#include <stdio.h>

class Player {
   public:
    int x;
    int y;
    void move(int dx, int dy) {
        x += dx;
        y += dy;
        printf("Move Player to position (%d, %d)\n", x, y);
    }

   private:
    int z;
};

typedef struct {
    int x;
    int y;
} Point;

int main(void) {
    Player player;
    player.x = 0;
    player.y = 0;
    Point point = {1, 1};
    player.move(point.x, point.y);
}