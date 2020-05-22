use crate::{
    args::{Cp, Opts},
    config::Config,
};
use anyhow::{anyhow, Result};
use std::{fs::File, path::Path};

pub fn cmd(conf: &Config, args: &Cp, Opts { verbose, .. }: &Opts) -> Result<()> {
    let accesspath: Vec<&str> = args.dst.split(':').collect();
    if accesspath.len() != 2 {
        return Err(anyhow!("dst should be config:path"));
    }
    let access = match conf.access.get(accesspath[0]) {
        Some(access) => access,
        None => return Err(anyhow!("config {} hast not been defined", accesspath[0])),
    };
    let srcpath = Path::new(&args.src);
    let srcname = srcpath.file_name().unwrap().to_str().unwrap();
    let mut dstname = String::from(accesspath[1]);
    if dstname.len() == 0 || dstname.ends_with('/') {
        dstname.push_str(srcname);
    };

    let reader = File::open(&srcpath).unwrap();
    let metadata = reader.metadata().unwrap();
    let url = format!(
        "https://{}/remote.php/dav/files/{}/{}",
        &access.host, &access.user, &dstname
    );
    let r1 = ureq::put(&url)
        .auth(&access.user, &access.password)
        .set("Content-Length", &metadata.len().to_string())
        .send(reader);
    match r1.status() {
        200..=226 => {
            if *verbose {
                println!("{} -> {}:{}", srcname, accesspath[0], dstname);
            }
            Ok(())
        }
        _ => return Err(anyhow!("Error during put - {:?}", r1)),
    }
}
