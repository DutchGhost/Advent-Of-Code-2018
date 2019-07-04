const std = @import("std");
const mem = std.mem;
const fmt = std.fmt;

const input = @embedFile("../input.txt");

fn solve(comptime N: type) N {
    var splitter = mem.separate(input, "\n");
    var sum = N(0);

    while(splitter.next()) |n| {
        var num = fmt.parseInt(N, n, 10) catch unreachable;
        sum += num;
    }

    return sum;
}

pub fn main() void {
    @setEvalBranchQuota(500000); 
    comptime var answer = comptime solve(i32);

    std.debug.warn("part 1: {}", answer);    
}
