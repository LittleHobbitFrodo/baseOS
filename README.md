# BaseOS

BaseOS is a template operating system. It offers a simple build and project management utility and C and Rust support with basic features.

# Main Goal
The goal of this project is to create simple platform to offer all features needed to build from scratch. That means:
  - Bootloader API
  - Allocator
  - Thread-safe primitives (Mutex, RwLock, ... - rust only (?) )
  - Containers (vector, hashmap, safe pointers, etc. )
  - Cells (RefCell, etc. - rust only )

In short, this project is a template for the operating system, not the operating system itself.


# What is supported?
CPU architectures:
  - [x] x86_64
  - [ ] arm64


Development platforms:
  - [ ] Fedora Linux
  - [ ] Ubuntu Linux
  - [ ] Arch linux and derivates
  - [x] MacOS with Apple Sillicon chips (almost supported)



Features presented in kernel:
  - [x] **Renderer** - Displays text on screen, supports various formats
    - C and Rust
  - [x] **Allocator** - heap implementation
    - Rust
  - [x] **BL API** - Ask bootloader for additional information
    - C and Rust
    - Thanks to Jason Youngberg for his *limine-rs* crate
  - [x] **Thread-safe Primitives** - ensures thread safety
    - using spin
  - [ ] **Containers** - vector, hash map, safe pointers and other primitives
    - None yet
  - [ ] **Cells** - Sharable mutable containers
    - None yet



# The utility
The *util* utility is a script written in bash that simplifies project management and building. It offers the following features:
  - [x] **info** - prints information about your project - number of lines of code, language, etc.
  - [x] **setup** - configures the project and sets the language and architecture
  - [x] **build** - builds the OS
  - [ ] **test** - tests kernel (rust only)
  - [x] **run** - runs the operating system in an emulator (Qemu)
  - [ ] **dependencies** - installs the tools needed to build the project and/or run the emulator on your computer.
  - [ ] **change** - renames the operating system or kernel or changes the project configuration
  - [x] **help** - displays information about the utility or its subcommands


# M.I.T. Licence
This project is under the M.I.T. Licence


That means the software is free (free of charge)  
You can redistribute this software in source or binary form  
You can redistribute modified versions of this software  
You can sublicence this software freely

I (@LittleHobbitFrodo) am not obliged to bear any consequences caused by this software or its possible malfunction

### Post Script
It would be nice if you mentioned that your project is based on my BaseOS :)
