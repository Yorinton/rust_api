use std::io::Read;

pub use docx_rs::*;
use serde::{Deserialize, Serialize};
use base64;

pub fn create_docx(doc: Doc) -> Result<(), DocxError> {
    // lambdaで一時ファイルをファイルシステムに保存する際は、
    // /tmp/配下にしないとRead-Onlyと怒られる
    let path = std::path::Path::new("/tmp/hello.docx");
    let file = std::fs::File::create(&path).unwrap();
    let mut docx = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(doc.header)).align(AlignmentType::Center))
        .add_paragraph(Paragraph::new())
        .add_numbering(Numbering::new(2, 2));
    
    for section in doc.sections.iter() {
        // .add_paragraphでdocxから値の所有権を奪い、新たなDocx型の値を返却している
        // 新しいDocx型の値を変数docxに束縛しているため、use of moved valueにならない
        docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(&section.title)));
        for paragraph in section.paragraphs.iter() {
            docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(&paragraph.body)));
            for sentence in paragraph.sentences.iter() {
                docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sentence.body)));
            }
        }
    }
    docx.build().pack(file)?;
    
    // 生成したファイルをbase64encode
    let mut file = std::fs::File::open(&path).unwrap();
    let mut buffer = Vec::new();
    // EOFまでのすべてのバイトを読み取り、bufferに配置
    file.read_to_end(&mut buffer).unwrap();
    // ファイルをbase64encodeする場合は、encodeメソッドにそのファイルのバイト列を指定する
    let base64 = base64::encode(buffer);
    println!("{:?}", base64);
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