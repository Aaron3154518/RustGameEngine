# Compiler variables
CC := x86_64-w64-mingw32-gcc
CFLAGS := -Wall -Wextra -pedantic -std=c99

a := sdl/SDL2-devel-2.26.5-VC/SDL2-2.26.5
b := sdl/SDL2_image-devel-2.6.3-VC/SDL2_image-2.6.3

# Library variables
SDL_LIB := $(a)/lib/x64
SDL_INC := $(a)/include
SDL_IMG_LIB := $(b)/lib/x64
SDL_IMG_INC := $(b)/include

# Linker flags
INCFLAGS := -I$(SDL_INC) -I$(SDL_IMG_INC)
LDFLAGS := -L$(SDL_LIB) -L$(SDL_IMG_LIB) -lmingw32 -lSDL2main -lSDL2_image -lSDL2

# Build directory
BUILD_DIR := build

# Main executable
TARGET := $(BUILD_DIR)/main

# Source files
SRC := $(wildcard *.c)

# Object files
OBJ := $(SRC:%.c=$(BUILD_DIR)/%.o)

# Create build directory
$(shell mkdir -p $(BUILD_DIR))

$(info SDL2 Include Directory: $(INCFLAGS))
$(info SDL2 Library Directory: $(LDFLAGS))

$(info SDL2_Image Include Directory: $(b)/include/SDL2)
$(info SDL2_Image Library Directory: $(b)/lib/x64)

# Link object files to create executable
$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)

# Compile source files into object files
$(BUILD_DIR)/%.o: %.c
	$(CC) $(CFLAGS) $(INCFLAGS) -c $< -o $@

# Remove object files and executable
clean:
	rm -rf $(BUILD_DIR)

# Build and run the program
run: $(TARGET)
	./$(TARGET)
