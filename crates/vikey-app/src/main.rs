#![windows_subsystem = "windows"]

use anyhow::Result;
use muda::{IconMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tray_icon::{Icon as TrayIcon, TrayIconBuilder};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowAttributes,
};

// Embed icons
const VIETNAMESE_ICON_BYTES: &[u8] = include_bytes!("../icons/vietnamese.png");
const ENGLISH_ICON_BYTES: &[u8] = include_bytes!("../icons/english.png");
const CHECK_ICON_BYTES: &[u8] = include_bytes!("../icons/check.png");

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    input_method: String, // "english", "telex", "vni"
    auto_correct: bool,
    shorthand: bool,
    startup: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            input_method: "english".to_string(),
            auto_correct: false,
            shorthand: false,
            startup: false,
        }
    }
}

/// Get the settings file path
/// 
/// Platform-specific paths:
/// - Windows: %APPDATA%\vikey\settings.toml
/// - macOS: ~/Library/Application Support/vikey/settings.toml
/// - Linux: ~/.config/vikey/settings.toml
fn get_settings_path() -> Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?;
    let vikey_dir = data_dir.join("vikey");
    fs::create_dir_all(&vikey_dir)?;
    Ok(vikey_dir.join("settings.toml"))
}

fn load_settings() -> Settings {
    match get_settings_path() {
        Ok(path) => {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(settings) = toml::from_str(&content) {
                        return settings;
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to get settings path: {:?}", e),
    }
    Settings::default()
}

fn save_settings(settings: &Settings) -> Result<()> {
    let path = get_settings_path()?;
    let content = toml::to_string_pretty(settings)?;
    fs::write(path, content)?;
    Ok(())
}

fn load_icon_from_bytes(bytes: &[u8]) -> Result<TrayIcon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(bytes)?
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Ok(TrayIcon::from_rgba(icon_rgba, icon_width, icon_height)?)
}

fn load_menu_icon(bytes: &[u8]) -> Option<muda::Icon> {
    let image = image::load_from_memory(bytes).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    muda::Icon::from_rgba(rgba, width, height).ok()
}

fn main() -> Result<()> {
    env_logger::init();

    // Single instance check
    let instance = single_instance::SingleInstance::new("vikey")
        .map_err(|e| anyhow::anyhow!("Failed to create single instance lock: {}", e))?;
    
    if !instance.is_single() {
        eprintln!("Another instance of Vikey is already running. Exiting.");
        std::process::exit(0);
    }

    // Load settings
    let mut settings = load_settings();
    println!("Loaded settings: {:?}", settings);

    let event_loop = EventLoop::new()?;
    
    // We need a hidden window for the event loop to work properly on some platforms/configs
    // and to handle menu events correctly in some cases.
    let window = event_loop.create_window(WindowAttributes::default().with_visible(false))?;

    // --- Menu Setup ---
    
    // 0. English (disable input method) - IconMenuItem
    let english_item = IconMenuItem::new(
        "English", 
        true, 
        if settings.input_method == "english" { load_menu_icon(CHECK_ICON_BYTES) } else { None },
        None
    );
    
    // 1. Chữ Việt submenu (enabled)
    let chu_viet_menu = Submenu::new("Chữ Việt", true);
    let telex_item = IconMenuItem::new(
        "Telex", 
        true, 
        if settings.input_method == "telex" { load_menu_icon(CHECK_ICON_BYTES) } else { None },
        None
    );
    let vni_item = IconMenuItem::new(
        "VNI", 
        true, 
        if settings.input_method == "vni" { load_menu_icon(CHECK_ICON_BYTES) } else { None },
        None
    );
    let viqr_item = MenuItem::new("VIQR", false, None); // disabled for now
    let _ = chu_viet_menu.append_items(&[&telex_item, &vni_item, &viqr_item]);
    
    // 2. Chữ Nôm submenu (disabled)
    let chu_nom_menu = Submenu::new("Chữ Nôm", true);
    let ky_am_item = MenuItem::new("Ký âm", false, None);
    let chiet_tu_item = MenuItem::new("Chiết tự", false, None);
    let _ = chu_nom_menu.append_items(&[&ky_am_item, &chiet_tu_item]);
    
    // 3. Other writing systems (disabled)
    let chu_tay_nguyen_item = MenuItem::new("Chữ Tây Nguyên", false, None);
    
    // 4. Tùy chọn submenu
    let tuy_chon_menu = Submenu::new("Tùy chọn", true);
    // Note: muda doesn't have CheckMenuItem, using regular MenuItem for now
    let auto_correct_item = MenuItem::new("Tự động sửa lỗi chính tả", false, None);
    let shorthand_item = MenuItem::new("Gõ tắt", false, None);
    let startup_item = MenuItem::new("Tự động khởi động", false, None);
    let chuyen_ma_item = MenuItem::new("Chuyển mã", true, None);
    let huong_dan_item = MenuItem::new("Hướng dẫn", true, None);
    let _ = tuy_chon_menu.append_items(&[&auto_correct_item, &shorthand_item, &startup_item]);
    
    // 5. Other items
    let thoat_item = MenuItem::new("Thoát", true, None);

    // Assemble menu
    let menu = Menu::new();
    let _ = menu.append_items(&[
        &english_item,
        &chu_viet_menu,
        &chu_nom_menu,
        &chu_tay_nguyen_item,
        &PredefinedMenuItem::separator(),
        &tuy_chon_menu,
        &PredefinedMenuItem::separator(),
        &huong_dan_item,
        &thoat_item,
    ]);

    // --- Tray Setup ---
    let vietnamese_icon = load_icon_from_bytes(VIETNAMESE_ICON_BYTES).unwrap_or_else(|_| {
        TrayIcon::from_rgba(vec![0, 0, 0, 0], 1, 1).unwrap()
    });
    
    let english_icon = load_icon_from_bytes(ENGLISH_ICON_BYTES).unwrap_or_else(|_| {
        TrayIcon::from_rgba(vec![0, 0, 0, 0], 1, 1).unwrap()
    });

    let initial_tooltip = if settings.input_method == "english" {
        "Vikey\nEnglish".to_string()
    } else {
        format!("Vikey\nChữ Việt\n{}", 
            if settings.input_method == "telex" { "TELEX" } else { "VNI" })
    };
    
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip(&initial_tooltip)
        .with_icon(if settings.input_method == "english" { 
            english_icon.clone() 
        } else { 
            vietnamese_icon.clone() 
        })
        .build()?;
    
    // Set submenu icon if Vietnamese input is selected
    if settings.input_method != "english" {
        if let Some(check_icon) = load_menu_icon(CHECK_ICON_BYTES) {
            let _ = chu_viet_menu.set_icon(Some(check_icon));
        }
    }

    // --- Vikey Engine Setup ---
    use vikey_core::engine::Engine;
    use vikey_vietnamese::VietnamesePlugin;
    
    let mut engine = Engine::new();
    if let Err(e) = engine.register(Box::new(VietnamesePlugin::new())) {
        eprintln!("Failed to register Vietnamese plugin: {:?}", e);
    }
    
    // Apply settings to engine
    if settings.input_method != "english" {
        if let Err(e) = engine.set_language("vietnamese") {
            eprintln!("Failed to set language: {:?}", e);
        }
        if let Err(e) = engine.set_input_method(&settings.input_method) {
            eprintln!("Failed to set input method: {:?}", e);
        }
    }

    // --- Event Loop ---
    let menu_channel = muda::MenuEvent::receiver();
    let _tray_channel = tray_icon::TrayIconEvent::receiver();
    let mut _is_enabled = settings.input_method != "english";
    let mut _current_method = settings.input_method.clone();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            
            Event::AboutToWait => {
                if let Ok(event) = menu_channel.try_recv() {
                    // Handle Thoát (Exit)
                    if event.id == thoat_item.id() {
                        elwt.exit();
                    } 
                    // Handle English (disable input method)
                    else if event.id == english_item.id() {
                        _is_enabled = false;
                        _current_method = "english".to_string();
                        println!("Switched to English (disabled)");
                        
                        // Update settings
                        settings.input_method = "english".to_string();
                        if let Err(e) = save_settings(&settings) {
                            eprintln!("Failed to save settings: {:?}", e);
                        }
                        
                        // Update menu icons
                        if let Some(check_icon) = load_menu_icon(CHECK_ICON_BYTES) {
                            let _ = english_item.set_icon(Some(check_icon)); // Show check
                        }
                        let _ = telex_item.set_icon(None); // Remove check
                        let _ = vni_item.set_icon(None); // Remove check
                        let _ = chu_viet_menu.set_icon(None); // Remove icon from submenu
                        
                        // Update icon and tooltip
                        let _ = _tray_icon.set_icon(Some(english_icon.clone()));
                        let _ = _tray_icon.set_tooltip(Some("Vikey\nOFF".to_string()));
                    }
                    // Handle Telex
                    else if event.id == telex_item.id() {
                        _is_enabled = true;
                        _current_method = "telex".to_string();
                        println!("Switched to Telex");
                        
                        // Update settings
                        settings.input_method = "telex".to_string();
                        if let Err(e) = save_settings(&settings) {
                            eprintln!("Failed to save settings: {:?}", e);
                        }
                        
                        // Update engine
                        if let Err(e) = engine.set_language("vietnamese") {
                            eprintln!("Failed to set language: {:?}", e);
                        }
                        if let Err(e) = engine.set_input_method("telex") {
                            eprintln!("Failed to set input method: {:?}", e);
                        }
                        
                        // Update menu icons
                        let _ = english_item.set_icon(None); // Remove check
                        if let Some(check_icon) = load_menu_icon(CHECK_ICON_BYTES) {
                            let _ = telex_item.set_icon(Some(check_icon.clone())); // Show check
                            let _ = chu_viet_menu.set_icon(Some(check_icon)); // Add check icon to submenu
                        }
                        let _ = vni_item.set_icon(None); // Remove check
                        
                        // Update icon and tooltip
                        let _ = _tray_icon.set_icon(Some(vietnamese_icon.clone()));
                        let _ = _tray_icon.set_tooltip(Some("Vikey\nChữ Việt\nTELEX".to_string()));
                    } 
                    // Handle VNI
                    else if event.id == vni_item.id() {
                        _is_enabled = true;
                        _current_method = "vni".to_string();
                        println!("Switched to VNI");
                        
                        // Update settings
                        settings.input_method = "vni".to_string();
                        if let Err(e) = save_settings(&settings) {
                            eprintln!("Failed to save settings: {:?}", e);
                        }
                        
                        // Update engine
                        if let Err(e) = engine.set_language("vietnamese") {
                            eprintln!("Failed to set language: {:?}", e);
                        }
                        if let Err(e) = engine.set_input_method("vni") {
                            eprintln!("Failed to set input method: {:?}", e);
                        }
                        
                        // Update menu icons
                        let _ = english_item.set_icon(None); // Remove check
                        let _ = telex_item.set_icon(None); // Remove check
                        if let Some(check_icon) = load_menu_icon(CHECK_ICON_BYTES) {
                            let _ = vni_item.set_icon(Some(check_icon.clone())); // Show check
                            let _ = chu_viet_menu.set_icon(Some(check_icon)); // Add check icon to submenu
                        }
                        
                        // Update icon and tooltip
                        let _ = _tray_icon.set_icon(Some(vietnamese_icon.clone()));
                        let _ = _tray_icon.set_tooltip(Some("Vikey\nChữ Việt\nVNI".to_string()));
                    }
                    // Handle Hướng dẫn
                    else if event.id == huong_dan_item.id() {
                        println!("Opening documentation...");
                        // TODO: Open README.md or documentation URL
                    }
                }
            }
            _ => {}
        }
    })?;

    Ok(())
}
