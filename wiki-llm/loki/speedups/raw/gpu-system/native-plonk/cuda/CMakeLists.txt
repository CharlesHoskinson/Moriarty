cmake_minimum_required(VERSION 3.24)

project(midnight_native_plonk LANGUAGES CXX CUDA)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CUDA_STANDARD 14)
set(CMAKE_CUDA_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_CUDA_RUNTIME_LIBRARY Shared)

if(NOT DEFINED CMAKE_CUDA_ARCHITECTURES)
    set(CMAKE_CUDA_ARCHITECTURES 75;80;86;89;90;120)
endif()

find_package(Threads REQUIRED)

add_library(midnight_native_plonk SHARED
    src/mn_prover.cu
    src/plonk_prover.cu
    src/field.cu
    src/curve.cu
    src/msm.cu
    src/fft.cu
    src/params.cu
    src/preprocess.cu
    src/test_api.cu
)

target_include_directories(midnight_native_plonk PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_compile_options(midnight_native_plonk PRIVATE
    $<$<COMPILE_LANGUAGE:CUDA>:-Wno-deprecated-gpu-targets>
)

set_target_properties(midnight_native_plonk PROPERTIES
    OUTPUT_NAME "midnight_native_plonk"
)

target_link_libraries(midnight_native_plonk PRIVATE Threads::Threads)
