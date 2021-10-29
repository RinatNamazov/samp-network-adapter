/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           lib.rs
 *  DESCRIPTION:    DllMain
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

use winapi::shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

pub mod plugin;
pub mod samp;
pub mod utils;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn DllMain(instance: HMODULE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                DisableThreadLibraryCalls(instance);
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
