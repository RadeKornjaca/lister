use super::Student as Student;
use std::error::Error;
use std::path::PathBuf;
use std::fs::File;
use std::env::current_dir;
use csv::Reader;
use std::io::Read;
use std::io::Write;
use pdf_writer::types::{ ActionType, AnnotationType, BorderType };
use pdf_writer::{ Content, Finish, Name, PdfWriter, Rect, Ref, Str };
use pdf_writer::types::{ LineJoinStyle::MiterJoin, LineCapStyle::ButtCap };


const PDF_EXTENSION: &str = "pdf";

pub fn table(students_grading_csv_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let students_grading_csv = File::open(students_grading_csv_path.clone())?;
    let mut students_grading_data = csv::Reader::from_reader(students_grading_csv);

    let students = parse_students_from_grading_file(&mut students_grading_data)?;

    let mut table_filename = students_grading_csv_path;
    table_filename.set_extension(PDF_EXTENSION);

    let pdf_writer = PdfWriter::new();
    create_pdf_table_file(pdf_writer, students, table_filename)?;

    Ok(())
}

fn parse_students_from_grading_file<R: Read>(students_grading_data: &mut Reader<R>) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut students_results: Vec<Student> = Vec::new();

    for student_grade_record in students_grading_data.deserialize() {
        let student_result: Student = student_grade_record?;
        students_results.push(student_result);
    }
    
    Ok(students_results)
}

fn create_pdf_table_file(mut pdf_writer: PdfWriter, students: Vec<Student>, pdf_filename_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let font_id = Ref::new(4);
    let content_id = Ref::new(5);
    let font_name = Name(b"F1");

    pdf_writer.catalog(catalog_id).pages(page_tree_id);

    pdf_writer.pages(page_tree_id).kids([page_id]).count(1);

    let mut page = pdf_writer.page(page_id);

    page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
    page.parent(page_tree_id);
    page.contents(content_id);

    page.resources().fonts().pair(font_name, font_id);
    page.finish();

    pdf_writer.type1_font(font_id).base_font(Name(b"Helvetica"));

    let mut content = Content::new();

    content.transform([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    content.save_state();
    content.transform([1.0, 0.0, 0.0, 1.0, 62.69291, 759.0236]);
    content.restore_state();
    content.save_state();

    content.transform([1.0, 0.0, 0.0, 1.0, 62.69291, 201.0236]);
    content.set_fill_rgb(1.0, 1.0, 1.0);
    content.end_path();

    let mut row_position = 558.0;
    let row_step = 36.0;

    while row_position >= 0.0 {
        content.rect(0.0, row_position, 469.8898, -18.0);
        content.fill_even_odd();
        content.set_fill_rgb(0.878431, 0.878431, 0.878431);
        content.end_path();
        row_position -= row_step;
    }

    // header
    content.save_state();
    content.transform([1.0, 0.0, 0.0, 1.0, 6.0, 543.0]);
    content.save_state();
    content.set_fill_rgb(0.960784, 0.960784, 0.862745);
    content.end_path();
    // content.rect(0.0, 0.0, 77.50281, 12.0);      // This is original from PDF file
    content.rect(-6.5, -3.0, 89.50281, 18.0);       // Since the original was inaccurate, this is a correct filling
    content.fill_even_odd();
    content.restore_state();
    content.save_state();
    content.set_fill_rgb(0.0, 0.0, 0.0);
    content.begin_text();
    content.set_text_matrix([1.0, 0.0, 0.0, 1.0, 0.0, 2.0]);
    content.next_line(7.85541, 0.0);
    content.set_font(font_name, 10.0);
    content.set_leading(12.0);
    content.show(Str(b"Broj indeksa"));
    content.next_line_using_leading();
    content.next_line(-7.85541, 0.0);
    content.end_text();

    content.restore_state();
    content.restore_state();
    content.save_state();
    content.transform([1.0, 0.0, 0.0, 1.0, 95.50281, 543.0]);
    content.save_state();
    content.set_fill_rgb(0.960784, 0.960784, 0.862745);
    content.end_path();
    // content.rect(0.0, 0.0, 286.3427, 12.0);      // This is original from PDF file
    content.rect(-6.5, -3.0, 298.3427, 18.0);       // Since the original was inaccurate, this is a correct filling
    content.fill_even_odd();
    content.restore_state();
    content.save_state();
    content.set_fill_rgb(0.0, 0.0, 0.0);
    content.begin_text();
    content.set_text_matrix([1.0, 0.0, 0.0, 1.0, 0.0, 2.0]);
    content.next_line(108.916, 0.0);
    content.set_font(font_name, 10.0);
    content.set_leading(12.0);
    content.show(Str(b"Ime i prezime"));
    content.next_line_using_leading();
    content.next_line(-108.916, 0.0);
    content.end_text();

    content.restore_state();
    content.restore_state();
    content.save_state();
    content.transform([1.0, 0.0, 0.0, 1.0, 393.8455, 543.0]);
    content.save_state();
    content.set_fill_rgb(0.960784, 0.960784, 0.862745);
    content.end_path();
    // content.rect(0.0, 0.0, 70.04424, 12.0);      // This is original from PDF file
    content.rect(-6.5, -3.0, 82.0443, 18.0);        // Since the original was inaccurate, this is a correct filling
    content.fill_even_odd();
    content.restore_state();
    content.save_state();
    content.set_fill_rgb(0.0, 0.0, 0.0);
    content.begin_text();
    content.set_text_matrix([1.0, 0.0, 0.0, 1.0, 0.0, 2.0]);
    content.set_font(font_name, 10.0);
    content.set_leading(12.0);
    content.show(Str(b"Broj poena"));
    content.next_line_using_leading();
    content.next_line(-7.956693, 0.0);
    content.end_text();
    content.restore_state();
    content.restore_state();

    let mut text_position = 525.0;
    let text_step = row_step / 2.0;

    for student in students {
        // Index number
        content.set_fill_rgb(0.0, 0.0, 0.0);
        content.save_state();
        content.transform([1.0, 0.0, 0.0, 1.0, 6.0, text_position]);
        content.save_state();
        content.begin_text();
        content.set_text_matrix([1.0, 0.0, 0.0, 1.0, 0.0, 2.0]);
        content.set_font(font_name, 10.0);
        content.set_leading(12.0);
        content.show(Str(student.index.as_bytes()));
        content.next_line_using_leading();
        content.end_text();
        content.restore_state();
        content.restore_state();
        content.save_state();

        // First and last name of the student
        content.transform([1.0, 0.0, 0.0, 1.0, 95.50281, text_position]);
        content.save_state();
        content.set_fill_rgb(0.0, 0.0, 0.0);
        content.begin_text();
        content.set_text_matrix([1.0, 0.0, 0.0, 1.0, 0.0, 2.0]);
        content.set_font(font_name, 10.0);
        content.set_leading(12.0);
        content.show(Str(student.name.as_bytes()));
        content.next_line_using_leading();
        content.end_text();
        content.restore_state();
        content.restore_state();
        content.save_state();

        // Points
        content.transform([1.0, 0.0, 0.0, 1.0, 393.8455, text_position]);
        content.save_state();
        content.set_fill_rgb(0.0, 0.0, 0.0);
        content.begin_text();
        content.set_text_matrix([1.0, 0.0, 0.0, 1.0, 0.0, 2.0]);
        content.set_font(font_name, 10.0);
        content.set_leading(12.0);
        content.show(Str(student.points.unwrap().to_string().as_bytes()));
        content.next_line_using_leading();
        content.end_text();
        content.restore_state();
        content.restore_state();
        content.save_state();

        text_position -= text_step;
    }

    // table border lines

    // table row lines
    let mut stroke_position = 540.0;
    let stroke_step = 18.0;

    content.set_line_cap(ButtCap);
    content.set_line_join(MiterJoin);
    content.set_stroke_rgb(0.0, 0.0, 0.0);
    content.set_line_width(0.25);

    while stroke_position >= stroke_step {
        content.end_path();
        content.move_to(0.0, stroke_position);
        content.line_to(469.8898, stroke_position);
        content.stroke();
        stroke_position -= stroke_step;
    }

    // column borders
    content.end_path();
    content.move_to(89.50281, 0.0);
    content.line_to(89.50281, 558.0);
    content.stroke();

    content.end_path();
    content.move_to(387.8455, 0.0);
    content.line_to(387.8455, 558.0);
    content.stroke();

    // table borders
    content.end_path();
    content.move_to(0.0, 558.0);
    content.line_to(469.8898, 558.0);
    content.stroke();

    content.end_path();
    content.move_to(0.0, 0.0);
    content.line_to(469.8898, 0.0);
    content.stroke();

    content.end_path();
    content.move_to(0.0, 0.0);
    content.line_to(0.0, 558.0);
    content.stroke();

    content.end_path();
    content.move_to(469.8898, 0.0);
    content.line_to(469.8898, 558.0);
    content.stroke();

    content.restore_state();
    content.restore_state();
    content.save_state();
    content.transform([1.0, 0.0, 0.0, 1.0, 62.69291, 201.0236]);
    content.restore_state();

    pdf_writer.stream(content_id, &content.finish());
    let buffer: Vec<u8> = pdf_writer.finish();

    std::fs::write(pdf_filename_path.into_os_string(), buffer)?;

    Ok(())
}