/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           lib.rs
 *  DESCRIPTION:    DllMain
 *  COPYRIGHT:      (c) 2021, 2023 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

use windows::Win32::{
    Foundation::{BOOL, HMODULE, TRUE},
    System::{
        LibraryLoader::DisableThreadLibraryCalls,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

pub mod plugin;
pub mod samp;
pub mod utils;

#[no_mangle]
extern "stdcall" fn DllMain(instance: HMODULE, reason: u32, _reserved: *mut ()) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                DisableThreadLibraryCalls(instance).unwrap();
            }
            plugin::initialize();
        }
        DLL_PROCESS_DETACH => {
            plugin::uninitialize();
        }
        _ => {}
    }
    TRUE
}
