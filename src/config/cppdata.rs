pub const HEADER_FILE: &str = r#"#ifndef {project_name_upper}_H
#define {project_name_upper}_H

#include <iostream>

/**
 * @brief Example header file for {project_name}
 * 
 * This is a sample header file demonstrating how to organize
 * your project's header files in the include/ directory.
 */

// Inline function definition (must be in header)
inline void print_message(const char* message) {
    std::cout << "Message: " << message << std::endl;
}

// Template function example (must be in header)
template<typename T>
T add(T a, T b) {
    return a + b;
}

#endif // {project_name_upper}_H
"#;

pub const MAIN_FILE: &str = r#"#include <iostream>
#include "{project_name}.hpp"

int main() 
{
    std::cout << "Hello, World!" << std::endl;
    
    // Use template function from header
    int result = add(3, 5);
    std::cout << "3 + 5 = " << result << std::endl;
    
    print_message("Welcome to CPM!");
    
    return 0;
}
"#;

pub const CPM_FILE: &str = r#"
[project]
name = "{project_name}"
version = "0.1.0"
description = "A C/C++ project built with CPM"
authors = ["Your Name <your_email@example.com>"]
license = "MIT"

[build]
output = "bin"
compiler = "g++"
flags = ["-Wall", "-Wextra", "-std=c++17"]
system_libraries = ["z", "pthread"]

[dependencies]

[dev_dependencies]

"#;

pub const GITIGNORE_FILE: &str = r#"build/
dist/
modules/
"#;

pub const README_FILE: &str = r#"# {project_name}
This is a simple C++ project created with CPM (C Package Manager).

## Building
To build this project, run:
```bash
cpm build
```

## Features
- Basic mathematical operations (add, subtract, multiply, divide)
- Template-based generic programming
- Modern C++17 standard

## Dependencies
This project uses standard C++ libraries only.
"#;
