const std = @import("std");
const print = @import("std").debug.print;

const COLS = 128;
const ROWS = 32;

pub fn main() anyerror!void {
    var current: [][COLS]u8 = try std.heap.page_allocator.alloc([COLS]u8, ROWS);
    var next: [][COLS]u8 = try std.heap.page_allocator.alloc([COLS]u8, ROWS);
    defer std.heap.page_allocator.free(current);
    defer std.heap.page_allocator.free(next);

    set_zero(current);
    set_zero(next);
    current[0][1] = 1;
    current[0][2] = 1;
    current[1][0] = 1;
    current[1][1] = 1;
    current[2][1] = 1;

    while (true) {
        print_field(current);
        step(current, next);
        var tmp = next;
        next = current;
        current = tmp;
        std.time.sleep(100000000);
    }
}

pub fn set_zero(field: [][COLS]u8) void {
    for (field) |*row| {
        for (row) |*value| {
            value.* = 0;
        }
    }
}

pub fn step(current: [][COLS]u8, next: [][COLS]u8) void {
    for (current) |row, i| {
        for (row) |cell, j| {
            const alive_neighbours
                = current[i][(j + 1) % COLS]
                + current[i][(j + COLS - 1) % COLS]
                + current[(i + 1) % ROWS][j]
                + current[(i + ROWS - 1) % ROWS][j]
                + current[(i + 1) % ROWS][(j + 1) % COLS]
                + current[(i + 1) % ROWS][(j + COLS - 1) % COLS]
                + current[(i + ROWS - 1) % ROWS][(j + 1) % COLS]
                + current[(i + ROWS - 1) % ROWS][(j + COLS - 1) % COLS];
            next[i][j] = @boolToInt(alive_neighbours == 3 or (cell == 1 and alive_neighbours == 2));
        }
    }
}

pub fn print_field(field: [][COLS]u8) void {
    print("\x1Bc", .{});
    for (field) |row| {
        for (row) |value| {
            const symbol: u8 = if (value == 0) '.' else '@';
            print("{c}", .{symbol});
        }
        print("\n", .{});
    }
}
