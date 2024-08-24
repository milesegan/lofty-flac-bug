use std::{
    fs::{copy, remove_file},
    path::Path,
    process::Command,
};

use lofty::{
    config::WriteOptions,
    file::TaggedFileExt,
    probe::Probe,
    tag::{Accessor, TagExt},
};

// use lofty::{Accessor, Probe, Tag, TagExt, TagType, TaggedFileExt};

pub fn write_tag(dest: &Path) {
    let mut dest_file = Probe::open(&dest)
        .expect("can't open")
        .read()
        .expect("can't read");
    let tag = dest_file.primary_tag_mut().expect("no tag");
    tag.set_genre("test artist".to_string());
    tag.save_to_path(dest, WriteOptions::new())
        .expect("can't write");
}

fn main() {
    let _ = remove_file("bad.flac");
    copy("sample.flac", "bad.flac").expect("copy failed");
    println!("copied to bad.flac");
    write_tag(Path::new("bad.flac"));
    println!("wrote tag to bad.flac");
    Command::new("flac")
        .args(["-t", "bad.flac"])
        .status()
        .expect("file is corrupted");
}
