// MIT License

// Copyright (c) 2021 AnonmousDapper

use core::marker::PhantomData;

use crate::memory::{
    page::{Page, PageSize, Size4Kib},
    PhysAddr, VirtAddr,
};

// ## virtual pages

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Frame<S: PageSize = Size4Kib> {
    start: PhysAddr,
    size: PhantomData<S>,
}

impl<S: PageSize> Frame<S> {
    #[inline]
    pub fn start_address(self) -> PhysAddr {
        self.start
    }

    #[inline]
    pub fn size(self) -> u64 {
        S::SIZE
    }
}
