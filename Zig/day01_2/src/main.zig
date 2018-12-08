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
    
    var arena_alloc = heap.ArenaAllocator.init(&allocator.allocator);
    defer arena_alloc.deinit();
    
    var parsed = Vec(N).init(&arena_alloc.allocator);
    defer parsed.deinit();

    comptime var splitter = comptime mem.split(input, "\n");
    
    inline while(comptime splitter.next()) |slice| {
        const num = comptime try fmt.parseInt(N, slice, 10);
        try parsed.append(num);
    }

    var set = map.AutoHashMap(N, void).init(&arena_alloc.allocator);
    defer set.deinit();
 
    while (true) {

        for(parsed.toSliceConst()) |*num| {
            frequency += num.*;
            const entry = try set.getOrPut(frequency);
            if (entry.found_existing) {
                return entry.kv.key;
            }
        }
    }
}

pub fn main() !void {
    @setEvalBranchQuota(500000);
    const answer = try solve(i32);

    std.debug.warn("part 2: {}\n", answer);    
}
