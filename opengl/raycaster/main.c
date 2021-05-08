#include <stdio.h>
#include <math.h>

#include <GL/gl.h>
#include <GL/glu.h>
#include <GL/glut.h>

#define PI 3.14159265359

const int window_width = 1280;
const int window_height = 720;
const int map_width = 256;
const int map_height = 256;

#define MAP_X 8
#define MAP_Y 8
#define N_RAYS 1280

const int TILE_SIZE = 64;

float rays_x[N_RAYS];
float rays_y[N_RAYS];
float rays_color[N_RAYS];

int frame = 0;
const int frame_rate = 60;
int keys[256] = {0};

const int map[MAP_Y][MAP_X] = {
    {1, 1, 1, 1, 1, 1, 1, 1},
    {1, 0, 0, 0, 0, 0, 0, 1},
    {1, 0, 1, 0, 1, 0, 0, 1},
    {1, 0, 1, 0, 1, 1, 1, 1},
    {1, 0, 0, 0, 0, 0, 0, 1},
    {1, 0, 0, 0, 0, 0, 1, 1},
    {1, 0, 0, 0, 0, 0, 0, 1},
    {1, 1, 1, 1, 1, 1, 1, 1}
};

typedef struct {
    float x;
    float y;
    float dx;
    float dy;
    float angle;
    float angle_2;
} Player;

Player player = {300, 300, 0, 0, 0};

void player_change_direction (float angle, float angle_2) {
    player.angle += angle;
    player.angle_2 += angle_2;
    player.dx = cos (player.angle);
    player.dy = sin (player.angle);
}

void player_move (float value, int tangential, int perpendicular) {
    player.x += value * perpendicular * player.dy;
    player.y -= value * perpendicular * player.dx;
    player.x += value * tangential * player.dx;
    player.y += value * tangential * player.dy;
}

void handler_mouse_motion (int x, int y) {
    static int last_x = window_width / 2;
    static int last_y = window_height / 2;

    if (x < 100 || x > window_width - 100 || y < 100 || y > window_height - 100) {
        glutWarpPointer (window_width / 2, window_height / 2);
        last_x = window_width / 2;
        last_y = window_height / 2;
        return;
    }

    player_change_direction (0.001 * (x - last_x), 0.001 * (y - last_y));
    last_x = x;
    last_y = y;
}

void handler_key_press (unsigned char key, int x, int y) {
    keys[key] = 1;
}

void handler_key_release (unsigned char key, int x, int y) {
    keys[key] = 0;
}

void draw_map () {
    const int offset = 32;
    const int tile_size_x = map_width / MAP_X;
    const int tile_size_y = map_height / MAP_Y;

    glColor3f (0.9, 0.9, 0.9);
    glBegin (GL_QUADS);
    glVertex2i (offset, offset);
    glVertex2i (offset + map_width, offset);
    glVertex2i (offset + map_width, offset + map_height);
    glVertex2i (offset, offset + map_height);
    glEnd ();

    int cube_size_x = tile_size_x - 2;
    int cube_size_y = tile_size_y - 2;
    for (int y = 0; y < MAP_Y; y++) {
        for (int x = 0; x < MAP_X; x++) {
            if (map[y][x]) {
                int x0 = offset + x * tile_size_x + 1;
                int y0 = offset + y * tile_size_y + 1;
                glColor3f (0.6f, 0.6f, 0.6f);
                glBegin (GL_QUADS);
                glVertex2i (x0, y0);
                glVertex2i (x0 + cube_size_x, y0);
                glVertex2i (x0 + cube_size_x, y0 + cube_size_y);
                glVertex2i (x0, y0 + cube_size_y);
                glEnd ();
            }
        }
    }

    glColor3f (1.0f, 1.0f, 1.0f);
    glLineWidth (1);
    for (int ray = 0; ray < N_RAYS; ray++) {
        float ray_x = rays_x[ray];
        float ray_y = rays_y[ray];
        glBegin (GL_LINES);
        glVertex2i (player.x / 2 + offset, player.y / 2 + offset);
        glVertex2i ((player.x + ray_x) / 2 + offset, (player.y + ray_y) / 2 + offset);
        glEnd ();
    }

    glColor3f (0.4f, 0.4f, 0.4f);
    glPointSize (8);
    glBegin (GL_POINTS);
    glVertex2i (player.x / 2 + offset , player.y / 2 + offset);
    glEnd ();

    glLineWidth (3);
    glBegin (GL_LINES);
    glVertex2i (player.x / 2 + offset, player.y / 2 + offset);
    glVertex2i ((player.x + 20 * player.dx) / 2 + offset , (player.y + 20 * player.dy) / 2 + offset);
    glEnd ();
}

void *calc_rays () {
    for (int ray = 0; ray < N_RAYS; ray++) {
        float angle = player.angle + 0.6 * (2 * ray + 1 - N_RAYS) / (float) N_RAYS;
        float ray_cos = cos (angle);
        float ray_sin = sin (angle);
        float ray_tan = tan (angle);
        // printf ("angle %f sin %f\n", angle, ray_sin);
        float ray_v_y = ((((int)player.y + 64 * (ray_sin > 0)) >> 6) << 6) - player.y;
        float ray_v_x = ray_v_y / ray_tan;
        float ray_h_x = ((((int)player.x + 64 * (ray_cos > 0)) >> 6) << 6) - player.x;
        float ray_h_y = ray_h_x * ray_tan;

        int hit_v = 0;
        int hit_h = 0;
        for (int dof = 0; dof < 8; dof++) {
            int map_v_x = (int) (player.x + ray_v_x) >> 6;
            int map_v_y = ((int) (player.y + ray_v_y) >> 6) - (ray_sin < 0);
            if (map_v_x >= 0 && map_v_x < 64 && map_v_y >= 0 && map_v_y < 64) {
                if (map[map_v_y][map_v_x]){
                    hit_v = 1;
                    break;
                }
            }
            int sign_v = 2 * (ray_sin > 0) - 1;
            ray_v_x += sign_v * 64 / ray_tan;
            ray_v_y += sign_v * 64;
        }

        for (int dof = 0; dof < 8; dof++) {
            int map_h_x = ((int) (player.x + ray_h_x) >> 6) - (ray_cos < 0);
            int map_h_y = (int) (player.y + ray_h_y) >> 6;
            if (map_h_x >= 0 && map_h_x < 64 && map_h_y >= 0 && map_h_y < 64) {
                if (map[map_h_y][map_h_x]){
                    hit_h = 1;
                    break;
                }
            }
            int sign_h = 2 * (ray_cos > 0) - 1;
            ray_h_x += sign_h * 64;
            ray_h_y += sign_h * 64 * ray_tan;
        }


        int ray_v = ray_v_x * ray_v_x + ray_v_y * ray_v_y;
        int ray_h = ray_h_x * ray_h_x + ray_h_y * ray_h_y;
        float color;
        if (hit_v && (!hit_h || ray_v < ray_h)) {
            rays_x[ray] = ray_v_x;
            rays_y[ray] = ray_v_y;
            rays_color[ray] = 0.95;
        } else if (hit_h) {
            rays_x[ray] = ray_h_x;
            rays_y[ray] = ray_h_y;
            rays_color[ray] = 0.85;
        } else {
            rays_x[ray] = player.x + 10000 * ray_cos;
            rays_y[ray] = player.y + 10000 * ray_sin;
            rays_color[ray] = 0.0;
        }
    }
}

void draw_rays () {
    for (int ray = 0; ray < N_RAYS; ray++) {
        float ray_x = rays_x[ray];
        float ray_y = rays_y[ray];
        float color = rays_color[ray];
        float distance = sqrt(ray_x * ray_x + ray_y * ray_y) * cosf (atan2f (ray_y, ray_x) - player.angle);
        int width = window_width / N_RAYS;
        int height = 50 * window_height / distance;
        int x0 = ray * width;
        int y0 = (window_height - height) / 2 - player.angle_2 * (1200 + distance);
        glColor3f (color, color, color);
        glBegin (GL_QUADS);
        glVertex2i (x0, y0);
        glVertex2i (x0 + width, y0);
        glVertex2i (x0 + width, y0 + height);
        glVertex2i (x0, y0 + height);
        glEnd ();
        glColor3f (0.3, 0.3, 0.3);
        glBegin (GL_QUADS);
        glVertex2i (x0, y0 + height);
        glVertex2i (x0 + width, y0 + height);
        glVertex2i (x0 + width, window_height);
        glVertex2i (x0, window_height);
        glEnd ();
    }
}

void display () {
    glClear (GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    calc_rays ();
    draw_rays ();
    draw_map ();
    glutSwapBuffers ();
}

void timer (int value) {
    frame += 1;
    if (frame % 60 == 0) {
        printf ("Current frame: %d\n", frame);
    }
    glutPostRedisplay ();
    player_move (1.0f , keys['w'] - keys['s'], keys['a'] - keys['d']);
    glutTimerFunc (1000 / frame_rate, timer, 0);
}

void init () {
    glutSetCursor (GLUT_CURSOR_NONE);
    glutWarpPointer (window_width / 2, window_height / 2);
    glClearColor (0.816, 0.922, 1.00, 0);
    gluOrtho2D (0, window_width, window_height, 0);
    player_change_direction (0.0f, 0.0f);
}

int main (int argc, char *argv[]) {
    glutInit (&argc, argv);
    glutInitDisplayMode (GLUT_DOUBLE | GLUT_RGBA);
    glutInitWindowSize (window_width, window_height);
    glutCreateWindow ("Raycasting");
    init ();
    glutDisplayFunc (display);
    glutKeyboardFunc (handler_key_press);
    glutKeyboardUpFunc (handler_key_release);

    glutMotionFunc (handler_mouse_motion);
    glutPassiveMotionFunc (handler_mouse_motion);
    glutTimerFunc (1000 / frame_rate, timer, 0);
    glutMainLoop ();
    return 0;
}
