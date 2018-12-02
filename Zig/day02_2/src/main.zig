const std = @import("std");
const mem = std.mem;
const map = std.hash_map;
const heap = std.heap;
const Vec = std.ArrayList;

const input = @embedFile("../input.txt");

const IDMatcher = struct {
    s1: []const u8,
    s2: []const u8,
};

const FindError = error {
    MatchNotFound,
};

fn is_match(box1: []const u8, box2: []const u8) ?IDMatcher {
    var count_equals: usize = 0;
    var count_equals_tail: usize = 0;

    var slice_index: usize = 0;

    while (slice_index != box1.len): ({slice_index += 1; count_equals += 1;}) {
        if (box1[slice_index] != box2[slice_index]) {
            break;
        }
    }
            
    slice_index += 1;

     while (slice_index != box1.len): ({slice_index += 1; count_equals_tail += 1;}) {
        if (box1[slice_index] != box2[slice_index]) {
            break;
        }
    }
            
    if (count_equals + count_equals_tail == box1.len - 1) {
        return IDMatcher { .s1 = box1[0..count_equals], .s2 = box1[count_equals + 1..] };
    }

    return null; 
}

fn solve() !IDMatcher {

    var allocator = heap.DirectAllocator.init();
    defer allocator.deinit();

    var boxes = Vec([] const u8).init(&allocator.allocator);
    defer boxes.deinit();
    
    var splitter = mem.split(input, "\n");

    while(splitter.next()) |line| {
        try boxes.append(line);
    }

    var boxes_slice = boxes.toSlice();

    for(boxes_slice) |box1, idx| {
        for(boxes_slice[idx + 1..]) |box2| {
            if (is_match(box1, box2)) |matched| {
                return matched;
            }
        }
    }

    return FindError.MatchNotFound;
}

pub fn main() !void {
    const answer = try solve();

    std.debug.warn("part 2: {}{}\n", answer.s1, answer.s2);
}
