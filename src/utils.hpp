/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           utils.hpp
 *  DESCRIPTION:    Utils
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

#pragma once

#include <cstdint>
#include <string>

void PatchPointer(uintptr_t address, uintptr_t value);
void PatchCallAddress(uintptr_t address, void* value);
