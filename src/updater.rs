use self_update::cargo_crate_version;

pub fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    if check_for_update()? {
        self_update::backends::github::Update::configure()
            .repo_owner("Huggepugge1")
            .repo_name("achievements-enhanced")
            .bin_name("achievements-enhanced")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;
    }
    Ok(())
}

pub fn check_for_update() -> Result<bool, Box<dyn ::std::error::Error>> {
    let latest = self_update::backends::github::Update::configure()
        .repo_owner("Huggepugge1")
        .repo_name("achievements-enhanced")
        .bin_name("achievements-enhanced")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .get_latest_release();

    match latest {
        Ok(latest) => {
            if *latest.version > *cargo_crate_version!() {
                println!("Update found: {}", latest.version);
                return Ok(true);
            } else {
                println!("No update found");
            }
        }
        Err(err) => {
            println!("Error checking for updates: {}", err);
        }
    }
    Ok(false)
}
