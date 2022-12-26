// MIT License

// Copyright (c) 2021 AnonmousDapper

pub mod page;

pub mod frame;

// TODO:

// 1. Allocator design (list, etc)

// 2. Allocator init

// 3. Frame allocator

// 4. Mapping & translation

// 5. malloc/free

extern "C" {
    pub static HEAP_SIZE: usize;
    pub static HEAP_START: usize;
    pub static KERNEL_STACK_START: usize;
    pub static KERNEL_STACK_END: usize;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(u64);

// 38      30 29     21 20     12 11          0
// |--------||--------||--------||------------|
//     9         9         9          12
//   VPN[2]    VPN[1]    VPN[0]   page offset

// 63->39 := 38

impl VirtAddr {
    #[inline]
    pub const fn new_truncate(addr: u64) -> Self {
        Self(((addr << 25) as i64 >> 25) as u64)
    }

    #[inline]
    pub const fn page_offset(self) -> u16 {
        self.0 as u16 & 0xFFF
    }

    #[inline]
    pub const fn vpn_0(self) -> u32 {
        (self.0 >> 12) as u32 & 0x1FF
    }

    #[inline]
    pub const fn vpn_1(self) -> u32 {
        (self.0 >> 12 >> 9) as u32 & 0x1FF
    }

    #[inline]
    pub const fn vpn_2(self) -> u32 {
        (self.0 >> 12 >> 9 >> 9) as u32 & 0x1FF
    }

    #[inline]
    pub const fn parts(self) -> [u32; 3] {
        [self.vpn_0(), self.vpn_1(), self.vpn_2()]
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(u64);

// 55                        30 29      21 20      12 11          0
// |--------------------------||---------||---------||------------|
//              26                  9          9           12
//            PPN[2]              PPN[1]     PPN[0]   page offset

impl PhysAddr {
    #[inline]
    pub const fn new_truncate(addr: u64) -> Self {
        Self(addr % (1 << 56))
    }

    #[inline]
    pub const fn page_offset(self) -> u16 {
        self.0 as u16 & 0xFFF
    }

    #[inline]
    pub const fn ppn_0(self) -> u32 {
        (self.0 >> 12) as u32 & 0x1FF
    }

    #[inline]
    pub const fn ppn_1(self) -> u32 {
        (self.0 >> 12 >> 9) as u32 & 0x1FF
    }

    #[inline]
    pub const fn ppn_2(self) -> u32 {
        (self.0 >> 12 >> 9 >> 9) as u32 & 0x3FF_FFFF
    }

    #[inline]
    pub const fn parts(self) -> [u32; 3] {
        [self.ppn_0(), self.ppn_1(), self.ppn_2()]
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

#[must_use]
pub fn init_paging() {
    let num_zones = HEAP_SIZE / (2u64.pow(MAX_ORDER - 1) * 4096);
}
