# *****************************************************************************
# *
# *  PROJECT:       RiNetworkAdapter
# *  LICENSE:       See LICENSE in the top level directory
# *  FILE:          CMakeLists.txt
# *  DESCRIPTION:   CMakeLists
# *  COPYRIGHT:     (c) 2021 RINWARES <rinwares.com>
# *  AUTHOR:        Rinat Namazov <rinat.namazov@rinwares.com>
# *
# *****************************************************************************

cmake_minimum_required(VERSION 3.8)
project("RiNetworkAdapter")

find_path(SIMPLEINI_INCLUDE_DIRS "ConvertUTF.c")

set(RINETWORKADAPTER_SOURCES
	"src/dllmain.hpp"
	"src/dllmain.cpp"
	"src/utils.hpp"
	"src/utils.cpp"
	"src/Plugin.hpp"
	"src/Plugin.cpp"
)

add_library(RiNetworkAdapter SHARED ${RINETWORKADAPTER_SOURCES})

set_target_properties(RiNetworkAdapter
	PROPERTIES
		SUFFIX ".asi"
)

target_include_directories(RiNetworkAdapter PRIVATE ${SIMPLEINI_INCLUDE_DIRS})  

install(TARGETS RiNetworkAdapter DESTINATION .)
