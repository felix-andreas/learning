const std = @import("std");
const expect = std.testing.expect;

pub fn main() void {
    std.debug.print("token: {}\n", .{TokenType.LParen});

    const file = std.fs.cwd().openFile("example.txt", .{}) catch |err| {
        std.debug.print("Error opening file: {}\n", .{err});
        return;
    };
    defer file.close();

    var buffer: [100]u8 = undefined;
    const size = file.readAll(&buffer) catch |err| {
        std.debug.print("Error reading file: {}\n", .{err});
        return;
    };

    const content = buffer[0..size];
    std.debug.print("File content: {s}\n", .{content});
}

const TokenType = enum {
    Identifier,
    LParen,
    RParen,
    Op,
};

const Operation = enum {
    Add,
    Subtract,
    Multiply,
    Divide,
};

const Token = union(TokenType) {
    Identifier: []const u8,
    LParen: void,
    RParen: void,
    Op: Operation,
};

fn addSmallInts(comptime T: type, a: T, b: T) T {
    return switch (@typeInfo(T)) {
        .ComptimeInt => a + b,
        .Int => |info| if (info.bits <= 16)
            a + b
        else
            @compileError("ints too large"),
        else => @compileError("only ints accepted"),
    };
}

test "typeinfo switch" {
    const x = addSmallInts(u16, 20, 30);
    try expect(@TypeOf(x) == u16);
    try expect(x == 50);
}
