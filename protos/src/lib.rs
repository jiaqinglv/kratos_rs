#[allow(non_snake_case)]
pub mod hello_world_v1 {
    include!("helloworld.v1.rs");
}


#[allow(non_snake_case)]
pub mod public {
    include!("public.rs");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("proto.pb");
}

