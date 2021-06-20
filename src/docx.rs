pub use docx_rs::*;
use serde::{Deserialize, Serialize};

const DUMMY: &str = "こんちわ";

pub fn create_docx(doc: Doc) -> Result<(), DocxError> {
    println!("{:?}", doc);
    let path = std::path::Path::new("/tmp/hello.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(840),
            None,
            None,
            None,
        ))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(840),
            Some(SpecialIndentType::FirstLine(720)),
            None,
            None,
        ))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(DUMMY)).indent(
            Some(1560),
            Some(SpecialIndentType::Hanging(720)),
            None,
            None,
        ))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(" World"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Hello"))
                .numbering(NumberingId::new(2), IndentLevel::new(0)),
        )
        .add_abstract_numbering(
            AbstractNumbering::new(2).add_level(
                Level::new(
                    0,
                    Start::new(1),
                    NumberFormat::new("decimal"),
                    LevelText::new("Section %1."),
                    LevelJc::new("left"),
                )
                .indent(
                    Some(720),
                    Some(SpecialIndentType::Hanging(320)),
                    None,
                    None,
                ),
            ),
        )
        .add_numbering(Numbering::new(2, 2))
        .build()
        .pack(file)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Doc {
    title: String,
    header: String,
    sections: Vec<Sec>
}

#[derive(Serialize, Deserialize, Debug)]
struct Sec {
    title: String,
    paragraphs: Vec<Para>
}

#[derive(Serialize, Deserialize, Debug)]
struct Para {
    body: String,
    sentences: Vec<Sentence>
}

#[derive(Serialize, Deserialize, Debug)]
struct Sentence {
    body: String
}


#[test]
fn test_create_docx() {
    let doc = Doc {
        title: "テストタイトル".to_string(),
        header: "テストヘッダー".to_string(),
        sections: vec![
            Sec {
                title: "セクションタイトル".to_string(),
                paragraphs: vec![
                    Para {
                        body: "テスト段落".to_string(),
                        sentences: vec![
                            Sentence {
                                body: "テスト文章".to_string()
                            }
                        ]
                    }
                ]
            }
        ]
    };
    match create_docx(doc) {
        Ok(()) => println!("成功！"),
        Err(e) => println!("{}", e)
    }
}