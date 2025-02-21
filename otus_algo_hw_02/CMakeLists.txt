# CMake minimum version
set(PRJ_CMAKE_MIN_VERSION 3.5 CACHE INTERNAL "CMake version")
cmake_minimum_required(VERSION ${PRJ_CMAKE_MIN_VERSION})
# Project information
set(PRJ_NAME "happy_tickets" CACHE INTERNAL "Project name")
set(PRJ_VERSION_MAJOR "0" CACHE INTERNAL "Minor version")
set(PRJ_VERSION_MINOR "0" CACHE INTERNAL "Major version")
set(PRJ_VERSION_PATCH "2" CACHE INTERNAL "Patch version")
set(PRJ_DESCRIPTION "Otus Algo HW2 Happy tickets" CACHE INTERNAL "Project description")
set(PRJ_HOMEPAGE_URL "https://github.com/free0ON/otus_algo_HW_02" CACHE INTERNAL "Project URL")
set(EMAIL_CONTACT "dmitrii.lozovoi@yandex.ru" CACHE INTERNAL "Email contact")

project(${PRJ_NAME} VERSION ${PRJ_VERSION_MAJOR}.${PRJ_VERSION_MINOR}.${PRJ_VERSION_PATCH} DESCRIPTION "${PRJ_DESCRIPTION}" HOMEPAGE_URL "${PRJ_HOMEPAGE_URL}")
message("PROJECT_NAME: ${PROJECT_NAME} VERSION: ${PROJECT_VERSION}")
message("GOOGLE_TEST: ${GOOGLE_TEST}")
message("DESCRIPTION: ${PRJ_DESCRIPTION}")
message("HOMEPAGE_URL: ${PRJ_HOMEPAGE_URL}")
message("CMAKE_MINIMUM_REQUIRED_VERSION: ${CMAKE_MINIMUM_REQUIRED_VERSION}")


message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")

set(CMAKE_CXX_FLAGS "-O3 -Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g -Wall -Wextra")

configure_file(src/version.h.in version.h)
add_library(version src/version.cpp)
add_executable(${PRJ_NAME} src/main.cpp)
add_executable(test_all test/test_all.cpp)
set_target_properties(${PRJ_NAME}  version test_all PROPERTIES
        CXX_STANDARD 17
        CXX_STANDARD_REQUIRED ON
        )

target_include_directories(${PRJ_NAME}
        PUBLIC
        "${CMAKE_BINARY_DIR}"
        src
        )

target_include_directories(version
        PUBLIC "${CMAKE_BINARY_DIR}"
        )

target_include_directories(test_all
        PRIVATE
        src
        )

target_link_libraries(${PRJ_NAME}
        PRIVATE
        version
        )

find_package(Threads REQUIRED)
find_package(GTest REQUIRED)

target_link_libraries(
        test_all
        PUBLIC
        gtest
        gtest_main
        version
        ${CMAKE_THREAD_LIBS_INIT}
)

enable_testing()
include(GoogleTest)
# add_test(BrutTickets test_all)
add_test(FastTickets test_all)

if (MSVC)
    target_compile_options(${PRJ_NAME}  PRIVATE  /W4)
    target_compile_options(version PRIVATE  /W4)
    target_compile_options(test_all PRIVATE  /W4)

else()
    target_compile_options(${PRJ_NAME} PRIVATE
            -Wall -Wextra -pedantic -Werror)
    target_compile_options(version PRIVATE
            -Wall -Wextra -pedantic -Werror)
    target_compile_options(test_all PRIVATE
            -Wall -Wextra -pedantic -Werror)

endif(MSVC)

install(TARGETS ${PRJ_NAME} RUNTIME DESTINATION bin)
set(CPACK_GENERATOR DEB)
set(CPACK_PACKAGE_VERSION_MAJOR "${PRJ_VERSION_MAJOR}")
set(CPACK_PACKAGE_VERSION_MINOR "${PRJ_VERSION_MINOR}")
set(CPACK_PACKAGE_VERSION_PATCH "${PRJ_VERSION_PATCH}")

set(CPACK_PACKAGE_CONTACT "${EMAIL_CONTACT}")

include(CPack)