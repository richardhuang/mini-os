//! 简单文件系统模块

use lazy_static::lazy_static;
use spin::Mutex;
use alloc::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Program,
}

pub struct FileEntry {
    pub name: &'static str,
    pub file_type: FileType,
    pub content: &'static [u8],
}

pub struct SimpleFileSystem {
    files: BTreeMap<&'static str, FileEntry>,
}

impl SimpleFileSystem {
    pub const fn new() -> Self {
        SimpleFileSystem {
            files: BTreeMap::new(),
        }
    }

    pub fn register_file(&mut self, entry: FileEntry) {
        self.files.insert(entry.name, entry);
    }

    pub fn run_program(&self, name: &str) {
        if let Some(file) = self.files.get(name) {
            if file.file_type == FileType::Program {
                crate::println!("Running program: {}", file.name);
                if let Ok(text) = core::str::from_utf8(file.content) {
                    crate::println!("{}", text);
                }
            } else {
                crate::println!("Error: {} is not a program", name);
            }
        } else {
            crate::println!("Error: Program {} not found", name);
        }
    }
}

lazy_static! {
    static ref FS: Mutex<SimpleFileSystem> = Mutex::new(SimpleFileSystem::new());
}

pub fn init() {
    let mut fs = FS.lock();

    fs.register_file(FileEntry {
        name: "hello",
        file_type: FileType::Program,
        content: b"
================================
  Hello World!
================================

Welcome to Mini OS!

This is an experimental operating system
written in Rust.

Features:
  - x86_64 architecture support
  - VGA text output
  - Interrupt handling
  - Basic memory management
  - Simple file system

Thank you for using!
================================
",
    });

    crate::println!("[OK] File system initialized");
}

pub fn run_program(name: &str) {
    FS.lock().run_program(name);
}
