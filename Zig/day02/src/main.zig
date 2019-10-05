const std = @import("std");
const mem = std.mem;
const input = @embedFile("../input.txt");

const builtin = @import("builtin");
const Int = builtin.TypeInfo.Int;

fn int_type(comptime is_signed: bool, comptime bits: usize) type {
    return @Type(
        builtin.TypeInfo{
            .Int = Int {
                .is_signed = is_signed,
                .bits = bits
            }
        });
}

/// Creates a bitvec that can hold `SIZE` elements of `WIDTH` bits.
fn BitVec(comptime SIZE: usize, comptime WIDTH: usize) type {
    const VecType = int_type(false, SIZE * WIDTH);
    const ElementType = int_type(false, WIDTH);
    const ShiftType = int_type(false, math.ceil(math.log2(f32(SIZE * WIDTH))));
    const Mask: VecType = math.maxInt(VecType) >> ((SIZE * WIDTH) - WIDTH);
    
    return struct {
        vec: VecType = 0,
        const BVec = @This();

        pub fn new() BVec {

            return BVec { };
        }

        pub fn index(self: *BVec, idx: u8) Entry {
            var element = @intCast(ElementType, (self.vec >> @intCast(ShiftType, idx * WIDTH)) & Mask ); 
            return Entry {
                .bitvec = self,
                .index = idx,
                .element = element
             };
        }

        pub fn iter(self: *const BVec) Iter {
            return Iter {
                .bitvec = self,
                .index = 0,
            };
        }

        const Entry = struct {
            bitvec: *BitVec(SIZE, WIDTH),
            index: u8,
            element: ElementType,

            fn deinit(self: Entry) void {
                var m = ~(Mask << @intCast(ShiftType, (self.index * WIDTH)));
                self.bitvec.vec &= m;

                var n: VecType = self.element;
                var shiftby: ShiftType = @intCast(ShiftType, (self.index * WIDTH));
                var shifted = n << shiftby;
                self.bitvec.vec = shifted | self.bitvec.vec;
            }
        };

        const Iter = struct {
            bitvec: *const BitVec(SIZE, WIDTH),
            index: u8,

            fn next(self: *Iter) ?ElementType {
                if (self.index >= SIZE) { return null; }

                var n = (self.bitvec.vec >> @intCast(ShiftType, self.index * WIDTH)) & Mask;
                self.index += 1;
                return @truncate(ElementType, n);
            }
        };
    };
}


const math = std.math;
pub fn main() void {

    var twos = usize(0);
    var threes = usize(0); 

    var splitter = comptime mem.separate(input, "\n");
    while(splitter.next()) |line| {

        var current = usize(0);
        var b = BitVec(26, 3).new();

        for(line) |c| {
            if (c == '\n' or c == '\r') { continue; }
            var e = b.index(c - 97);
            defer e.deinit();
            e.element = math.add(u3, e.element, 1) catch e.element;
        }

        var iter = b.iter();
        while(iter.next()) |value| {
            var v = value -% 2;
            if (v > 1) { continue; }
            current = (usize(1) << (v)) | current;
        }

        twos += (current & 0b01);
        threes += ((current >> 1) & 0b01);
    }
    std.debug.warn("{}", twos * threes);
}