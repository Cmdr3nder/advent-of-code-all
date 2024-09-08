use std::fs;

use anyhow::Result;

use crate::day::Day;

#[derive(Copy, Clone, PartialEq)]
enum Digit {
    One,
    Zero,
}

impl Digit {
    fn invert(&self) -> Self {
        match *self {
            Digit::One => Digit::Zero,
            Digit::Zero => Digit::One,
        }
    }

    fn from_char(ch: &char) -> Option<Self> {
        match ch {
            '1' => Some(Digit::One),
            '0' => Some(Digit::Zero),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Digit::One => '1',
            Digit::Zero => '0',
        }
    }
}

#[derive(Copy, Clone)]
enum DirectedChunk {
    Original,
    Reversed,
    Digit(Digit),
}

impl DirectedChunk {
    fn invert(&self) -> Self {
        match *self {
            DirectedChunk::Digit(d) => DirectedChunk::Digit(d.invert()),
            DirectedChunk::Original => DirectedChunk::Reversed,
            DirectedChunk::Reversed => DirectedChunk::Original,
        }
    }

    fn len(&self, source_len: usize) -> usize {
        match *self {
            DirectedChunk::Original | DirectedChunk::Reversed => source_len,
            DirectedChunk::Digit(_) => 1,
        }
    }
}

struct Disk<'a> {
    source: &'a [Digit],
    max_len: usize,
    contents: Vec<DirectedChunk>,
}

impl<'a> Disk<'a> {
    fn new(source: &'a [Digit], max_len: usize) -> Self {
        Disk {
            source,
            max_len,
            contents: vec![DirectedChunk::Original],
        }
    }

    fn len(&self) -> usize {
        let actual_len: usize = self
            .contents
            .iter()
            .map(|chunk| chunk.len(self.source.len()))
            .sum();
        actual_len.min(self.max_len)
    }

    fn fill(&mut self) -> &mut Self {
        while self.len() < self.max_len {
            let iter = (0..self.contents.len()).rev();
            self.contents.push(DirectedChunk::Digit(Digit::Zero));
            for i in iter {
                self.contents.push(self.contents[i].invert());
            }
        }
        self
    }

    fn checksum(&self) -> Vec<Digit> {
        let mut checksum = Vec::new();
        let mut disk_contents = self.into_iter();

        while let Some(a) = disk_contents.next() {
            match disk_contents.next() {
                Some(b) => checksum.push(if a == b { Digit::One } else { Digit::Zero }),
                None => break,
            }
        }

        if checksum.len() % 2 == 0 {
            Disk::new(&checksum, checksum.len()).checksum()
        } else {
            checksum
        }
    }
}

struct DiskIter<'a> {
    disk: &'a Disk<'a>,
    chunk_at: usize,
    source_at: usize,
}

impl<'a> DiskIter<'a> {
    fn disk_index(&self) -> usize {
        let mut disk_index = 0;
        for chunk_index in 0..self.chunk_at {
            disk_index += match self.disk.contents.get(chunk_index) {
                Some(c) => c.len(self.disk.source.len()),
                None => 0,
            };
        }
        disk_index += match self.disk.contents.get(self.chunk_at) {
            Some(DirectedChunk::Digit(_)) | None => 0,
            Some(DirectedChunk::Original) => self.source_at,
            Some(DirectedChunk::Reversed) => (self.disk.source.len() - 1) - self.source_at,
        };
        disk_index
    }
}

impl<'a> Iterator for DiskIter<'a> {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.disk_index() >= self.disk.max_len {
            None
        } else {
            match self.disk.contents.get(self.chunk_at) {
                None => None,
                Some(DirectedChunk::Digit(d)) => {
                    self.chunk_at += 1;
                    self.source_at = match self.disk.contents.get(self.chunk_at) {
                        None | Some(DirectedChunk::Original | DirectedChunk::Digit(_)) => 0,
                        Some(DirectedChunk::Reversed) => self.disk.source.len() - 1,
                    };
                    Some(*d)
                }
                Some(DirectedChunk::Original) => {
                    let snapshot_source_at = self.source_at;
                    if self.source_at == self.disk.source.len() - 1 {
                        self.chunk_at += 1;
                        self.source_at = match self.disk.contents.get(self.chunk_at) {
                            None | Some(DirectedChunk::Original | DirectedChunk::Digit(_)) => 0,
                            Some(DirectedChunk::Reversed) => self.disk.source.len() - 1,
                        };
                    } else {
                        self.source_at += 1;
                    }
                    Some(self.disk.source[snapshot_source_at])
                }
                Some(DirectedChunk::Reversed) => {
                    let snapshot_source_at = self.source_at;
                    if self.source_at == 0 {
                        self.chunk_at += 1;
                        self.source_at = match self.disk.contents.get(self.chunk_at) {
                            None | Some(DirectedChunk::Original | DirectedChunk::Digit(_)) => 0,
                            Some(DirectedChunk::Reversed) => self.disk.source.len() - 1,
                        };
                    } else {
                        self.source_at -= 1;
                    }
                    Some(self.disk.source[snapshot_source_at].invert())
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a Disk<'a> {
    type Item = Digit;
    type IntoIter = DiskIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        DiskIter {
            disk: self,
            chunk_at: 0,
            source_at: match self.contents.get(0) {
                None | Some(DirectedChunk::Original | DirectedChunk::Digit(_)) => 0,
                Some(DirectedChunk::Reversed) => self.source.len() - 1,
            },
        }
    }
}

pub struct Day16;

impl Day for Day16 {
    fn main() -> Result<()> {
        let input: Vec<Digit> = (fs::read_to_string("input/2016/day16.txt")?)
            .chars()
            .filter(|ch| *ch == '1' || *ch == '0')
            .filter_map(|ch| Digit::from_char(&ch))
            .collect();
        let disk1_checksum: String = Disk::new(&input, 272)
            .fill()
            .checksum()
            .iter()
            .map(|d| d.to_char())
            .collect();
        println!("Disk 1 Checksum: '{disk1_checksum}'");
        // TAKES TOO LONG TO COMPUTE PART 2, NEED TO RETHINK
        let disk2_checksum: String = Disk::new(&input, 35651584)
            .fill()
            .checksum()
            .iter()
            .map(|d| d.to_char())
            .collect();
        println!("Disk 2 Checksum: '{disk2_checksum}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_day_2016_16() {
        let source = vec![Digit::One];
        let single_step: String = Disk::new(&source, 3)
            .fill()
            .into_iter()
            .map(|d| d.to_char())
            .collect();
        assert_eq!("100", single_step);
    }

    #[test]
    fn example_0_day_2016_16() {
        let source = vec![Digit::Zero];
        let single_step: String = Disk::new(&source, 3)
            .fill()
            .into_iter()
            .map(|d| d.to_char())
            .collect();
        assert_eq!("001", single_step);
    }

    #[test]
    fn example_11111_day_2016_16() {
        let source = vec![Digit::One, Digit::One, Digit::One, Digit::One, Digit::One];
        let single_step: String = Disk::new(&source, 11)
            .fill()
            .into_iter()
            .map(|d| d.to_char())
            .collect();
        assert_eq!("11111000000", single_step);
    }

    #[test]
    fn example_111100001010_day_2016_16() {
        let source = vec![
            Digit::One,
            Digit::One,
            Digit::One,
            Digit::One,
            Digit::Zero,
            Digit::Zero,
            Digit::Zero,
            Digit::Zero,
            Digit::One,
            Digit::Zero,
            Digit::One,
            Digit::Zero,
        ];
        let single_step: String = Disk::new(&source, 25)
            .fill()
            .into_iter()
            .map(|d| d.to_char())
            .collect();
        assert_eq!("1111000010100101011110000", single_step);
    }
}
