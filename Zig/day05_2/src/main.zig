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

fn react(slice: []const u8, stack: *Vec(u8), filter_char: u8) !usize {
    for (input[0..input.len - 1]) |c| {
        if (c != filter_char and c != filter_char + 'a' - 'A') {
            if (stack.len > 0 and stack.at(stack.len - 1) == complement(c)) {
                _ = stack.pop();
            }
            else {
                try stack.append(c);
            }
        }
    }

    return stack.len;
}

pub fn main() !void {
    var allocator = heap.DirectAllocator.init();
    defer allocator.deinit();

    var stack = Vec(u8).init(&allocator.allocator);
    defer stack.deinit();
    
    var len: usize = std.math.maxInt(usize);

    var filter_char: u8 = 65;

    while(filter_char <= 90) : (filter_char += 1) {
        debug.warn("c {} {c}\n", filter_char, filter_char);
        var reacted_len = try react(input[0..input.len - 1], &stack, filter_char);
        if (reacted_len < len) {
            len = reacted_len;
        }
        
        // this clears the Vec, keeping its capacity.
        stack.len = 0;
    }
    debug.warn("The remaining shortest length = {}\n", len);
}
