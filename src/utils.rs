/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           utils.rs
 *  DESCRIPTION:    Utils
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

use std::ffi::CString;
use winapi::shared::minwindef::{DWORD, HMODULE, LPVOID};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

pub fn get_module_handle(name: &str) -> HMODULE {
    let c_name = CString::new(name).unwrap();
    unsafe { GetModuleHandleA(c_name.as_ptr()) }
}

pub unsafe fn patch_pointer(address: usize, value: usize) {
    let address = address as LPVOID;
    let size = std::mem::size_of::<usize>();
    let mut vp: DWORD = PAGE_EXECUTE_READWRITE;
    VirtualProtect(address, size, vp, &mut vp);
    *(address as *mut usize) = value;
    VirtualProtect(address, size, vp, &mut vp);
}

pub unsafe fn patch_call_address(address: usize, value: usize) {
    patch_pointer(
        address + 1,
        value - address - std::mem::size_of::<*const usize>(),
    )
}

pub unsafe fn extract_call_target_address(address: usize) -> usize {
    *((address + 0x1) as *const usize)
}
