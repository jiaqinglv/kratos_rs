use std::{env, path::PathBuf};

use std::fs;


fn load_proto(input_path:& str,out_path:&mut Vec<PathBuf>) {
    for entry in fs::read_dir(input_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // 目录
            load_proto(path.clone().to_str().unwrap(), out_path);
        } else {
            out_path.push(path.clone());
        }
    }
}

fn main() {
    let _out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut out_path = vec![];
    load_proto("proto/api",&mut out_path);

    tonic_build::configure()
        .out_dir("protos/src")
        .file_descriptor_set_path("protos/src/proto.pb")
        .compile(&out_path, &["proto"])
        .unwrap();
}
