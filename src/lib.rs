#![no_std]
extern crate alloc as alloc_crate;
use alloc_crate::{
    boxed::Box,
    vec,
    vec::Vec,
};
use core::{
    mem::{self, MaybeUninit},
    ops::{Deref, DerefMut},
    slice,
};

pub unsafe trait Alignment: Copy + Sized {}

fn align_up<T>(u: usize) -> usize {
    (u + mem::align_of::<T>() - 1) / mem::align_of::<T>()
}

unsafe impl Alignment for u8 {}
unsafe impl Alignment for u16 {}
unsafe impl Alignment for u32 {}
unsafe impl Alignment for u64 {}
unsafe impl Alignment for usize {}
unsafe impl Alignment for i8 {}
unsafe impl Alignment for i16 {}
unsafe impl Alignment for i32 {}
unsafe impl Alignment for i64 {}
unsafe impl Alignment for isize {}

struct AlignedBuffer<A: Alignment> {
    data: Box<[MaybeUninit<A>]>,
}

impl<A: Alignment> AlignedBuffer<A> {
    fn new(size_bytes: usize) -> AlignedBuffer<A> {
        let size = align_up::<A>(size_bytes);
        let data_vec = vec![MaybeUninit::uninit(); size];
        AlignedBuffer {
            data: data_vec.into_boxed_slice()
        }
    }

    #[inline]
    fn len_u8(&self) -> usize {
        self.data.len() * mem::size_of::<A>()
    }

    fn reserve(&mut self, additional: usize) {
        let mut this = Vec::new().into_boxed_slice();
        mem::swap(&mut self.data, &mut this);
        let mut vec = Vec::from(this);
        vec.resize(vec.len() + align_up::<A>(additional), MaybeUninit::uninit());
        self.data = vec.into_boxed_slice();
    }
}

impl<A: Alignment> Deref for AlignedBuffer<A> {
    type Target = [MaybeUninit<u8>];
    fn deref(&self) -> &[MaybeUninit<u8>] {
        let _ = self.data.as_ptr();
        unsafe {
            slice::from_raw_parts(
                self.data.as_ptr() as *const MaybeUninit<u8>,
                self.len_u8(),
            )
        }
    }
}

impl<A: Alignment> DerefMut for AlignedBuffer<A> {
    fn deref_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe {
            slice::from_raw_parts_mut(
                self.data.as_mut_ptr() as *mut MaybeUninit<u8>,
                self.len_u8(),
            )
        }
    }
}

pub struct HetBuf<A: Alignment> {
    data: AlignedBuffer<A>,
    head: usize,
}

impl<A: Alignment> HetBuf<A> {
    pub fn new() -> HetBuf<A> {
        HetBuf {
            data: AlignedBuffer::new(0),
            head: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> HetBuf<A> {
        HetBuf {
            data: AlignedBuffer::new(capacity),
            head: 0,
        }
    }

    pub fn push_item<T>(&mut self, item: T) -> &mut T {
        let item_ref = &mut self.alloc_slice::<T>(1)[0];
        unsafe {
            item_ref.as_mut_ptr().write(item);
            &mut *item_ref.as_mut_ptr()
        }
    }

    pub fn push_iter<T>(&mut self, iter: impl IntoIterator<Item=T>) -> &mut [T] {
        let mut iter = iter.into_iter();
        let (size_hint, _) = iter.size_hint();
        let initial_slice = self.alloc_slice::<T>(size_hint);

        let mut actual_len = 0;
        for (item_ref, item) in initial_slice.iter_mut().zip(&mut iter) {
            unsafe { item_ref.as_mut_ptr().write(item); }
            actual_len += 1;
        }

        let initial_slice_head = initial_slice.as_ptr() as usize - self.data.as_ptr() as usize;

        for item in iter {
            self.push_item(item);
            actual_len += 1;
        }

        unsafe {
            cast_slice_mut(&mut self.data[initial_slice_head..initial_slice_head + mem::size_of::<T>() * actual_len])
        }
    }

    pub fn as_slice(&self) -> &[MaybeUninit<u8>] {
        &*self.data
    }

    fn alloc_slice<T>(&mut self, len: usize) -> &mut [MaybeUninit<T>] {
        if self.data.len() == 0 {
            self.data.reserve(mem::size_of::<T>());
        }
        let align_offset = (unsafe{ self.data.as_ptr().offset(self.head as isize) } as *const MaybeUninit<u8>).align_offset(mem::align_of::<T>());
        let head_aligned = self.head + align_offset;
        let head_new = head_aligned + mem::size_of::<T>() * len;
        let current_len = self.data.len_u8();

        if head_new > current_len {
            self.data.reserve(head_new - current_len);
        }

        self.head = head_new;

        let raw_slice = &mut self.data[head_aligned..head_new];
        unsafe {
            cast_slice_mut::<_, MaybeUninit<T>>(raw_slice)
        }
    }
}

unsafe fn cast_slice_mut<T, U>(e: &mut [T]) -> &mut [U] {
    if mem::size_of_val(e) == 0 {
        slice::from_raw_parts_mut(e.as_mut_ptr() as *mut U, 0)
    } else {
        assert_eq!(mem::size_of_val(e) % mem::size_of::<U>(), 0);
        slice::from_raw_parts_mut(e.as_mut_ptr() as *mut U,
                                  mem::size_of_val(e) / mem::size_of::<U>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_item() {
        let mut buf = HetBuf::<u32>::new();
        assert_eq!(0x0F, *buf.push_item(0x0Fu8));
        assert_eq!(4, buf.as_slice().len());

        assert_eq!(0xF00F, *buf.push_item(0xF00Fu16));
        assert_eq!(4, buf.as_slice().len());

        assert_eq!(0xF0, *buf.push_item(0xF0u8));
        assert_eq!(8, buf.as_slice().len());

        assert_eq!(0x04030201, *buf.push_item(0x04030201u32));
        assert_eq!(12, buf.as_slice().len());

        unsafe {
            assert_eq!(0x0F, *buf.as_slice()[0].as_ptr());
            if cfg!(target_endian = "little") {
                assert_eq!(0x0F, *buf.as_slice()[2].as_ptr());
                assert_eq!(0xF0, *buf.as_slice()[3].as_ptr());
            } else if cfg!(target_endian = "big") {
                assert_eq!(0xF0, *buf.as_slice()[2].as_ptr());
                assert_eq!(0x0F, *buf.as_slice()[3].as_ptr());
            } else {
                unreachable!()
            }
            assert_eq!(0xF0, *buf.as_slice()[4].as_ptr());
            if cfg!(target_endian = "little") {
                assert_eq!(0x01, *buf.as_slice()[8].as_ptr());
                assert_eq!(0x02, *buf.as_slice()[9].as_ptr());
                assert_eq!(0x03, *buf.as_slice()[10].as_ptr());
                assert_eq!(0x04, *buf.as_slice()[11].as_ptr());
            } else if cfg!(target_endian = "big") {
                assert_eq!(0x04, *buf.as_slice()[8].as_ptr());
                assert_eq!(0x03, *buf.as_slice()[9].as_ptr());
                assert_eq!(0x02, *buf.as_slice()[10].as_ptr());
                assert_eq!(0x01, *buf.as_slice()[11].as_ptr());
            } else {
                unreachable!()
            }
        }
    }
}
