/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           utils.cpp
 *  DESCRIPTION:    Utils
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

#include "utils.hpp"

#include <Windows.h>

void PatchPointer(uintptr_t address, uintptr_t value) {
    DWORD vp = PAGE_EXECUTE_READWRITE;
    VirtualProtect(reinterpret_cast<void*>(address), sizeof(value), vp, &vp);
    *reinterpret_cast<uintptr_t*>(address) = value;
    VirtualProtect(reinterpret_cast<void*>(address), sizeof(value), vp, &vp);
}

void PatchCallAddress(uintptr_t address, void* value) { PatchPointer(address + 1, reinterpret_cast<uintptr_t>(value) - address - 5); }
