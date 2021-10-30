/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           Plugin.cpp
 *  DESCRIPTION:    Plugin
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

#include "Plugin.hpp"

#include <SimpleIni.h>
#include <Windows.h>

#include <sstream>

#include "utils.hpp"

const char* CONFIG_FILENAME = "RiNetworkAdapter.ini";

const char* LOCAL_IP_ADDRESS = "127.0.0.1";

uintptr_t                     Plugin::sm_sampBaseAddress = 0;
SampVersion                   Plugin::sm_sampVersion     = SampVersion::SAMP_UNKNOWN;
std::string                   Plugin::sm_adapterAddress;
Plugin::t_RakPeer__Initialize Plugin::sm_RakPeer__Initialize = nullptr;
uintptr_t                     Plugin::sm_patchCallAddress    = 0;

Plugin::Plugin() {
    sm_sampBaseAddress = reinterpret_cast<uintptr_t>(GetModuleHandle("samp.dll"));
    if (sm_sampBaseAddress == 0) {
        return;
    }

    if (!detectSampVersion()) {
        return;
    }

    initializePatchs();

    parseCmdArgs(sm_adapterAddress);

    if (sm_adapterAddress.empty()) {
        CSimpleIniA ini;
        if (ini.LoadFile(CONFIG_FILENAME) == SI_OK) {
            sm_adapterAddress = ini.GetValue("ri_network_adapter", "address", "127.0.0.1");
        }
    }
}

Plugin::~Plugin() {
    // Unhook
    PatchCallAddress(sm_patchCallAddress, sm_RakPeer__Initialize);
}

void Plugin::setNetworkAdapterAddress(const char* address) { sm_adapterAddress = address; }

void Plugin::parseCmdArgs(std::string& adapterAddress) {
    std::istringstream cmdLine(GetCommandLine());
    std::string        arg;
    while (std::getline(cmdLine, arg, ' ')) {
        if (arg == "--adapter_address") {
            std::getline(cmdLine, adapterAddress, ' ');
            break;
        }
    }
}

void Plugin::initializePatchs() {
    uintptr_t rakPeerInitAddr = sm_sampBaseAddress;
    sm_patchCallAddress += sm_sampBaseAddress;

    switch (sm_sampVersion) {
        case SampVersion::SAMP_037_R1: {
            rakPeerInitAddr += 0x3ECB0;
            sm_patchCallAddress += 0x30667;
            break;
        }
        case SampVersion::SAMP_037_R2: {
            rakPeerInitAddr += 0x3ED90;
            sm_patchCallAddress += 0x30747;
            break;
        }
        case SampVersion::SAMP_037_R3:
        case SampVersion::SAMP_037_R3_1: {
            rakPeerInitAddr += 0x42060;
            sm_patchCallAddress += 0x33A17;
            break;
        }
        case SampVersion::SAMP_037_R4: {
            rakPeerInitAddr += 0x427A0;
            sm_patchCallAddress += 0x34157;
            break;
        }
        case SampVersion::SAMP_03DL_R1: {
            rakPeerInitAddr += 0x42260;
            sm_patchCallAddress += 0x33C17;
            break;
        }
        case SampVersion::SAMP_UNKNOWN:
        default:
            break;
    }

    sm_RakPeer__Initialize = reinterpret_cast<t_RakPeer__Initialize>(rakPeerInitAddr);

    PatchCallAddress(sm_patchCallAddress, hook_RakPeer__Initialize);
}

bool Plugin::detectSampVersion() {
    IMAGE_NT_HEADERS* ntheader
        = reinterpret_cast<IMAGE_NT_HEADERS*>(sm_sampBaseAddress + reinterpret_cast<IMAGE_DOS_HEADER*>(sm_sampBaseAddress)->e_lfanew);

    switch (ntheader->OptionalHeader.AddressOfEntryPoint) {
        case 0x31DF13:
            sm_sampVersion = SampVersion::SAMP_037_R1;
            break;
        case 0x3195DD:
            sm_sampVersion = SampVersion::SAMP_037_R2;
            break;
        case 0xCC490:
            sm_sampVersion = SampVersion::SAMP_037_R3;
            break;
        case 0xCC4D0:
            sm_sampVersion = SampVersion::SAMP_037_R3_1;
            break;
        case 0xCBCD0:
            sm_sampVersion = SampVersion::SAMP_037_R4;
            break;
        case 0xFDB60:
            sm_sampVersion = SampVersion::SAMP_03DL_R1;
            break;
        default:
            sm_sampVersion = SampVersion::SAMP_UNKNOWN;
            break;
    }

    return sm_sampVersion != SampVersion::SAMP_UNKNOWN;
}

bool __fastcall Plugin::hook_RakPeer__Initialize(void* ecx, void* edx, uint16_t maxConnections, uint16_t localPort,
                                                 int32_t threadSleepTimer, const char* forceHostAddress) {
    if (sm_adapterAddress == LOCAL_IP_ADDRESS) {
        forceHostAddress = nullptr;
    } else {
        forceHostAddress = sm_adapterAddress.c_str();
    }

    return sm_RakPeer__Initialize(ecx, maxConnections, localPort, threadSleepTimer, forceHostAddress);
}
