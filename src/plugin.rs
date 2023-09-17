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

use crate::samp;
use crate::samp::SampVersion;
use crate::utils;
use ini::Ini;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::sync::Once;

const CONFIG_FILENAME: &str = "RiNetworkAdapter.ini";

const LOCAL_IP_ADDRESS: &str = "127.0.0.1";

// thiscall
type RakPeerInitializeFuncType = extern "fastcall" fn(
    ecx: usize,
    edx: usize,
    max_connections: u16,
    local_port: u16,
    thread_sleep_timer: i32,
    force_host_address: *const c_char,
) -> bool;

static mut PLUGIN: Option<Plugin> = None;

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

        let network_adapter_address: CString;

        match Plugin::parse_cmd_args() {
            Some(adapter) => network_adapter_address = CString::new(adapter).unwrap(),
            None => match Ini::load_from_file(CONFIG_FILENAME) {
                Ok(conf) => {
                    let section = conf.section(Some("ri_network_adapter")).unwrap();
                    let address = section.get("address").unwrap();

                    network_adapter_address = CString::new(address).unwrap();
                }
                Err(_) => {
                    network_adapter_address = CString::new("127.0.0.1").unwrap();
                }
            },
        }
        Plugin {
            patch_call_address,
            rakpeer_initialize,
            network_adapter_address,
        }
    }

    fn parse_cmd_args() -> Option<String> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() % 2 == 0 {
            for (pos, arg) in args.iter().enumerate() {
                if arg == "--adapter_address" {
                    let next_arg = &args[pos + 1];
                    return Some(next_arg.clone());
                }
            }
        }

        None
    }

    fn initialize_patchs(&self) {
        unsafe {
            utils::patch_call_address(self.patch_call_address, hook_rakpeer_initialize as usize);
        }
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
    let samp_base_address = utils::get_module_handle("samp.dll") as usize;
    if samp_base_address == 0 {
        return;
    }

    let samp_version = samp::get_samp_version(samp_base_address);
    if samp_version.is_err() {
        return;
    }

    let plugin = Plugin::new(samp_base_address, samp_version.unwrap());
    plugin.initialize_patchs();

    unsafe {
        PLUGIN = Some(plugin);
    }
}

pub fn uninitialize() {
    static DESTROY: Once = Once::new();

    DESTROY.call_once(|| unsafe {
        PLUGIN.take();
    });
}

extern "fastcall" fn hook_rakpeer_initialize(
    ecx: usize,
    edx: usize,
    max_connections: u16,
    local_port: u16,
    thread_sleep_timer: i32,
    _force_host_address: *const c_char,
) -> bool {
    let plugin = unsafe { PLUGIN.as_mut().unwrap() };

    let adapter_address = if plugin.network_adapter_address.to_str().unwrap() == LOCAL_IP_ADDRESS {
        ptr::null()
    } else {
        plugin.network_adapter_address.as_ptr()
    };

    (plugin.rakpeer_initialize)(
        ecx,
        edx,
        max_connections,
        local_port,
        thread_sleep_timer,
        adapter_address,
    )
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SetNetworkAdapterAddress(address: *mut c_char) {
    unsafe {
        let plugin = PLUGIN.as_mut().unwrap();
        plugin.network_adapter_address = CString::from(CStr::from_ptr(address));
    }
}
