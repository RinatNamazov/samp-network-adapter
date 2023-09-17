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
    V037R1,
    V037R2,
    V037R3,
    V037R3_1,
    V037R4,
    V037R4_2,
    V037R5,
    V03DLR1,
}

pub fn get_samp_version(base_address: usize) -> Result<SampVersion, &'static str> {
    let entry_point = unsafe {
        let dos_header = *(base_address as *const IMAGE_DOS_HEADER);
        let nt_headers =
            *((base_address + (dos_header.e_lfanew as usize)) as *const IMAGE_NT_HEADERS32);

        nt_headers.OptionalHeader.AddressOfEntryPoint
    };

    match entry_point {
        0x31DF13 => Ok(SampVersion::V037R1),
        0x3195DD => Ok(SampVersion::V037R2),
        0xCC490 => Ok(SampVersion::V037R3),
        0xCC4D0 => Ok(SampVersion::V037R3_1),
        0xCBCD0 => Ok(SampVersion::V037R4),
        0xCBCB0 => Ok(SampVersion::V037R4_2),
        0xCBC90 => Ok(SampVersion::V037R5),
        0xFDB60 => Ok(SampVersion::V03DLR1),
        _ => Err("Unknown SA-MP version."),
    }
}
