extern crate gfx;
extern crate glfw;
extern crate libc;
extern crate gl;

use glfw::{Action, Context, Key, WindowHint, OpenGlProfileHint};
use glfw::ffi::{glfwGetPrimaryMonitor, glfwGetMonitorName, glfwGetVideoMode};
use std::slice;

// Convert a pointer to a string
fn ptr_to_string(ptr: *const i8) -> String {
    // Check if the pointer is null
    if ptr.is_null() {
        // Return an empty string
        return String::new();
    }
    // Get the length of the string
    let len = unsafe { libc::strlen(ptr) as usize } ;
    // Convert the pointer to a slice
    let slice = unsafe { slice::from_raw_parts(ptr as *const u8, len) };
    // From UTF-8 lossy is used to convert the slice to a string
    return String::from_utf8_lossy(slice).into_owned()
}

// Get the primary monitor
fn get_primary_monitor() -> *mut glfw::ffi::GLFWmonitor {
    // Wrap the unsafe function in a safe function
    return unsafe { glfwGetPrimaryMonitor() }
}

fn get_video_mode(monitor: *mut glfw::ffi::GLFWmonitor) -> *const glfw::ffi::GLFWvidmode {
    // Wrap the unsafe function in a safe function
    return unsafe { glfwGetVideoMode(monitor) }
}

fn get_monitor_name(monitor: *mut glfw::ffi::GLFWmonitor) -> String {
    // Wrap the unsafe function in a safe function
    let monitor_name = unsafe { glfwGetMonitorName(monitor) };
    // let monitor_name = monitor_name.to_string_lossy().into_owned();
    let monitor_name = ptr_to_string(monitor_name);
    // Return the monitor name 
    return monitor_name
}

fn set_sigint_handler() {
    // Register a signal handler for keyboard interrupts
    ctrlc::set_handler(move || {
        println!("Keyboard interrupt detected, exiting gracefully...");
        std::process::exit(0);
    }).expect("Error setting signal handler");
}

fn main() {
    // Register a signal handler for keyboard interrupts
    set_sigint_handler();

    // Initialize the GLFW window
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Set the window hints
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::Resizable(true));
    glfw.window_hint(WindowHint::Decorated(true));
    glfw.window_hint(WindowHint::Samples(Some(4)));

    // Return the primary monitor pointer 
    let monitor = get_primary_monitor(); 
    // Get the monitor name
    let monitor_name = get_monitor_name(monitor);
    println!("Monitor name: {}", monitor_name);
    // Get the video mode
    let video_mode = get_video_mode(monitor);
    println!("Video mode: {:?}", video_mode);
    // Create a raw pointer to the video mode
    let ptr = video_mode as *const glfw::ffi::GLFWvidmode;
    // Get the width and height of the video mode using the raw pointer
    let (width, height) = unsafe { ((*ptr).width, (*ptr).height) };
    // Convert the width and height to u32
    let (width, height) = (width as u32, height as u32);
    println!("Width: {}, Height: {}", width, height);

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(width, height, "Max Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");


    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load the OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Set the clear color
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    // loop until the user closes the window
    while !window.should_close() {

        // Poll for and process events
        glfw.poll_events();

        // Render here
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // Swap front and back buffers
        window.swap_buffers();

        for (_, event) in glfw::flush_messages(&events) {
            // Handle events
            match event {
                // Resize the window
                glfw::WindowEvent::FramebufferSize(_width, _height) => {
                }
                // Close the window
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}