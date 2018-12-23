const std = @import("std");
const debug = std.debug;
const Vec = std.ArrayList;
const heap = std.heap;

const input = @embedFile("../input.txt");

fn complement(c: u8) u8 {
    switch (c) {
        65 ... 90 => {
            return c + 'a' - 'A';
        },

        91 ... 122 => {
            return c - ('a' - 'A');
        },

        else => unreachable,
    }
}

pub fn main() !void {
    var allocator = heap.DirectAllocator.init();    
    defer allocator.deinit();

    var stack = Vec(u8).init(&allocator.allocator);
    defer stack.deinit();

    for (input[0..input.len - 1]) |c| {
        if (stack.len > 0 and stack.at(stack.len - 1) == complement(c)) {
            _ = stack.pop();
        }
        else {
            try stack.append(c);
        }
    }

    debug.warn("The remaining length = {}\n", stack.len);
}
