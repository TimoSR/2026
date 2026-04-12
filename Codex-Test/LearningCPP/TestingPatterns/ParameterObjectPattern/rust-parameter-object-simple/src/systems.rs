use std::collections::HashMap;

use crate::config::{
    AppConfig, AudioConfig, DiagnosticsConfig, DisplayConfig, RenderConfig, WindowConfig,
};

#[derive(Debug)]
pub struct Graphics {
    display: DisplayConfig,
    diagnostics: DiagnosticsConfig,
    render: RenderConfig,
    running: bool,
    images: HashMap<String, String>,
}

impl Graphics {
    pub fn create(config: &AppConfig) -> Self {
        let graphics = Self {
            display: config.display.clone(),
            diagnostics: config.diagnostics.clone(),
            render: config.render.clone(),
            running: true,
            images: HashMap::new(),
        };
        println!(
            "[Graphics] Created ({}, {}, diagnostics={})",
            graphics.display.summary(),
            graphics.render.summary(),
            graphics.diagnostics.summary()
        );
        graphics
    }

    pub fn create_default() -> Self {
        Self::create(&AppConfig::default())
    }

    pub fn update_config(&mut self, config: &AppConfig) {
        self.display = config.display.clone();
        self.diagnostics = config.diagnostics.clone();
        self.render = config.render.clone();
        println!(
            "[Graphics] Updated ({}, {}, diagnostics={})",
            self.display.summary(),
            self.render.summary(),
            self.diagnostics.summary()
        );
    }

    pub fn load_image(&mut self, name: &str, file_path: &str) {
        self.images.insert(name.to_string(), file_path.to_string());
        println!("[Graphics] Loaded image '{name}' from '{file_path}'");
    }

    pub fn draw_text(&self, text: &str, x: i32, y: i32) {
        if self.running {
            println!("[Graphics] Draw text '{text}' at ({x}, {y})");
        }
    }

    pub fn draw_image(&self, name: &str, x: i32, y: i32) {
        if self.running {
            if let Some(file_path) = self.images.get(name) {
                println!("[Graphics] Draw image '{name}' ({file_path}) at ({x}, {y})");
            } else {
                println!("[Graphics] Missing image '{name}' at ({x}, {y})");
            }
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
        println!("[Graphics] Stopped");
    }
}

#[derive(Debug)]
pub struct Audio {
    diagnostics: DiagnosticsConfig,
    config: AudioConfig,
    running: bool,
}

impl Audio {
    pub fn create(config: &AppConfig) -> Self {
        let audio = Self {
            diagnostics: config.diagnostics.clone(),
            config: config.audio.clone(),
            running: true,
        };
        println!(
            "[Audio] Created ({}, diagnostics={})",
            audio.config.summary(),
            audio.diagnostics.summary()
        );
        audio
    }

    pub fn create_default() -> Self {
        Self::create(&AppConfig::default())
    }

    pub fn update_config(&mut self, config: &AppConfig) {
        self.diagnostics = config.diagnostics.clone();
        self.config = config.audio.clone();
        println!(
            "[Audio] Updated ({}, diagnostics={})",
            self.config.summary(),
            self.diagnostics.summary()
        );
    }

    pub fn play_sound(&self, file_path: &str) {
        if self.running {
            println!("[Audio] Playing '{file_path}'");
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
        println!("[Audio] Stopped");
    }
}

#[derive(Debug)]
pub struct Window {
    config: WindowConfig,
    open: bool,
}

impl Window {
    pub fn create(config: &AppConfig) -> Self {
        let window = Self {
            config: config.window.clone(),
            open: true,
        };
        println!("[Window] Opened ({})", window.config.summary());
        window
    }

    pub fn create_default() -> Self {
        Self::create(&AppConfig::default())
    }

    pub fn update_config(&mut self, config: &AppConfig) {
        self.config = config.window.clone();
        println!("[Window] Updated ({})", self.config.summary());
    }

    pub fn close(&mut self) {
        self.open = false;
        println!("[Window] Closed");
    }
}
