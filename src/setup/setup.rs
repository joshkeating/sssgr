use std::fs;
use std::path::Path;

pub fn pre_process() {
    println!("==> Creating output directories...");
    let output_dirs = build_output_structure();
    output_dirs.expect("failed create output directories!");

    println!("==> Validating structure of input files...");
    let input_files = validate_input_files();
    input_files.expect("input file structure incorrect!");

    println!("==> Copying static input files into place...");
    let copied_files = copy_static_input_files();
    copied_files.expect("failed to copy static input files!");
}

fn build_output_structure() -> std::io::Result<()> {

    // remove old output dir and everything contained within
    if Path::new("./out").exists() {
        fs::remove_dir_all("./out")?;
    }

    // create output dir and required sub-directories
    fs::create_dir_all("./out/res")?;
    fs::create_dir("./out/res/images")?;
    fs::create_dir("./out/res/pdfs")?;

    Ok(())
}

fn validate_input_files() -> std::io::Result<()> {

    // make sure that all expected directories and files are present
    fs::metadata("./in/posts")?;
    fs::metadata("./in/style.css")?;
    fs::metadata("./in/res/images")?;
    fs::metadata("./in/res/pdfs")?;
    fs::metadata("./in/res/favicon.ico")?;

    Ok(())
}

fn copy_static_input_files() -> std::io::Result<()> {

    // copy all constant files
    fs::copy("./in/style.css", "./out/style.css")?;
    fs::copy("./in/res/favicon.ico", "./out/res/favicon.ico")?;

    // copy all static assets from images
    let copy_from_images = copy_from_dir("images");
    copy_from_images.expect("failed to copy over input images!");
    // copy all static assets from pdfs
    let copy_from_pdfs = copy_from_dir("pdfs");
    copy_from_pdfs.expect("failed to copy over input images!");

    Ok(())
}

fn copy_from_dir(dir_name: &str) -> std::io::Result<()> {

    // build input and output paths
    let input_dir = format!("{}{}", "./in/res/", dir_name);
    let output_dir = format!("{}{}", "./out/res/", dir_name);

    // copy all files from input $dir_name over to output $dir_name
    for entry in fs::read_dir(Path::new(&input_dir))? {
        let entry = entry?;
        let input_path = entry.path();
        if input_path.is_file() {
            let filename = input_path.file_name().unwrap_or_default();
            let output_path = Path::new(&output_dir).join(filename);
            fs::copy(input_path, output_path)?;
        }
    }

    Ok(())
}


