#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int x;
    int y;
} Player;

Player *Player__create(int x, int y) {
    Player *player = malloc(sizeof(Player));
    player->x = x;
    player->y = y;
    return player;
}

void Player__move(Player *player, int dx, int dy) {
    player->x += dx;
    player->y += dy;
    printf("Move Player to position (%d, %d)\n", player->x, player->y);
}

typedef struct {
    int x;
    int y;
} Point;

int main(void) {
    Point point = {1, 1};
    Player *player = Player__create(0, 0);
    Player__move(player, point.x, point.x);
}