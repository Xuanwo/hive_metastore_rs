fn main() {
    volo_build::Builder::thrift()
        .add_service("../thrift_idl/hms.thrift")
        .out_dir("../src")
        .split_generated_files(true)
        .write()
        .unwrap();
}
