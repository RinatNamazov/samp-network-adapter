/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           Plugin.hpp
 *  DESCRIPTION:    Plugin
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

#pragma once

#include <Windows.h>

#include <cstdint>
#include <memory>
#include <string>

enum class SampVersion { SAMP_UNKNOWN, SAMP_037_R1, SAMP_037_R2, SAMP_037_R3, SAMP_037_R3_1, SAMP_037_R4, SAMP_03DL_R1 };

class Plugin final {
public:
    Plugin();
    ~Plugin();

    void setNetworkAdapterAddress(const char* address);

private:
    static uintptr_t   sm_sampBaseAddress;
    static SampVersion sm_sampVersion;
    static std::string sm_adapterAddress;
    static uintptr_t   sm_patchCallAddress;

    using t_RakPeer__Initialize = bool(__thiscall*)(void* _this, uint16_t maxConnections, uint16_t localPort, int32_t threadSleepTimer,
                                                    const char* forceHostAddress);
    static t_RakPeer__Initialize sm_RakPeer__Initialize;

    void parseCmdArgs(std::string& adapterAddress);
    void initializePatchs();
    bool detectSampVersion();

    static bool __fastcall hook_RakPeer__Initialize(void* ecx, void* edx, uint16_t maxConnections, uint16_t localPort,
                                                    int32_t threadSleepTimer, const char* forceHostAddress);
};
