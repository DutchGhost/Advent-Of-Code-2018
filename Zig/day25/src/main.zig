const std = @import("std");
const Vec = std.ArrayList;
const fmt = std.fmt;
const mem = std.mem;
const heap = std.heap;

const input = @embedFile("../input.txt");

fn abs(n: i32) i32 {
    if (n < 0) { return n * -1; } else { return n; }
}

const Point = struct {
    const Self = @This();

    x: i32,
    y: i32,
    z: i32,
    q: i32,

    fn manhatten(self: *const Self, other: *const Self) i32 {
        return  abs(self.x + other.x) +
                abs(self.y + other.y) +
                abs(self.z + other.z) +
                abs(self.q + other.q);
    }
};

fn parse(bytes: []const u8, buffer: *Vec(Vec(Point)), alloc: *mem.Allocator) !void {
    var splitter = mem.split(bytes, "\n");
    
    while(splitter.next()) |line| {
        var line_splitter = mem.split(line, ",");
        
        var x = try fmt.parseInt(i32, line_splitter.next().?, 10);
        var y = try fmt.parseInt(i32, line_splitter.next().?, 10);
        var z = try fmt.parseInt(i32, line_splitter.next().?, 10);
        var q = try fmt.parseInt(i32, line_splitter.next().?, 10);

        var point = Point {.x = x, .y = y, .z = z, .q = q};

        var constullation = Vec(Point).init(alloc);
        try constullation.append(point); 
        try buffer.append(constullation);
    }
}

fn find(idx: usize, point: Point, constellations: []Vec(Point)) !void {
    for(constellations) |*constellation| {
        for(constellation.toSlice()) |p| {
            std.debug.warn("{} : {}\n", point, p);
            if (point.manhatten(&p) <= 3) {
                try constellations[idx].appendSlice(constellation.toSlice());
                _ = constellation.shrink(0);
                break;
            }
        }
    }
}
pub fn main() !void {
    var allocator = heap.DirectAllocator.init();
    defer allocator.deinit();

    var buffer = Vec(Vec(Point)).init(&allocator.allocator);
    defer {
        for(buffer.toSlice()) |constellation| {
            constellation.deinit();
        }
        buffer.deinit();
    }

    try parse(input, &buffer, &allocator.allocator);

    var slice = buffer.toSlice();
    std.debug.warn("slice len = {}\n", slice.len);
    for(slice) |constellation, idx| {
        for(constellation.toSlice()) |point| {
            //try find(idx, point, slice[0..idx]);
            for(slice[0..idx]) |*constellation2| {
                for(constellation2.toSlice()) |p| {
                    std.debug.warn("{} : {}\n", point, p);
                    if (point.manhatten(&p) <= 3) {
                    try slice[idx].appendSlice(constellation2.toSlice());
                        _ = constellation2.shrink(0);
                        break;
                    }
                }
            }

        }
    }
    
    var total: usize = 0;
    for(slice) |constellation| {
        if(constellation.len != 0) {
            total += 1;
        }
    }

    std.debug.warn("Total = {}\n", total);
}
