// MIT License

// Copyright (c) 2021 AnonmousDapper

use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Not};

use core::marker::PhantomData;

use crate::memory::{PhysAddr, VirtAddr};

pub trait PageSize: Copy + Eq + PartialOrd + Ord {
    const SIZE: u64;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size4Kib {}

impl PageSize for Size4Kib {
    const SIZE: u64 = 4096;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size2Mib {}

impl PageSize for Size2Mib {
    const SIZE: u64 = Size4Kib::SIZE * 512;
}

// ## PTE flags

pub enum TableEntryFlags {
    None = 0,
    Valid = 1,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    User = 1 << 4,
    Global = 1 << 5,
    Accessed = 1 << 6,
    Dirty = 1 << 7,

    ReadWrite = (1 << 1) | (1 << 2),
    ReadExecute = (1 << 1) | (1 << 3),
    Rwx = (1 << 1) | (1 << 2) | (1 << 3),

    ReadWriteU = (1 << 1) | (1 << 2) | (1 << 4),
    ReadExecuteU = (1 << 1) | (1 << 3) | (1 << 4),
    RwxU = (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4),
}

impl Not for TableEntryFlags {
    type Output = u8;

    fn not(self) -> Self::Output {
        !(self as u8)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct EntryFlags(u8);

impl BitAnd<TableEntryFlags> for EntryFlags {
    type Output = u8;

    #[inline]
    fn bitand(self, rhs: TableEntryFlags) -> Self::Output {
        self.0 & rhs as u8
    }
}

impl BitAnd<u8> for EntryFlags {
    type Output = u8;

    #[inline]
    fn bitand(self, rhs: u8) -> Self::Output {
        self.0 & rhs
    }
}

impl BitOrAssign<TableEntryFlags> for EntryFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: TableEntryFlags) {
        self.0 |= rhs as u8
    }
}

impl BitAndAssign<u8> for EntryFlags {
    #[inline]
    fn bitand_assign(&mut self, rhs: u8) {
        self.0 &= rhs
    }
}

impl EntryFlags {
    #[inline]
    pub fn set_field(&mut self, field: TableEntryFlags, val: bool) {
        if val {
            *self |= field;
        } else {
            *self &= !field;
        }
    }

    #[inline]
    pub fn is_valid(self) -> bool {
        self & TableEntryFlags::Valid == 1
    }

    #[inline]
    pub fn set_valid(&mut self, val: bool) {
        self.set_field(TableEntryFlags::Valid, val);
    }

    #[inline]
    pub fn is_read(self) -> bool {
        self & TableEntryFlags::Read == 1
    }

    #[inline]
    pub fn set_read(&mut self, val: bool) {
        self.set_field(TableEntryFlags::Read, val);
    }

    #[inline]
    pub fn is_write(self) -> bool {
        self & TableEntryFlags::Write == 1
    }

    #[inline]
    pub fn set_write(&mut self, val: bool) {
        self.set_field(TableEntryFlags::Write, val);
    }

    #[inline]
    pub fn is_execute(self) -> bool {
        self & TableEntryFlags::Execute == 1
    }

    #[inline]
    pub fn set_execute(&mut self, val: bool) {
        self.set_field(TableEntryFlags::Execute, val);
    }

    #[inline]
    pub fn is_user(self) -> bool {
        self & TableEntryFlags::User == 1
    }

    #[inline]
    pub fn set_user(&mut self, val: bool) {
        self.set_field(TableEntryFlags::User, val);
    }

    #[inline]
    pub fn is_global(self) -> bool {
        self & TableEntryFlags::Global == 1
    }

    #[inline]
    pub fn set_global(&mut self, val: bool) {
        self.set_field(TableEntryFlags::Global, val);
    }

    #[inline]
    pub fn is_accessed(self) -> bool {
        self & TableEntryFlags::Accessed == 1
    }

    #[inline]
    pub fn set_accessed(&mut self, val: bool) {
        self.set_field(TableEntryFlags::Accessed, val);
    }

    #[inline]
    pub fn is_dirty(self) -> bool {
        self & TableEntryFlags::Dirty == 1
    }

    #[inline]
    pub fn set_dirty(&mut self, val: bool) {
        self.set_field(TableEntryFlags::Dirty, val);
    }

    #[inline]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

// ## virtual pages

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Page<S: PageSize = Size4Kib> {
    start: VirtAddr,
    size: PhantomData<S>,
}

impl<S: PageSize> Page<S> {
    #[inline]
    pub fn start_address(self) -> VirtAddr {
        self.start
    }

    #[inline]
    pub fn size(self) -> u64 {
        S::SIZE
    }
}

// ## the PTE itself

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

// 63 62 61 60    54 53                       28 27      19 18      10 9 8 7  6  5  4  3  2  1  0
// |-| |--||-------||--------------------------||---------||---------||--||-||-||-||-||-||-||-||-|
//  1   2     7                26                   9          9       2   1  1  1  1  1  1  1  1
//  N PBMT Reserved          PPN[2]               PPN[1]     PPN[0]   RSW  D  A  G  U  X  W  R  V

// 63->54 := 0

impl PageTableEntry {
    #[inline]
    pub const fn new(entry: u64) -> Self {
        Self(entry << 10 >> 10)
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.flags().is_valid()
    }

    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.flags() & TableEntryFlags::Rwx != 0
    }

    #[inline]
    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    #[inline]
    pub fn is_free(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn set_free(&mut self) {
        self.0 = 0;
    }

    #[inline]
    pub fn set(&mut self, new: u64) {
        let flags = EntryFlags(new as u8);

        match (flags.is_execute(), flags.is_write(), flags.is_read()) {
            (false, true, false) | (true, true, false) => {
                panic!(
                    "RWX flags are set to a reserved configuration: {:03b}",
                    flags.as_u8() & 0x7
                )
            }
            _ => {
                self.0 = new;
            }
        }
    }

    #[inline]
    pub const fn flags(&self) -> EntryFlags {
        EntryFlags(self.0 as u8)
    }

    #[inline]
    pub const fn ppn_0(&self) -> u32 {
        (self.0 >> 10) as u32 & 0x1FF
    }

    #[inline]
    pub const fn ppn_1(&self) -> u32 {
        (self.0 >> 10 >> 9) as u32 & 0x1FF
    }

    #[inline]
    pub const fn ppn_2(&self) -> u32 {
        (self.0 >> 10 >> 18) as u32 & 0x3FF_FFFF
    }

    #[inline]
    pub const fn ppn_parts(&self) -> [u32; 3] {
        [self.ppn_0(), self.ppn_1(), self.ppn_2()]
    }

    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

#[repr(C, align(4096))]
pub struct PageTable([PageTableEntry; 512]);

impl PageTable {}
