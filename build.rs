use walkdir::WalkDir;

fn main() {
    let protos = WalkDir::new("lol-grpc-models/proto")
        .into_iter()
        .filter_map(|it| it.ok())
        .filter(|it| it.file_type().is_file())
        .filter(|it| it.path().extension().and_then(|it| it.to_str()) == Some("proto"))
        .filter_map(|it| it.path().to_str().map(|s| s.to_owned()))
        .collect::<Vec<String>>();

    tonic_build::configure()
        .compile_well_known_types(true)
        .type_attribute(".", "#[derive(Deserialize)]")
        .type_attribute(".", "#[derive(Serialize)]")
        .compile(protos.as_slice(), &["lol-grpc-models/"])
        .unwrap()
}
