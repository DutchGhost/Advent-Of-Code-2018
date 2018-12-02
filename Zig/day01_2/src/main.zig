const std = @import("std");
const mem = std.mem;
const fmt = std.fmt;
const map = std.hash_map;
const heap = std.heap;
const Vec = std.ArrayList;

const input = @embedFile("../input.txt");

fn solve(comptime N: type) !N {
    var frequency = N(0);

    var allocator = heap.DirectAllocator.init();
    defer allocator.deinit();
    
    var parsed = Vec(N).init(&allocator.allocator);
    defer parsed.deinit();

    var splitter = mem.split(input, "\n");
    
    while(splitter.next()) |slice| {
        var num = try fmt.parseInt(N, slice, 10);
        try parsed.append(num);
    }

    var set = map.AutoHashMap(N, void).init(&allocator.allocator);
    defer set.deinit();
 
    while (true) {

        for(parsed.toSlice()) |num| {
            frequency += num;
            const entry = try set.getOrPut(frequency);
            if (entry.found_existing) {
                return entry.kv.key;
            }
        }
    }
}

pub fn main() !void {
    const answer = try solve(i32);

    std.debug.warn("part 2: {}\n", answer);    
}
