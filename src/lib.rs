#![no_std]
extern crate alloc as alloc_crate;
use alloc_crate::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    boxed::Box,
    vec::Vec,
};
use core::{
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};

fn align_up<T>(u: usize) -> usize {
    (u + mem::align_of::<T>() - 1) / mem::align_of::<T>()
}

struct AlignedBuffer<A> {
    data: NonNull<[MaybeUninit<u8>]>,
    _align: PhantomData<*mut A>,
}

#[repr(C, align(1))] #[derive(Clone, Copy)] struct Align1([u8; 1]);
#[repr(C, align(2))] #[derive(Clone, Copy)] struct Align2([u8; 2]);
#[repr(C, align(4))] #[derive(Clone, Copy)] struct Align4([u8; 4]);
#[repr(C, align(8))] #[derive(Clone, Copy)] struct Align8([u8; 8]);
#[repr(C, align(16))] #[derive(Clone, Copy)] struct Align16([u8; 16]);
#[repr(C, align(32))] #[derive(Clone, Copy)] struct Align32([u8; 32]);
#[repr(C, align(64))] #[derive(Clone, Copy)] struct Align64([u8; 64]);
#[repr(C, align(128))] #[derive(Clone, Copy)] struct Align128([u8; 128]);
#[repr(C, align(256))] #[derive(Clone, Copy)] struct Align256([u8; 256]);
#[repr(C, align(512))] #[derive(Clone, Copy)] struct Align512([u8; 512]);
#[repr(C, align(1024))] #[derive(Clone, Copy)] struct Align1024([u8; 1024]);
#[repr(C, align(2048))] #[derive(Clone, Copy)] struct Align2048([u8; 2048]);
#[repr(C, align(4096))] #[derive(Clone, Copy)] struct Align4096([u8; 4096]);
#[repr(C, align(8192))] #[derive(Clone, Copy)] struct Align8192([u8; 8192]);
#[repr(C, align(16384))] #[derive(Clone, Copy)] struct Align16384([u8; 16384]);
#[repr(C, align(32768))] #[derive(Clone, Copy)] struct Align32768([u8; 32768]);
#[repr(C, align(65536))] #[derive(Clone, Copy)] struct Align65536([u8; 65536]);
#[repr(C, align(131072))] #[derive(Clone, Copy)] struct Align131072([u8; 131072]);
#[repr(C, align(262144))] #[derive(Clone, Copy)] struct Align262144([u8; 262144]);
#[repr(C, align(524288))] #[derive(Clone, Copy)] struct Align524288([u8; 524288]);
#[repr(C, align(1048576))] #[derive(Clone, Copy)] struct Align1048576([u8; 1048576]);
#[repr(C, align(2097152))] #[derive(Clone, Copy)] struct Align2097152([u8; 2097152]);
#[repr(C, align(4194304))] #[derive(Clone, Copy)] struct Align4194304([u8; 4194304]);
#[repr(C, align(8388608))] #[derive(Clone, Copy)] struct Align8388608([u8; 8388608]);
#[repr(C, align(16777216))] #[derive(Clone, Copy)] struct Align16777216([u8; 16777216]);
#[repr(C, align(33554432))] #[derive(Clone, Copy)] struct Align33554432([u8; 33554432]);
#[repr(C, align(67108864))] #[derive(Clone, Copy)] struct Align67108864([u8; 67108864]);
#[repr(C, align(134217728))] #[derive(Clone, Copy)] struct Align134217728([u8; 134217728]);
#[repr(C, align(268435456))] #[derive(Clone, Copy)] struct Align268435456([u8; 268435456]);
#[repr(C, align(536870912))] #[derive(Clone, Copy)] struct Align536870912([u8; 536870912]);

macro_rules! with_align {
    ($align:expr, $expr:expr) => {{
        #[allow(dead_code)]
        match $align {
            1 => {
                type Align = Align1;
                $expr
            },
            2 => {
                type Align = Align2;
                $expr
            },
            4 => {
                type Align = Align4;
                $expr
            },
            8 => {
                type Align = Align8;
                $expr
            },
            16 => {
                type Align = Align16;
                $expr
            },
            32 => {
                type Align = Align32;
                $expr
            },
            64 => {
                type Align = Align64;
                $expr
            },
            128 => {
                type Align = Align128;
                $expr
            },
            256 => {
                type Align = Align256;
                $expr
            },
            512 => {
                type Align = Align512;
                $expr
            },
            1024 => {
                type Align = Align1024;
                $expr
            },
            2048 => {
                type Align = Align2048;
                $expr
            },
            4096 => {
                type Align = Align4096;
                $expr
            },
            8192 => {
                type Align = Align8192;
                $expr
            },
            16384 => {
                type Align = Align16384;
                $expr
            },
            32768 => {
                type Align = Align32768;
                $expr
            },
            65536 => {
                type Align = Align65536;
                $expr
            },
            131072 => {
                type Align = Align131072;
                $expr
            },
            262144 => {
                type Align = Align262144;
                $expr
            },
            524288 => {
                type Align = Align524288;
                $expr
            },
            1048576 => {
                type Align = Align1048576;
                $expr
            },
            2097152 => {
                type Align = Align2097152;
                $expr
            },
            4194304 => {
                type Align = Align4194304;
                $expr
            },
            8388608 => {
                type Align = Align8388608;
                $expr
            },
            16777216 => {
                type Align = Align16777216;
                $expr
            },
            33554432 => {
                type Align = Align33554432;
                $expr
            },
            67108864 => {
                type Align = Align67108864;
                $expr
            },
            134217728 => {
                type Align = Align134217728;
                $expr
            },
            268435456 => {
                type Align = Align268435456;
                $expr
            },
            536870912 => {
                type Align = Align536870912;
                $expr
            },
            _ => unreachable!(),
        }
    }};
}

impl<A> AlignedBuffer<A> {
    fn new(size_bytes: usize) -> AlignedBuffer<A> {
        let data: *mut [MaybeUninit<u8>] = if size_bytes == 0 {
            Box::into_raw(Vec::new().into_boxed_slice())
        } else {
            unsafe {
                let len = align_up::<A>(size_bytes);
                let size = mem::align_of::<A>().checked_mul(len).expect("array size overflow");
                let align = mem::align_of::<A>();
                let layout = Layout::from_size_align_unchecked(size, align);
                let data_raw_ptr: *mut u8 = alloc(layout);
                if data_raw_ptr == ptr::null_mut() {
                    handle_alloc_error(layout);
                }
                slice::from_raw_parts_mut(
                    data_raw_ptr as *mut MaybeUninit<u8>,
                    size,
                )
            }
        };
        AlignedBuffer {
            data: unsafe{ NonNull::new_unchecked(data) },
            _align: PhantomData,
        }
    }

    fn reserve(&mut self, additional: usize) {
        if self.len() == 0 {
            *self = Self::new(additional);
        } else {
            with_align!(mem::align_of::<A>(), unsafe {
                debug_assert_eq!(0, self.len() % mem::size_of::<MaybeUninit<Align>>());
                debug_assert_eq!(0, self.as_ptr().align_offset(mem::align_of::<Align>()));

                let data_aligned: *mut [MaybeUninit<Align>] = core::slice::from_raw_parts_mut(
                    self.as_mut_ptr() as *mut MaybeUninit<Align>,
                    self.len() / mem::size_of::<MaybeUninit<Align>>(),
                );
                let data_box: Box<[MaybeUninit<Align>]> = Box::from_raw(data_aligned);

                let mut vec = Vec::<MaybeUninit<Align>>::from(data_box);
                let new_len = align_up::<A>(self.len() + additional);
                // we use `resize_with` to avoid overflowing the stack on large alignments
                vec.resize_with(new_len, || MaybeUninit::uninit());

                let data_box: Box<[MaybeUninit<Align>]> = vec.into_boxed_slice();
                let data_aligned: *mut [MaybeUninit<Align>] = Box::into_raw(data_box);

                let size = mem::size_of::<MaybeUninit<Align>>().checked_mul((*data_aligned).len()).expect("array size overflow");
                let data = core::slice::from_raw_parts_mut(
                    (*data_aligned).as_mut_ptr() as *mut MaybeUninit<u8>,
                    size
                );
                self.data = NonNull::new_unchecked(data);
            });
        }
    }
}

impl<A> Deref for AlignedBuffer<A> {
    type Target = [MaybeUninit<u8>];
    fn deref(&self) -> &[MaybeUninit<u8>] {
        unsafe{ self.data.as_ref() }
    }
}

impl<A> DerefMut for AlignedBuffer<A> {
    fn deref_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe{ self.data.as_mut() }
    }
}

impl<A> Drop for AlignedBuffer<A> {
    fn drop(&mut self) {
        if self.len() > 0 {
            unsafe {
                let layout = Layout::from_size_align_unchecked(self.len(), mem::align_of::<A>());
                dealloc(self.as_mut_ptr() as *mut u8, layout);
            }
        }
    }
}

pub struct HetBuf<A> {
    data: AlignedBuffer<A>,
    head: usize,
}

impl<A> HetBuf<A> {
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

    pub fn push_item_with<T, F>(&mut self, item: F) -> &mut T
        where F: FnOnce() -> T,
    {
        let item_ref = &mut self.alloc_slice::<T>(1)[0];
        unsafe {
            item_ref.as_mut_ptr().write(item());
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
        assert!(
            mem::align_of::<T>() <= mem::align_of::<A>(),
            "unsupported alignment: {} is larger than {}", mem::align_of::<T>(), mem::align_of::<A>()
        );
        if self.data.len() == 0 {
            self.data.reserve(mem::size_of::<T>());
        }
        let align_offset = (unsafe{ self.data.as_ptr().offset(self.head as isize) } as *const MaybeUninit<u8>).align_offset(mem::align_of::<T>());
        let head_aligned = self.head + align_offset;
        let head_new = head_aligned + mem::size_of::<T>() * len;
        let current_len = self.data.len();

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
    use proptest::{
        prelude::*,
        collection::vec,
    };
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

    fn align_strategy() -> impl Strategy<Value=usize> {
        (0..=12u32).prop_map(|a| 2usize.pow(a))
    }

    proptest! {
        #[test]
        fn fuzz(
            base_alignment in align_strategy(),
            insertion_alignments in vec(align_strategy(), 0..2048),
        ) {
            let zero_buf = &*alloc_crate::vec![0; 2usize.pow(12)];
            with_align!(base_alignment, {
                let mut buf = HetBuf::<Align>::new();
                for insert in insertion_alignments {
                    if insert > base_alignment {
                        continue;
                    }
                    with_align!(insert, {
                        assert_eq!(
                            &zero_buf[0..insert],
                            &buf.push_item_with(|| unsafe{ mem::zeroed::<Align>() }).0[..]
                        );
                    })
                }
            })
        }
    }
}
