use aoc::aoc;
use core::marker::PhantomData;

trait BitWidth {
    const WIDTH: u8;
}

impl BitWidth for [(); 4] {
    const WIDTH: u8 = 4;
}

struct BitArray<W>(u128, PhantomData<W>);

impl<W> BitArray<W> {
    pub const fn new() -> Self {
        Self(0, PhantomData)
    }
}

/// (self.0 >> idx * WIDTH) & MASK
/// self.0 = (new_value << idx * WIDTH) | x
impl<W: BitWidth> BitArray<W> {
    /// First call:
    ///     self.0 = 10100110rrrr1110, where rrrr = 0000,
    ///     mask = 0000000011110000
    ///
    ///     10100110 0000 1110
    ///     00000000 1111 0000
    ///     =================== &
    ///     00000000 0000 0000
    ///     
    ///     Entry { num_at_idx: 0000 }
    ///     *entry += 1 => num_at_idx: 0001
    ///
    /// Second call:
    ///     self.0 = 10100110rrrr1110, where rrrr = 0001,
    ///     mask = 0000000011110000
    ///
    ///     10100110 0001 1110
    ///     00000000 1111 0000
    ///     ==================== &
    ///     00000000 0001 0000
    ///
    ///     Entry { num_at_idx: 0001 },
    ///     *entry += 1 => num_at_idx: 0011
    fn entry(&mut self, idx: char) -> Entry<W> {
        let idx = idx as u8 - 97;
        let mask: u128 = 0b1111 << (idx * W::WIDTH);
        let num_at_idx = (self.0 & mask) >> (idx * W::WIDTH);
        Entry {
            bitarray: self,
            idx,
            num: num_at_idx as u8,
        }
    }
   
    pub fn iter(&self) -> BitIter<W> {
        BitIter::new(self)
    }
}

struct Entry<'a, W: BitWidth> {
    bitarray: &'a mut BitArray<W>,
    idx: u8,
    num: u8,
}

impl<W: BitWidth> Drop for Entry<'_, W> {
    fn drop(&mut self) {
        let num = (self.num as u128) << (self.idx * W::WIDTH);
        self.bitarray.0 = self.bitarray.0 | num;
    }
}

impl<W: BitWidth> core::ops::Deref for Entry<'_, W> {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.num
    }
}

impl<W: BitWidth> core::ops::DerefMut for Entry<'_, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.num
    }
}

struct BitIter<'a, W> {
    bitarray: &'a BitArray<W>,
    index: u8,
}

impl<'a, W> BitIter<'a, W> {
    const fn new(bitarray: &'a BitArray<W>) -> Self {
        Self { bitarray, index: 0 }
    }
}

impl<W: BitWidth> Iterator for BitIter<'_, W> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 128 {
            None
        } else {
            let mask = 0b1111 << (self.index);
            let ret = (self.bitarray.0 & mask) >> (self.index);
            self.index += W::WIDTH;
            Some(ret.count_ones() as u8)
        }
    }
}

#[aoc(2018, 2, 1)]
fn main(input: &str) -> usize {
    let mut twos = 0;
    let mut threes = 0;

    let mut array = BitArray::<[(); 4]>::new();

    let mut current: u8 = 0;

    for line in input.lines() {
        for c in line.chars() {
            *array.entry(c) += 1;
        }

        for freq in array.iter().filter(|n| *n == 2 || *n == 3).map(|n| n - 2) {
            current = (1 << freq) | current;
        }

        twos += (current & 0b01) as usize;
        threes += ((current >> 1) & 0b01) as usize;

        current = 0;
        array = BitArray::new();
    }
    twos * threes
}
