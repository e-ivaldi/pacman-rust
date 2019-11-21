extern crate fs_extra;

use std::env;

use fs_extra::dir::{copy, CopyOptions};

fn main() {
    let profile = env::var("PROFILE").unwrap();

    let options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: false,
        depth: 0,
    };

    let resources_dir = "resources";

    let source = format!(
        "{}/{}",
        std::env::current_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap(),
        resources_dir
    );

    let target = format!("target/{}", profile);

    copy(&source, &target, &options)
        .expect(format!("Unable to copy {} to {}", source, target).as_str());
}
