use crate::mm::aarch64::*;

use core::ops::{Deref, DerefMut};

#[repr(align(4096))]
pub struct Aligned4K<T: Sized>(T);

impl<T: Sized> Aligned4K<T> {
    /// Creates a new [`Aligned4K`] instance with the given value.
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for Aligned4K<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Aligned4K<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[unsafe(link_section = ".data")]
pub static mut BOOT_PT_L0: Aligned4K<[A64PTE; 512]> = Aligned4K::new([A64PTE::empty(); 512]);

#[unsafe(link_section = ".data")]
static mut BOOT_PT_L1: Aligned4K<[A64PTE; 512]> = Aligned4K::new([A64PTE::empty(); 512]);

/// 初始化启动页表
pub unsafe fn init_boot_page_table() {
    unsafe {
        // 0x0000_0000_0000 ~ 0x0080_0000_0000, table
        BOOT_PT_L0[0] = A64PTE::new_table(&raw mut BOOT_PT_L1 as usize);
        // 0x0000_0000_0000..0x0000_4000_0000, 1G block, device memory
        BOOT_PT_L1[0] = A64PTE::new_page(
            0,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
            true,
        );
        // 0x0000_4000_0000..0x0000_8000_0000, 1G block, normal memory
        BOOT_PT_L1[1] = A64PTE::new_page(
            0x4000_0000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
            true,
        );
        // 0x0000_8000_0000..0x0000_C000_0000, 1G block, device memory (for UART at 0x9000000)
        BOOT_PT_L1[2] = A64PTE::new_page(
            0x8000_0000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
            true,
        );
    }
}
