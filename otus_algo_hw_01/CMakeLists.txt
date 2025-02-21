cmake_minimum_required(VERSION 3.2)
set(PRJ_NAME "otus_algo_hw_01" CACHE INTERNAL "Project name")
set(PRJ_VERSION_MAJOR "0" CACHE INTERNAL "Minor version")
set(PRJ_VERSION_MINOR "1" CACHE INTERNAL "Major version")
set(PRJ_VERSION_PATCH "1" CACHE INTERNAL "Patch version")
set(PRJ_DESCRIPTION "Allocator template class" CACHE INTERNAL "Project description")
set(PRJ_HOMEPAGE_URL "https://github.com/free0ON/otus_cpp_HW_03" CACHE INTERNAL "Project URL")
set(EMAIL_CONTACT "dmitrii.lozovoi@yandex.ru" CACHE INTERNAL "Email contact")

# file( GLOB SRC RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} *.cpp )
# message(STATUS "Found source files: " "${SRC} ")
project(otus_algo_hw_01)

# foreach( FILE ${SRC} )
#     string( REPLACE ".cpp" "" TARGET ${FILE})
#     add_executable( ${TARGET} ${FILE})
#     set_target_properties(${TARGET} PROPERTIES CXX_STANDARD 17 CXX_STANDARD_REQUIRED ON)

#     install(TARGETS ${TARGET} RUNTIME DESTINATION bin)
#     set(CPACK_GENERATOR DEB)
#     set(CPACK_PACKAGE_VERSION_MAJOR "${PRJ_VERSION_MAJOR}")
#     set(CPACK_PACKAGE_VERSION_MINOR "${PRJ_VERSION_MINOR}")
#     set(CPACK_PACKAGE_VERSION_PATCH "${PRJ_VERSION_PATCH}")
#     set(CPACK_PACKAGE_CONTACT "${EMAIL_CONTACT}")
# endforeach( FILE ${SRC} )
# include(CPack)


install(TARGETS ${PRJ_NAME} RUNTIME DESTINATION bin)
set(CPACK_ABSOLUTE_DESTINATION_FILES ${PROJECT_SOURCE_DIR})
set(CPACK_GENERATOR DEB)
set(CPACK_PACKAGE_VERSION_MAJOR "${PRJ_VERSION_MAJOR}")
set(CPACK_PACKAGE_VERSION_MINOR "${PRJ_VERSION_MINOR}")
set(CPACK_PACKAGE_VERSION_PATCH "${PRJ_VERSION_PATCH}")

set(CPACK_PACKAGE_CONTACT "${EMAIL_CONTACT}")

include(CPack)