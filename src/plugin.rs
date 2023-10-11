/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           plugin.rs
 *  DESCRIPTION:    Plugin
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

use std::{
    cell::OnceCell,
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr,
};

use ini::Ini;
use windows::{core::w, Win32::System::LibraryLoader::GetModuleHandleW};

use crate::{samp, samp::SampVersion, utils};

const CONFIG_FILENAME: &str = "RiNetworkAdapter.ini";

const LOCAL_IP_ADDRESS: &str = "127.0.0.1";

// thiscall
type RakPeerInitializeFuncType = extern "thiscall" fn(
    this: usize,
    max_connections: u16,
    local_port: u16,
    thread_sleep_timer: i32,
    force_host_address: *const c_char,
) -> bool;

static mut PLUGIN: OnceCell<Plugin> = OnceCell::new();

pub struct Plugin {
    patch_call_address: usize,
    rakpeer_initialize: RakPeerInitializeFuncType,
    network_adapter_address: CString,
}

impl Drop for Plugin {
    fn drop(&mut self) {
        // Unhook
        unsafe {
            utils::patch_call_address(self.patch_call_address, self.rakpeer_initialize as usize);
        }
    }
}

impl Plugin {
    pub fn new(samp_base_address: usize, samp_version: SampVersion) -> Plugin {
        let patch_call_address = samp_base_address + Plugin::get_patch_call_offset(samp_version);

        let rakpeer_initialize: RakPeerInitializeFuncType =
            unsafe { std::mem::transmute(utils::extract_call_target_address(patch_call_address)) };

        let network_adapter_address = match Plugin::parse_cmd_args() {
            Some(adapter) => CString::new(adapter).unwrap(),
            None => match Ini::load_from_file(CONFIG_FILENAME) {
                Ok(conf) => {
                    let section = conf
                        .section(Some("ri_network_adapter"))
                        .expect("section not found");
                    let address = section.get("address").expect("address not found");

                    CString::new(address).unwrap()
                }
                Err(_) => CString::default(),
            },
        };

        unsafe {
            utils::patch_call_address(patch_call_address, hook_rakpeer_initialize as usize);
        }

        Plugin {
            patch_call_address,
            rakpeer_initialize,
            network_adapter_address,
        }
    }

    fn parse_cmd_args() -> Option<String> {
        let mut args = std::env::args();

        while let Some(arg) = args.next() {
            if arg == "--adapter_address" {
                if let Some(next_arg) = args.next() {
                    return Some(next_arg);
                }
            }
        }

        None
    }

    fn get_patch_call_offset(samp_version: SampVersion) -> usize {
        match samp_version {
            SampVersion::V037R1 => 0x30667,
            SampVersion::V037R2 => 0x30747,
            SampVersion::V037R3 | SampVersion::V037R3_1 => 0x33A17,
            SampVersion::V037R4 => 0x34107,
            SampVersion::V037R4_2 | SampVersion::V037R5 => 0x34157,
            SampVersion::V03DLR1 => 0x33C17,
        }
    }
}

pub fn initialize() {
    if let Ok(samp_base_address) = unsafe { GetModuleHandleW(w!("samp.dll")) } {
        if !samp_base_address.is_invalid() {
            let samp_base_address = samp_base_address.0 as usize;
            if let Ok(samp_version) = samp::get_samp_version(samp_base_address) {
                unsafe { PLUGIN.get_or_init(|| Plugin::new(samp_base_address, samp_version)) };
            }
        }
    }
}

pub fn uninitialize() {
    unsafe {
        PLUGIN.take();
    }
}

extern "thiscall" fn hook_rakpeer_initialize(
    this: usize,
    max_connections: u16,
    local_port: u16,
    thread_sleep_timer: i32,
    _force_host_address: *const c_char,
) -> bool {
    let plugin = unsafe { PLUGIN.get().unwrap() };
    let adapter = &plugin.network_adapter_address;

    let adapter_address = if adapter.is_empty() || adapter.to_string_lossy() == LOCAL_IP_ADDRESS {
        ptr::null()
    } else {
        adapter.as_ptr()
    };

    (plugin.rakpeer_initialize)(
        this,
        max_connections,
        local_port,
        thread_sleep_timer,
        adapter_address,
    )
}

#[no_mangle]
pub extern "C" fn SetNetworkAdapterAddress(address: *mut c_char) {
    let address = unsafe { CStr::from_ptr(address) };
    let address = CString::from(address);

    let plugin = unsafe { PLUGIN.get_mut().unwrap() };
    plugin.network_adapter_address = address;
}
