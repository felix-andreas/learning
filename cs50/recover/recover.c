#include <stdio.h>
#include <stdlib.h>

int check_jpg(unsigned char buffer[]) {
    return buffer[0] == 0xff && buffer[1] == 0xd8 && buffer[2] == 0xff &&
           (buffer[3] & 0xf0) == 0xe0;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: recover <path>\n");
        return 1;
    }

    FILE *raw_file = fopen(argv[1], "r");
    if (raw_file == NULL) {
        fprintf(stderr, "Cannot read file: %s\n", argv[1]);
        return 1;
    }

    unsigned char buffer[512];
    for (int i = 0, done = 0; !done; i++) {
        printf("recover file: %d\n", i);
        char name[sizeof "xxx.jpg"];
        sprintf(name, "%03d.jpg", i);
        FILE *image_file = NULL;
        image_file = fopen(name, "w");
        if (image_file == NULL) {
            fprintf(stderr, "Cannot open file: %s\n", name);
            return 1;
        }

        while (!check_jpg(buffer)) {
            fread(buffer, sizeof buffer, 1, raw_file);
        }

        do {
            fwrite(buffer, sizeof buffer, 1, image_file);
            if (fread(buffer, sizeof buffer, 1, raw_file) != 1) {
                printf("End of file!\n");
                done = 1;
                break;
            };
        } while (!check_jpg(buffer));

        fclose(image_file);
    }

    fclose(raw_file);
    return 0;
}
