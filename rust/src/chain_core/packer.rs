//! Tooling for packing and unpacking from streams
//!
//! This will allow us to expose some standard way of serializing
//! data.

const INITIAL_BUFFERED_CAPACITY: usize = 2048;
use crate::*;
use alloc::{vec, vec::Vec};
use core::{marker, ops};
use core2::io::{BufRead, Read, Result, Write};

pub struct Codec<I>(I);
impl<I> Codec<I> {
    pub fn new(inner: I) -> Self {
        Codec(inner)
    }

    pub fn into_inner(self) -> I {
        self.0
    }
}

pub struct Buffered<I: Write>(I, Codec<Vec<u8>>);

pub struct Hole<T> {
    _marker: marker::PhantomData<T>,
    start: usize,
    end: usize,
}

impl<R: BufRead> Codec<R> {
    #[inline]
    pub fn get_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.0.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    #[inline]
    pub fn get_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.0.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
    #[inline]
    pub fn get_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.0.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }
    #[inline]
    pub fn get_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.0.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
    #[inline]
    pub fn get_u128(&mut self) -> Result<u128> {
        let mut buf = [0u8; 16];
        self.0.read_exact(&mut buf)?;
        Ok(u128::from_be_bytes(buf))
    }
    #[inline]
    pub fn get_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; n];
        self.0.read_exact(&mut buf)?;
        Ok(buf)
    }
}
impl<W: Write> Codec<W> {
    #[inline]
    pub fn buffered(self) -> Buffered<W> {
        Buffered(self.0, Codec(Vec::with_capacity(INITIAL_BUFFERED_CAPACITY)))
    }

    #[inline]
    pub fn put_u8(&mut self, v: u8) -> Result<()> {
        self.0.write_all(&[v])
    }
    #[inline]
    pub fn put_u16(&mut self, v: u16) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())
    }
    #[inline]
    pub fn put_u32(&mut self, v: u32) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())
    }
    #[inline]
    pub fn put_u64(&mut self, v: u64) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())
    }
    #[inline]
    pub fn put_u128(&mut self, v: u128) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())
    }
}
impl<W: Write> Buffered<W> {
    #[inline]
    pub fn hole<T>(&mut self, len: usize) -> Result<Hole<T>> {
        use Write;
        let start = (self.1).0.len();
        let end = start + len;
        let buf = vec![0; len];
        self.write_all(&buf)?;
        Ok(Hole {
            _marker: marker::PhantomData,
            start: start,
            end: end,
        })
    }

    #[inline]
    pub fn into_inner(self) -> Result<Codec<W>> {
        let mut codec = Codec(self.0);
        let buffer = (self.1).0;
        codec.0.write_all(&buffer)?;
        Ok(codec)
    }

    #[inline]
    pub fn fill_hole_u8(&mut self, hole: Hole<u8>, value: u8) {
        (self.1).0[hole.start..hole.end].copy_from_slice(&[value])
    }
    #[inline]
    pub fn fill_hole_u16(&mut self, hole: Hole<u16>, value: u16) {
        (self.1).0[hole.start..hole.end].copy_from_slice(&value.to_be_bytes())
    }
    #[inline]
    pub fn fill_hole_u32(&mut self, hole: Hole<u32>, value: u32) {
        (self.1).0[hole.start..hole.end].copy_from_slice(&value.to_be_bytes())
    }
    #[inline]
    pub fn fill_hole_u64(&mut self, hole: Hole<u64>, value: u64) {
        (self.1).0[hole.start..hole.end].copy_from_slice(&value.to_be_bytes())
    }
    #[inline]
    pub fn fill_hole_u128(&mut self, hole: Hole<u128>, value: u128) {
        (self.1).0[hole.start..hole.end].copy_from_slice(&value.to_be_bytes())
    }

    #[inline]
    pub fn buffered_len(&self) -> usize {
        (self.1).0.len()
    }
}

impl<R: Read> Read for Codec<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}
impl<BR: BufRead> BufRead for Codec<BR> {
    #[inline]
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.0.fill_buf()
    }
    #[inline]
    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}
impl<W: Write> Write for Codec<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }
    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
impl<W: Write> Write for Buffered<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.1.write(buf)
    }
    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.1.flush()
    }
}
impl<I: Write> ops::Deref for Buffered<I> {
    type Target = Codec<Vec<u8>>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}
impl<I: Write> ops::DerefMut for Buffered<I> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
