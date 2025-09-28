use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::System;
use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem, PredefinedMenuItem}};
use winit::event_loop::{EventLoop, ControlFlow};
use gtk;

fn main() {
    gtk::init().unwrap();

    println!("CPUsage starting... Look for the ⚡ icon in your system tray.");
    println!("CPU usage will update every second.");
    println!("Press Ctrl+C to quit.");

    let event_loop = EventLoop::new().unwrap();
    let tray_menu = Menu::new();

    let quit_item = MenuItem::new("Quit", true, None);

    tray_menu.append(&PredefinedMenuItem::separator()).unwrap();
    tray_menu.append(&quit_item).unwrap();

    let tray_icon = match TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_title("⚡ 0%")
        .build() {
        Ok(icon) => {
            println!("Tray icon created successfully.");
            println!("If you don't see it in your system tray, check:");
            println!("- Your desktop environment settings for system tray");
            println!("- If using GNOME, install 'AppIndicator and KStatusNotifierItem Support' extension");
            println!("- If using Unity, ensure the system tray is enabled in panel settings");
            println!("- Try right-clicking the top panel and looking for tray options");
            icon
        }
        Err(e) => {
            eprintln!("Failed to create tray icon: {:?}", e);
            eprintln!("This might be because your desktop environment doesn't support system tray icons.");
            eprintln!("Try running in a different desktop environment or check your tray settings.");
            std::process::exit(1);
        }
    };

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut system = System::new();
        loop {
            system.refresh_cpu_usage();
            thread::sleep(Duration::from_millis(100));
            let cpu_usage = system.global_cpu_usage();
            let _ = tx.send(cpu_usage);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mut last_cpu = 0.0;
    let last_print = Instant::now();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        if let Ok(cpu_usage) = rx.try_recv() {
            println!("CPU Usage: {:.1}%", cpu_usage);
            if (cpu_usage - last_cpu).abs() > 0.5 {
                let title = format!("⚡ {:.0}%", cpu_usage);
                tray_icon.set_title(Some(&title));
                last_cpu = cpu_usage;
            }
        }
    }).unwrap();
}
