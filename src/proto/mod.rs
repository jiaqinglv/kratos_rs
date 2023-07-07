pub mod hello_world_v1 {
    include!("helloworld.v1.rs");
}


pub mod public {
    include!("public.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("proto.pb");
}

