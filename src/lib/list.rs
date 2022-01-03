use super::Student as Student;
use super::CSV_EXTENSION as CSV_EXTENSION;
use std::path::PathBuf;
use std::fs::File;
use std::error::Error;
use std::env;
use std::ffi::OsString;
use csv::Reader;
use csv::Writer;
use std::io::Read;
use std::io::Write;

type LabCsvRecord = (String, String, String,);

pub fn list(lab_filename: PathBuf) -> Result<(), Box<dyn Error>> {
    let lab_csv_file = File::open(lab_filename)?;
    let mut lab_data = csv::Reader::from_reader(lab_csv_file);

    let mut students = parse_students_from_lab_file(&mut lab_data)?;

    let current_dirpath = env::current_dir()?;
    let mut grading_filename = match current_dirpath.file_name() {
        Some(d) => d.to_os_string(),
        None    => panic!("Cannot get current directory name!")
    };
    grading_filename.push(CSV_EXTENSION);

    let mut grading_csv_file = File::create(grading_filename)?;
    let mut grading_students_writer = csv::Writer::from_writer(grading_csv_file);

    fill_grading_csv_file(&mut grading_students_writer, &mut students)?;

    Ok(())
}

fn parse_students_from_lab_file<R: Read>(lab_data: &mut Reader<R>) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut lab_csv_data: Vec<LabCsvRecord> = Vec::new();

    for lab_record in lab_data.deserialize() {
        let student_in_lab: LabCsvRecord = lab_record?;
        lab_csv_data.push(student_in_lab);
    }

    let students: Vec<Student> = lab_csv_data.into_iter()
        .filter(|student_in_lab| !student_in_lab.1.trim().is_empty() && !student_in_lab.2.trim().is_empty())
        .map(|student_in_lab| Student { 
            index: student_in_lab.1.trim().to_string(),
            name: student_in_lab.2.trim().to_string(),
            points: None,
            comment: None, 
        })
        .collect::<Vec<Student>>();
    
    Ok(students)
}

fn fill_grading_csv_file<W: Write>(grading_students_writer: &mut Writer<W>, students: &mut Vec<Student>) -> Result<(), Box<dyn Error>> {
    students.sort_by(|s1, s2| s1.index.cmp(&s2.index));
    for student in students {
        grading_students_writer.serialize(student)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use csv::ReaderBuilder;
    use csv::WriterBuilder;

    #[test]
    fn parse_students_from_lab_file_basic() -> Result<(), Box<dyn Error>> {
        let data = "\
s100, ra176-2021, Lazar Lazarevic
s101, ra169-2021, Petar Petrovic
s102, ra240-2021, Ana Aleksic
s103, ra211-2021, Milana Milovanovic
";

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .from_reader(data.as_bytes());

        let students = parse_students_from_lab_file(&mut rdr)?;

        assert_eq!(students.len(), 4);
        assert_eq!(students[0].index, "ra176-2021"         );
        assert_eq!(students[0].name,  "Lazar Lazarevic"    );
        assert_eq!(students[1].index, "ra169-2021"         );
        assert_eq!(students[1].name,  "Petar Petrovic"     );
        assert_eq!(students[2].index, "ra240-2021"         );
        assert_eq!(students[2].name,  "Ana Aleksic"        );
        assert_eq!(students[3].index, "ra211-2021"         );
        assert_eq!(students[3].name,  "Milana Milovanovic" );

        Ok(())
    }
}

#[test]
fn fill_grading_csv_file_basic() -> Result<(), Box<dyn Error>> {
    let mut students = vec![
        Student { index: "ra176-2021".to_string(), name: "Lazar Lazarevic".to_string()   , points: None, comment: None, },
        Student { index: "ra169-2021".to_string(), name: "Petar Petrovic".to_string()    , points: None, comment: None, },
        Student { index: "ra240-2021".to_string(), name: "Ana Aleksic".to_string()       , points: None, comment: None, },
        Student { index: "ra211-2021".to_string(), name: "Milana Milovanovic".to_string(), points: None, comment: None, },
    ];

    let expected_data = "\
Broj indeksa,Ime i prezime,Broj poena,Komentar
ra169-2021,Petar Petrovic,,
ra176-2021,Lazar Lazarevic,,
ra211-2021,Milana Milovanovic,,
ra240-2021,Ana Aleksic,,
";

    let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);

    fill_grading_csv_file(&mut wrt, &mut students)?;

    let data = String::from_utf8(wrt.into_inner()?)?;

    assert_eq!(data, expected_data);

    Ok(())
}
