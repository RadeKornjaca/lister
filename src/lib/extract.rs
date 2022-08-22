use tar::{ Archive, Entry };
use flate2::read::GzDecoder;
use std::fs::{ File, create_dir_all };
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;

pub fn extract(tar_filepath: PathBuf, dest_dirname: PathBuf) -> Result<(), Box<dyn Error>> {
    let tar_file = File::open(tar_filepath)?;
    let mut tar_archive = Archive::new(tar_file);

    create_dir_all(dest_dirname.as_path())?;

    for file in tar_archive.entries().unwrap() {
        let file = file.unwrap();

        let mut filepath = match file.path() {
            Ok(path) => path,
            Err(_) => continue,
        };

        let file_extension = match get_extension_from_path(filepath.to_mut()) {
            Some(file_extension) => file_extension,
            None => continue,
        };

        match file_extension {
            "tgz" => extract_from_student_computer(file, &dest_dirname),
            "txt" => extract_working_students_list(file, &dest_dirname),
            _ => (),
        };

    }

    Ok(())
}

fn extract_from_student_computer(file: Entry<File>, dest_dirname: &PathBuf) {
    let student_unzipped_archive = GzDecoder::new(file);
    let mut student_archive = Archive::new(student_unzipped_archive);

    for f in student_archive.entries().unwrap() {
        let mut f = f.unwrap();

        let filepath = f.header().path().unwrap();

        let file_extension = match get_extension_from_path(&filepath) {
            Some(file_extension) => file_extension,
            None => continue,
        };

        if file_extension == "c" {
            let mut dest_filepath = dest_dirname.clone();
            dest_filepath.push(filepath.parent().unwrap().file_name().unwrap());
            dest_filepath.set_extension(file_extension);
            f.unpack(dest_filepath.as_path()).unwrap();
        }
    }
}

fn extract_working_students_list(mut file: Entry<File>, dest_dirname: &PathBuf) {
    let mut dest_filename = dest_dirname.clone();
    dest_filename.push(file.header().path().unwrap().file_name().unwrap());

    file.unpack(dest_filename).unwrap();
}

fn get_extension_from_path(filepath: &Path) -> Option<&str> {    
    filepath.extension().and_then(OsStr::to_str)
}
