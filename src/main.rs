use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    toml_file: String,
    lock_file: String,
}

fn main() {
    let opt = Opt::from_args();
    let manifest = cargo_toml::Manifest::from_path(opt.toml_file).unwrap();
    let members = manifest.workspace.unwrap().members;
    let lockfile = cargo_lock::Lockfile::load(opt.lock_file).unwrap();
    let packages = lockfile.packages;
    let filtered: Vec<cargo_lock::package::Package> = packages.into_iter().filter(|p| !members.contains(&p.name.as_str().to_string())).collect();
    for p in filtered {
        println!("Package {}:", p.name);
        println!("  Version: {}", p.version);
        println!("  Website: https://crates.io/crates/{}", p.name);
    }
}
