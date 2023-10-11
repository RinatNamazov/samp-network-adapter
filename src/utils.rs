/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           utils.rs
 *  DESCRIPTION:    Utils
 *  COPYRIGHT:      (c) 2021, 2023 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

use std::ffi::c_void;

use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

pub unsafe fn patch_pointer(address: usize, value: usize) {
    let address = address as *const c_void;
    let size = std::mem::size_of::<usize>();
    let mut vp = PAGE_EXECUTE_READWRITE;
    VirtualProtect(address, size, vp, &mut vp).unwrap();
    *(address as *mut usize) = value;
    VirtualProtect(address, size, vp, &mut vp).unwrap();
}

pub unsafe fn patch_call_address(address: usize, value: usize) {
    patch_pointer(address + 1, value - address - 1 - 4);
}

pub unsafe fn extract_call_target_address(address: usize) -> usize {
    let relative = *((address + 1) as *const usize);
    address + relative + 1 + 4
}
