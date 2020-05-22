use crate::{
    args::{Cp, Opts},
    config::Config,
};
use anyhow::{anyhow, Result};
use std::{fs::File, path::Path};

pub fn cmd(conf: &Config, args: &Cp, Opts { verbose, .. }: &Opts) -> Result<()> {
    let hostpath: Vec<&str> = args.dst.split(':').collect();
    if hostpath.len() != 2 {
        return Err(anyhow!("dst should be host:path"));
    }
    let auth = match conf.hosts.get(hostpath[0]) {
        Some(auth) => auth,
        None => return Err(anyhow!("host {} not defined", hostpath[0])),
    };
    let srcpath = Path::new(&args.src);
    let srcname = srcpath.file_name().unwrap().to_str().unwrap();
    let mut dstname = String::from(hostpath[1]);
    if dstname.len() == 0 || dstname.ends_with('/') {
        dstname.push_str(srcname);
    };

    let reader = File::open(&srcpath).unwrap();
    let metadata = reader.metadata().unwrap();
    let url = format!(
        "https://{}/remote.php/dav/files/{}/{}",
        hostpath[0], &auth.user, &dstname
    );
    let r1 = ureq::put(&url)
        .auth(&auth.user, &auth.password)
        .set("Content-Length", &metadata.len().to_string())
        .send(reader);
    match r1.status() {
        200..=226 => {
            if *verbose {
                println!("{} -> {}:{}", srcname, hostpath[0], dstname);
            }
            Ok(())
        }
        _ => return Err(anyhow!("Error during put - {:?}", r1)),
    }
}
