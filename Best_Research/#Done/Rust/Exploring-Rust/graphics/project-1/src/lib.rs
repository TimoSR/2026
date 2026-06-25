#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{EventLoop, EventLoopWindowTarget}, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder
};

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }    

    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).expect("Windowbuilder failed");

    // event_loop.run(move |event: Event<()>, control_flow: &EventLoopWindowTarget<()>| {
    //     match event {

    //         Event::WindowEvent 
    //         {
    //             ref event, 
    //             window_id,
    //         } 
            
    //         if window_id == window.id() => match event 
    //         {
    //             WindowEvent::CloseRequested | WindowEvent::KeyboardInput 
    //             {
    //                 event: KeyEvent 
    //                 { 
    //                     state: ElementState::Pressed, 
    //                     physical_key: PhysicalKey::Code(KeyCode::Escape), 
    //                     ..
    //                 },
    //                 ..
    //             } => control_flow.exit(),
    //             _ => {}
    //         },
    //         _ => {}
    //     }
    // });


    // let _ = event_loop.run(move |event: Event<()>, control_flow: &EventLoopWindowTarget<()>| {
    //     match event {
    //            //Seems like it is destructoring going
    //         Event::WindowEvent {
    //             window_id,
    //             event: ref inner_event, // 'inner_event' will be a reference to the WindowEvent enum,
    //                                     // similar to 'ref event' in your original nested match.
    //         } 
            
    //         if window_id == window.id() => { // Ensure 'window_id' is the correct type for comparison.
    //                                           // If 'event' is a reference, 'window_id' might be &WindowId.
    //                                           // If so, compare with '*window_id == window.id()'.
    //                                           // Assuming 'window_id' from the pattern is `winit::window::WindowId` (which is Copy)
    //                                           // and `window.id()` returns `winit::window::WindowId`.
        
    //             if matches!(inner_event,
    //                 // First condition: CloseRequested
    //                 WindowEvent::CloseRequested |
    //                 // Second condition: Specific KeyboardInput
    //                 WindowEvent::KeyboardInput {
    //                     // 'event:' here is the field name within WindowEvent::KeyboardInput
    //                     event: KeyEvent {
    //                         state: ElementState::Pressed,
    //                         physical_key: PhysicalKey::Code(KeyCode::Escape),
    //                         .. // Use '..' to ignore other fields in KeyEvent if it's non-exhaustive or has more fields
    //                     },
    //                     .. // Use '..' to ignore other fields in WindowEvent::KeyboardInput (like device_id, is_synthetic)
    //                 }
    //             ) {
    //                 control_flow.exit();
    //             }
    //             // The `_ => {}` from your inner match is implicitly handled: if the `if matches!`
    //             // condition is false, no action is taken within this arm for that specific WindowEvent.
    //         }
    //         // This outer `_ => {}` handles all other `Event` variants or `WindowEvent`s
    //         // that didn't match the `window_id` guard.
    //         _ => {}
    //     }
    // });

    let _ = event_loop.run(move |event: Event<()>, control_flow: &EventLoopWindowTarget<()>| {
        match event 
        {
            Event::WindowEvent {
                window_id,
                event: ref inner_event,
            }
             
            if window_id == window.id() => 
            {
                if matches!(
                    inner_event,
                    WindowEvent::CloseRequested 
                    |
                    WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                        ..
                    }) { control_flow.exit(); }
            }
            _ => {}
        }
    });
}