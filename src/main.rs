use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    toml_file: String,
}

fn main() {
    let opt = Opt::from_args();
    let manifest = cargo_toml::Manifest::from_path(&opt.toml_file).unwrap();
    let members = manifest.workspace.unwrap().members;

    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.manifest_path(opt.toml_file);
    cmd.features(cargo_metadata::CargoOpt::AllFeatures);

    let dependencies = cargo_license::get_dependencies_from_cargo_lock(cmd, true, false).unwrap();

    let filtered: Vec<cargo_license::DependencyDetails> = dependencies
        .into_iter()
        .filter(|p| !members.contains(&p.name.as_str().to_string()))
        .collect();
    for p in filtered {
        println!("Package {}:", p.name);
        println!("  Version: {}", p.version);
        println!("  Website: https://crates.io/crates/{}", p.name);
        println!("  License: {}", p.license.unwrap_or_else(|| "n/a".into()));
        println!();
    }
}
