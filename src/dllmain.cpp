/*****************************************************************************
 *
 *  PROJECT:        RiNetworkAdapter
 *  LICENSE:        See LICENSE in the top level directory
 *  FILE:           dllmain.cpp
 *  DESCRIPTION:    DllMain
 *  COPYRIGHT:      (c) 2021 RINWARES <rinwares.com>
 *  AUTHOR:         Rinat Namazov <rinat.namazov@rinwares.com>
 *
 *****************************************************************************/

#include "dllmain.hpp"

#include <Windows.h>

#include "Plugin.hpp"

Plugin* g_plugin;

BOOL APIENTRY DllMain(HMODULE hModule, DWORD dwReasonForCall, LPVOID lpReserved) {
    switch (dwReasonForCall) {
        case DLL_PROCESS_ATTACH:
            g_plugin = new Plugin();
            break;

        case DLL_PROCESS_DETACH:
            if (g_plugin != nullptr) {
                delete g_plugin;
            }
            break;
    }
    return TRUE;
}

DLLEXPORT void SetNetworkAdapterAddress(const char* address) { g_plugin->setNetworkAdapterAddress(address); }
