cmake_minimum_required(VERSION 3.20)

project(CookBookCPP LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(COOKBOOK_GSL_DIR "${CMAKE_SOURCE_DIR}/third_party/microsoft-gsl")

if(EXISTS "${COOKBOOK_GSL_DIR}/CMakeLists.txt")
    add_subdirectory("${COOKBOOK_GSL_DIR}" EXCLUDE_FROM_ALL)
elseif(EXISTS "${COOKBOOK_GSL_DIR}/include/gsl/gsl")
    add_library(Microsoft.GSL INTERFACE)
    add_library(Microsoft.GSL::GSL ALIAS Microsoft.GSL)
    target_include_directories(Microsoft.GSL INTERFACE
        "${COOKBOOK_GSL_DIR}/include"
    )
else()
    message(FATAL_ERROR
        "Microsoft GSL not found. Put it under third_party/microsoft-gsl "
        "so that third_party/microsoft-gsl/include/gsl/gsl exists."
    )
endif()

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG "${CMAKE_BINARY_DIR}/bin")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE "${CMAKE_BINARY_DIR}/bin")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELWITHDEBINFO "${CMAKE_BINARY_DIR}/bin")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_MINSIZEREL "${CMAKE_BINARY_DIR}/bin")

file(GLOB_RECURSE DEMO_SOURCES CONFIGURE_DEPENDS
    "${CMAKE_SOURCE_DIR}/demos/*.cpp"
)

if(NOT DEMO_SOURCES)
    message(STATUS "No demo source files found under demos/")
endif()

foreach(DEMO_SOURCE IN LISTS DEMO_SOURCES)
    file(RELATIVE_PATH DEMO_RELATIVE_PATH "${CMAKE_SOURCE_DIR}" "${DEMO_SOURCE}")
    string(SHA1 DEMO_HASH "${DEMO_RELATIVE_PATH}")
    string(SUBSTRING "${DEMO_HASH}" 0 8 DEMO_HASH_SHORT)
    set(DEMO_TARGET "demo_${DEMO_HASH_SHORT}")

    add_executable("${DEMO_TARGET}" "${DEMO_SOURCE}")

    target_include_directories("${DEMO_TARGET}" PRIVATE
        "${CMAKE_SOURCE_DIR}/include"
    )

    target_link_libraries("${DEMO_TARGET}" PRIVATE Microsoft.GSL::GSL)

    if(MSVC)
        target_compile_options("${DEMO_TARGET}" PRIVATE /utf-8)
    endif()

    set_target_properties("${DEMO_TARGET}" PROPERTIES
        OUTPUT_NAME "${DEMO_TARGET}"
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
        RUNTIME_OUTPUT_DIRECTORY_DEBUG "${CMAKE_BINARY_DIR}/bin"
        RUNTIME_OUTPUT_DIRECTORY_RELEASE "${CMAKE_BINARY_DIR}/bin"
        RUNTIME_OUTPUT_DIRECTORY_RELWITHDEBINFO "${CMAKE_BINARY_DIR}/bin"
        RUNTIME_OUTPUT_DIRECTORY_MINSIZEREL "${CMAKE_BINARY_DIR}/bin"
        VS_DEBUGGER_WORKING_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
    )

    message(STATUS "Demo: ${DEMO_RELATIVE_PATH} -> target/exe: ${DEMO_TARGET}")
endforeach()
