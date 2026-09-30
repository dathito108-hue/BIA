use bia_core::integrated_cognition::IntegratedCognition;
#[test]
fn learns_expression_and_transfers_after_restore(){
 let mut c=IntegratedCognition::default();
 let before=c.handle("Mưa có làm phát sinh đường ướt không?").unwrap();assert!(before.contains("chưa hiểu"),"{before}");
 assert!(c.handle("Cách nói làm phát sinh: gây ra").unwrap().contains("Đã học"));
 c.handle("Ghi nhớ rằng mưa làm phát sinh đường ướt.");
 c.handle("Ghi nhớ rằng băng tan gây ra nước.");
 let answer=c.handle("Băng tan có làm phát sinh nước không?").unwrap();assert!(answer.contains("ủng hộ"),"{answer}");
 let mut restored=IntegratedCognition::default();assert!(restored.restore(&c.export()));
 assert!(restored.handle("Băng tan có làm phát sinh nước không?").unwrap().contains("ủng hộ"));
 restored.handle("Rút cách nói làm phát sinh");assert!(restored.handle("Băng tan có làm phát sinh nước không?").unwrap().contains("chưa hiểu"));
 assert!(restored.handle("Băng tan có gây ra nước không?").unwrap().contains("ủng hộ"));
}
#[test]
fn role_followups_and_selected_provenance(){
 let mut c=IntegratedCognition::default();
 c.handle("Nguồn mua: mưa gây ra đường ướt.");c.handle("Nguồn uot: đường ướt gây ra trơn trượt.");c.handle("Nguồn khac: nhiệt gây ra giãn nở.");
 assert!(c.handle("Mưa có gây ra đường ướt không?").unwrap().contains("ủng hộ"));
 assert!(c.handle("Còn kết quả trơn trượt thì sao?").unwrap().contains("2 mắt xích"));
 let sources=c.handle("Dựa vào đâu?").unwrap();assert!(sources.contains("mua:")&&sources.contains("uot:"),"{sources}");assert!(!sources.contains("khac:"));
 assert!(c.handle("Còn nhiệt thì sao?").unwrap().contains("nguyên nhân hay kết quả"));
 let changed=c.handle("Còn nguyên nhân nhiệt thì sao?").unwrap();assert!(changed.contains("chưa"),"{changed}");
 assert!(c.handle("Dựa vào đâu?").unwrap().contains("chưa xác định"));
}
#[test]
fn expression_authority_and_duplicate_learning(){
 let mut c=IntegratedCognition::default();c.handle("Cách nói làm phát sinh: gây ra");let snapshot=c.export();
 c.handle("Cách nói làm phát sinh: gây ra");assert_eq!(snapshot,c.export());
 c.handle("Cách nói không làm phát sinh: gây ra");assert_eq!(snapshot,c.export());
 c.handle("Cách nói tự chuyển tiền: mở ví");assert_eq!(snapshot,c.export());
 c.handle("Ghi nhớ rằng mưa không làm phát sinh tuyết.");assert_eq!(snapshot,c.export());
 assert!(c.executable_plan("mua -> tuyet").is_none());
}
