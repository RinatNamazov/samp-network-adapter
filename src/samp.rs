/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           samp.rs
 *  DESCRIPTION:    SA:MP misc functions
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

use winapi::um::winnt::{IMAGE_DOS_HEADER, IMAGE_NT_HEADERS32};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum SampVersion {
    Unknown,
    V037R1,
    V037R2,
    V037R3,
    V037R3A, // R3-1
    V037R4,
    V03DLR1,
}

pub fn get_samp_version(base_address: usize) -> SampVersion {
    unsafe {
        let dos_header = *(base_address as *const IMAGE_DOS_HEADER);
        let nt_headers =
            *((base_address + (dos_header.e_lfanew as usize)) as *const IMAGE_NT_HEADERS32);

        match nt_headers.OptionalHeader.AddressOfEntryPoint {
            0x31DF13 => SampVersion::V037R1,
            0x3195DD => SampVersion::V037R2,
            0xCC490 => SampVersion::V037R3,
            0xCC4D0 => SampVersion::V037R3A,
            0xCBCD0 => SampVersion::V037R4,
            0xFDB60 => SampVersion::V03DLR1,
            _ => SampVersion::Unknown,
        }
    }
}
