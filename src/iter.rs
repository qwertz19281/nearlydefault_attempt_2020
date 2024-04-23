use std::{sync::Arc, path::{PathBuf, Path}};
use walkdir::*;
use super::*;
use rayon::*;
use image::*;
use image::imageops::*;
use std::fs::*;

pub fn transfer_dir(src: &Path, dest: &Path, p: &Scope, o: &Arc<Opt>) -> anyhow::Result<()> {
    for e in WalkDir::new(src) {
        if let Some(e) = soft_error!(e,"Error {}") {
            if let Some(meta) = soft_error!(e.metadata(),"Error {}") {
                if meta.is_file() {
                    let fsrc = e.path();
                    let rel = fsrc.strip_prefix(src).unwrap();
                    let fdest = dest.join(&rel);
                    enqueue_file(fsrc,fdest,rel,p,o);
                }
            }
        }
    }
    Ok(())
}

pub fn enqueue_file(src: &Path, dest: PathBuf, rel: &Path, p: &Scope, o: &Arc<Opt>) {
    //eprint!("Transfer: {}: ",rel.to_string_lossy());

    let rel_s = rel.to_string_lossy();

    if let Some(x) = &o.include_regex {
        if !x.is_match(rel_s.as_ref()) {
            return;
        }
    }
    if let Some(x) = &o.exclude_regex {
        if x.is_match(rel_s.as_ref()) {
            return;
        }
    }

    let src = src.to_owned();
    let rel = rel.to_owned();
    let o = Arc::clone(o);

    p.spawn(move |_| {
        eprintln!("Process: {}",rel.to_string_lossy());

        if let Err(e) = process_file(&src, &dest, &o) {
            eprintln!("\tError: {}",e);
        }
    });
}

pub fn process_file(src: &Path, dest: &Path, o: &Arc<Opt>) -> anyhow::Result<()> {
    if let Some(p) = dest.parent() {
        std::fs::create_dir_all(p)?;
    }

    let image = {
        let file = read(src)?;
        load_from_memory(&file)?
    };

    let image = image.resize_exact(image.width()*o.scale, image.height()*o.scale, FilterType::Nearest);

    let image = image.to_rgba();

    let image = 
        if o.alternative {
            match alg_alt::process_alt(&image,o) {
                Err(e) => {
                    eprintln!("\tError: {}. Fallback to standard alg",e);
                    alg::process(&image,o)
                },
                Ok(o) => Ok(o),
            }
        } else {
            alg::process(&image,o)
        }?;

    let encoded = {
        let mut dest = Vec::with_capacity(image.len()*2);

        {
            let mut encoder = ::png::Encoder::new(&mut dest, image.width(), image.height());
            encoder.set_color(::png::ColorType::RGBA);
            encoder.set_depth(::png::BitDepth::Eight);
            encoder.set_compression(::png::Compression::Best);
            encoder.set_filter(::png::FilterType::Paeth);
            let mut writer = encoder.write_header()?;
            writer.write_image_data(&image)?;
        }

        dest
    };

    std::fs::write(dest, &encoded)?;

    Ok(())
}