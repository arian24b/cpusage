use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyAccessory, NSMenu, NSMenuItem, NSStatusBar, NSVariableStatusItemLength};
use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc::{msg_send, sel, sel_impl};
use sysinfo::System;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);

        // Create status item
        let status_bar = NSStatusBar::systemStatusBar(nil);
        let status_item = status_bar.statusItemWithLength_(NSVariableStatusItemLength);

        // Create a simple CPU icon using system symbols or fallback to emoji
        let button: id = msg_send![status_item, button];

        // Use a simple emoji icon for now
        let _: () = msg_send![button, setTitle: NSString::alloc(nil).init_str("⚡")];

        // Create menu with more options
        let menu = NSMenu::new(nil).autorelease();

        // CPU usage display item (disabled)
        let cpu_display_item = NSMenuItem::new(nil)
            .initWithTitle_action_keyEquivalent_(
                NSString::alloc(nil).init_str("CPU: 0%"),
                sel!(dummy:),
                NSString::alloc(nil).init_str("")
            );
        let _: () = msg_send![cpu_display_item, setEnabled: NO];
        menu.addItem_(cpu_display_item);

        // Separator
        let separator = NSMenuItem::separatorItem(nil);
        menu.addItem_(separator);

        // Quit item
        let quit_item = NSMenuItem::new(nil)
            .initWithTitle_action_keyEquivalent_(
                NSString::alloc(nil).init_str("Quit"),
                sel!(terminate:),
                NSString::alloc(nil).init_str("q")
            );
        menu.addItem_(quit_item);
        let _: () = msg_send![status_item, setMenu: menu];

        // Create channel for CPU updates
        let (tx, rx) = mpsc::channel();

        // Store references for updates
        let cpu_display_item_ptr = cpu_display_item as *mut objc::runtime::Object;

        // Background thread for CPU monitoring - optimized
        thread::spawn(move || {
            // Initialize system with minimal components for lower memory usage
            let mut system = System::new();
            let mut last_update = Instant::now();

            loop {
                // Only refresh CPU info, not all system info
                system.refresh_cpu_usage();

                // Wait a bit for CPU measurement to stabilize
                thread::sleep(Duration::from_millis(100));

                let cpu_usage = system.global_cpu_usage();

                // Only send updates if enough time has passed (reduce channel overhead)
                if last_update.elapsed() >= Duration::from_secs(3) {
                    let _ = tx.send(cpu_usage);
                    last_update = Instant::now();
                }

                // Longer sleep to reduce CPU usage of monitoring thread
                thread::sleep(Duration::from_secs(5));
            }
        });

        // Optimized run loop with proper NSApp integration
        let mut last_cpu_value = -1.0; // Cache last value to avoid unnecessary updates

        loop {
            let pool = NSAutoreleasePool::new(nil);

            // Check for CPU updates (non-blocking)
            if let Ok(cpu_usage) = rx.try_recv() {
                // Only update UI if value changed significantly (reduce UI updates)
                if (cpu_usage - last_cpu_value).abs() > 0.5 {
                    let title = format!("{:.0}%", cpu_usage);
                    let menu_title = format!("CPU: {:.0}%", cpu_usage);

                    // Update button with emoji and percentage
                    let button: id = msg_send![status_item, button];
                    let ns_title = NSString::alloc(nil).init_str(&format!("⚡{}", title));
                    let _: () = msg_send![button, setTitle: ns_title];

                    // Update menu item
                    let menu_ns_title = NSString::alloc(nil).init_str(&menu_title);
                    let _: () = msg_send![cpu_display_item_ptr, setTitle: menu_ns_title];

                    last_cpu_value = cpu_usage;
                }
            }

            // Process events more efficiently with timeout
            let event: id = msg_send![app, nextEventMatchingMask:-1i32
                                      untilDate:nil
                                      inMode:cocoa::foundation::NSDefaultRunLoopMode
                                      dequeue:1u8];
            if event != nil {
                let _: () = msg_send![app, sendEvent: event];
            }

            pool.drain();

            // Longer sleep to reduce main thread CPU usage
            thread::sleep(Duration::from_millis(500));
        }
    }
}
