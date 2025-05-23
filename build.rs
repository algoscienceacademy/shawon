use std::process::Command;
// Removed the unused import
use std::path::Path;

fn main() {
    // Path to the directory containing our C++ code
    let cpp_dir = "src/cpp";
    
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(cpp_dir).unwrap();
    
    // Write the C++ wrapper file
    std::fs::write(
        Path::new(cpp_dir).join("qt_wrapper.cpp"),
        include_str!("src/cpp/qt_wrapper.cpp.in")
    ).unwrap();
    
    // Find Qt installation
    let qmake_output = Command::new("qmake")
        .arg("--version")
        .output()
        .expect("qmake must be installed and in PATH");
    
    println!("cargo:warning=Found Qt: {}", String::from_utf8_lossy(&qmake_output.stdout));
    
    // Get Qt include and lib paths
    let qt_include = Command::new("qmake")
        .arg("-query")
        .arg("QT_INSTALL_HEADERS")
        .output()
        .expect("qmake failed")
        .stdout;
    
    let qt_libs = Command::new("qmake")
        .arg("-query")
        .arg("QT_INSTALL_LIBS")
        .output()
        .expect("qmake failed")
        .stdout;
    
    let qt_include = String::from_utf8_lossy(&qt_include).trim().to_string();
    let qt_libs = String::from_utf8_lossy(&qt_libs).trim().to_string();
    
    // Build our C++ wrapper
    let mut build = cc::Build::new();
    
    build
        .cpp(true)
        .file("src/cpp/qt_wrapper.cpp")
        .file("src/cpp/button.cpp")
        .file("src/cpp/graphics_item.cpp")
        .file("src/cpp/graphics_scene.cpp")
        .file("src/cpp/graphics_view.cpp")
        .file("src/cpp/widget_utils.cpp") // Add our new file
        .include(&qt_include)
        .include(format!("{}/QtWidgets", qt_include))
        .include(format!("{}/QtCore", qt_include))
        .include(format!("{}/QtGui", qt_include))
        .flag("-std=c++11");
    
    // Add platform-specific flags
    if cfg!(target_os = "windows") {
        // Check if we're using MSVC or MinGW
        let target = std::env::var("TARGET").unwrap_or_default();
        if target.contains("msvc") {
            build.flag("-EHsc");
        } else {
            // MinGW equivalent flags
            build.flag("-fexceptions");
            build.flag("-fnon-call-exceptions");
        }
    } else if cfg!(target_os = "linux") {
        // Linux-specific flags
        build.flag("-fPIC");
        
        // On Linux, we might need additional include paths for X11
        build.include("/usr/include/x11");
        build.include("/usr/include/gtk-3.0");
    } else if cfg!(target_os = "macos") {
        // macOS-specific flags
        build.flag("-fPIC");
        build.flag("-F/Library/Frameworks");
        build.flag("-framework");
        build.flag("Cocoa");
        
        // Add macOS framework paths
        println!("cargo:rustc-link-arg=-framework");
        println!("cargo:rustc-link-arg=Cocoa");
    }
    
    build.compile("shawon");
    
    // Link against Qt libraries
    println!("cargo:rustc-link-search={}", qt_libs);
    println!("cargo:rustc-link-lib=Qt5Widgets");
    println!("cargo:rustc-link-lib=Qt5Gui");
    println!("cargo:rustc-link-lib=Qt5Core");
    
    // Additional dependencies on Windows
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=User32");
        println!("cargo:rustc-link-lib=Gdi32");
        println!("cargo:rustc-link-lib=Shell32");
    }
    
    println!("cargo:rerun-if-changed=src/cpp/qt_wrapper.cpp.in");
    println!("cargo:rerun-if-changed=src/cpp/qt_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/cpp/button.cpp");
    println!("cargo:rerun-if-changed=src/cpp/graphics_item.cpp");
    println!("cargo:rerun-if-changed=src/cpp/graphics_scene.cpp");
    println!("cargo:rerun-if-changed=src/cpp/graphics_view.cpp");
    println!("cargo:rerun-if-changed=src/cpp/widget_utils.cpp"); // Add our new file
    println!("cargo:rerun-if-changed=build.rs");
}
