# CMAKE generated file: DO NOT EDIT!
# Generated by "MinGW Makefiles" Generator, CMake Version 3.19

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Disable VCS-based implicit rules.
% : %,v


# Disable VCS-based implicit rules.
% : RCS/%


# Disable VCS-based implicit rules.
% : RCS/%,v


# Disable VCS-based implicit rules.
% : SCCS/s.%


# Disable VCS-based implicit rules.
% : s.%


.SUFFIXES: .hpux_make_needs_suffix_list


# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

SHELL = cmd.exe

# The CMake executable.
CMAKE_COMMAND = C:\cmake\bin\cmake.exe

# The command to remove a file.
RM = C:\cmake\bin\cmake.exe -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = "E:\playground\Exercism Solutions\cpp\series"

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = "E:\playground\Exercism Solutions\cpp\series\build"

# Utility rule file for test_series.

# Include the progress variables for this target.
include CMakeFiles/test_series.dir/progress.make

CMakeFiles/test_series: series.exe
	.\series.exe

test_series: CMakeFiles/test_series
test_series: CMakeFiles/test_series.dir/build.make

.PHONY : test_series

# Rule to build all files generated by this target.
CMakeFiles/test_series.dir/build: test_series

.PHONY : CMakeFiles/test_series.dir/build

CMakeFiles/test_series.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles\test_series.dir\cmake_clean.cmake
.PHONY : CMakeFiles/test_series.dir/clean

CMakeFiles/test_series.dir/depend:
	$(CMAKE_COMMAND) -E cmake_depends "MinGW Makefiles" "E:\playground\Exercism Solutions\cpp\series" "E:\playground\Exercism Solutions\cpp\series" "E:\playground\Exercism Solutions\cpp\series\build" "E:\playground\Exercism Solutions\cpp\series\build" "E:\playground\Exercism Solutions\cpp\series\build\CMakeFiles\test_series.dir\DependInfo.cmake" --color=$(COLOR)
.PHONY : CMakeFiles/test_series.dir/depend

