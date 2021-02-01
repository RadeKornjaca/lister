use std::path::PathBuf;
use structopt::StructOpt;
use lister::lib::*;
use std::error::Error;

#[derive(Debug, StructOpt)]
#[structopt(about = "A tool for extraction and grading document generation for student assignments")]
enum Subcommands {
    #[structopt(about = "Extracts tarball given from lab to a specified directory")]
    Extract {
        #[structopt(parse(from_os_str))]
        tar_file: PathBuf,
        #[structopt(parse(from_os_str))]
        dest_dirname: PathBuf
    },
    #[structopt(about = "Create CSV grading file for the extracted group of students")]
    List {
        #[structopt(parse(from_os_str))]
        lab_filename: PathBuf
    },
    #[structopt(about = "Generate PDF file that contains the grades for the extracted group of students")]
    Table {
        #[structopt(parse(from_os_str))]
        students_grading_csv: PathBuf
    }
}

impl Subcommands {
    fn execute(self) -> Result<(), Box<dyn Error>> {
        match self {
            Subcommands::Extract { tar_file, dest_dirname } => extract(tar_file, dest_dirname),
            Subcommands::List { lab_filename }              => list(lab_filename),
            Subcommands::Table { students_grading_csv }     => table(students_grading_csv),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Subcommands::from_args();
    println!("{:?}", opt);

    opt.execute()
}
