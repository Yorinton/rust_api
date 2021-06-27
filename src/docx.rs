pub use docx_rs::*;
use serde::{Deserialize, Serialize};

pub fn create_docx(doc: Doc) -> Result<(), DocxError> {
    println!("{:?}", doc);
    // lambdaで一時ファイルをファイルシステムに保存する際は、
    // /tmp/配下にしないとRead-Onlyと怒られる
    let path = std::path::Path::new("/tmp/hello.docx");
    let file = std::fs::File::create(&path).unwrap();
    let mut docx = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(doc.header)).align(AlignmentType::Center))
        .add_paragraph(Paragraph::new())
        .add_numbering(Numbering::new(2, 2));
    
    // @TODO ループ内で所有権を奪う処理が必要な場合のもっと良い対応ないか調査
    for section in doc.sections.iter() {
        docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(&section.title)));
        for paragraph in section.paragraphs.iter() {
            docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(&paragraph.body)));
            for sentence in paragraph.sentences.iter() {
                docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sentence.body)));
            }
        }
    }
    docx.build().pack(file)?;
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
                        body: "テスト段落1".to_string(),
                        sentences: vec![
                            Sentence {
                                body: "テスト文章1-1".to_string()
                            },
                            Sentence {
                                body: "テスト文章1-2".to_string()
                            }
                        ]
                    },
                    Para {
                        body: "テスト段落2".to_string(),
                        sentences: vec![
                            Sentence {
                                body: "テスト文章2-1".to_string()
                            },
                            Sentence {
                                body: "テスト文章2-2".to_string()
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