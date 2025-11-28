use aarch64_cpu::{asm::barrier, registers::*};
use crate::mm::aarch64::MemAttr;
use core::arch::asm;

pub unsafe fn init_mmu(root_paddr: usize) {

    MAIR_EL1.set(MemAttr::MAIR_VALUE);

    // Enable TTBR0 and TTBR1 walks, page size = 4K, vaddr size = 48 bits, paddr size = 48 bits.
    let tcr_flags0 = TCR_EL1::EPD0::EnableTTBR0Walks
        + TCR_EL1::TG0::KiB_4
        + TCR_EL1::SH0::Inner
        + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::T0SZ.val(16);
    let tcr_flags1 = TCR_EL1::EPD1::EnableTTBR1Walks
        + TCR_EL1::TG1::KiB_4
        + TCR_EL1::SH1::Inner
        + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::T1SZ.val(16);
    TCR_EL1.write(TCR_EL1::IPS::Bits_48 + tcr_flags0 + tcr_flags1);
    barrier::isb(barrier::SY);

    // Set both TTBR0 and TTBR1
    let root_paddr = root_paddr as u64;
    TTBR0_EL1.set(root_paddr);
    TTBR1_EL1.set(root_paddr);

    // Flush the entire TLB
    flush_tlb(None);

    // Enable the MMU and turn on I-cache and D-cache
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);
    barrier::isb(barrier::SY);
}

#[inline]
pub fn flush_tlb(vaddr: Option<usize>) {
    if let Some(vaddr) = vaddr {
        const VA_MASK: usize = (1 << 44) - 1; // VA[55:12] => bits[43:0]
        let operand = (vaddr >> 12) & VA_MASK;

        #[cfg(not(feature = "arm-el2"))]
        unsafe {
            // TLB Invalidate by VA, All ASID, EL1, Inner Shareable
            asm!("tlbi vaae1is, {}; dsb sy; isb", in(reg) operand)
        }
        #[cfg(feature = "arm-el2")]
        unsafe {
            // TLB Invalidate by VA, EL2, Inner Shareable
            asm!("tlbi vae2is, {}; dsb sy; isb", in(reg) operand)
        }
    } else {
        // flush the entire TLB
        #[cfg(not(feature = "arm-el2"))]
        unsafe {
            // TLB Invalidate by VMID, All at stage 1, EL1
            asm!("tlbi vmalle1; dsb sy; isb")
        }
        #[cfg(feature = "arm-el2")]
        unsafe {
            // TLB Invalidate All, EL2
            asm!("tlbi alle2; dsb sy; isb")
        }
    }
}

pub fn virt_to_phys(vaddr: usize) -> usize {
    vaddr - 0xFFFF_0000_0000_0000 as usize
}

pub fn phys_to_virt(paddr: usize) -> usize {
    paddr + 0xFFFF_0000_0000_0000 as usize
}
