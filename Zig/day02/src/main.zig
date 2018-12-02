const std = @import("std");
const mem = std.mem;
const map = std.hash_map;
const heap = std.heap;

const input = @embedFile("../input.txt");

fn solve() !u32 {
    var twos: u32 = 0;
    var threes: u32 = 0;

    var allocator = heap.DirectAllocator.init();
    defer allocator.deinit();

    var frequencies = map.AutoHashMap(u8, u32).init(&allocator.allocator);
    defer frequencies.deinit();
    
    var splitter = mem.split(input, "\n");

    while(splitter.next()) |line| {
        for(line) |c| {
            var entry = try frequencies.getOrPut(c);
            
            if (entry.found_existing) {
                entry.kv.value += 1;
            } else {
                entry.kv.value = 1;
            }
        }
        
        var two_and_three = [2]u32 {0, 0};

        var frequency_iter = frequencies.iterator();
        while(frequency_iter.next()) |entry| {
            if (entry.value == 2 and two_and_three[0] == 0) { two_and_three[0] += 1; }
            if (entry.value == 3 and two_and_three[1] == 0) { two_and_three[1] += 1; }
        }
        
        twos += two_and_three[0];
        threes += two_and_three[1];
        
        frequencies.clear();
    }

    return twos * threes;
}

pub fn main() anyerror!void {
    const answer = try solve();

    std.debug.warn("part 1: {}\n", answer);
}
