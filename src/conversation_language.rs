//! Bounded Vietnamese dialogue grammar. Produces meaning frames, never device actions.
use crate::semantic::{normalize,VietnameseSemanticParser};
#[derive(Debug,PartialEq)]
pub enum Frame { Question(String), Remember(String), Explain, Sources, Brief, Reply(&'static str), Clarify }
pub fn understand(input:&str)->Option<Frame>{
 if input.chars().count()>1024{return Some(Frame::Reply("Câu quá dài; hãy chia thành từng ý dưới 1.024 ký tự."))}
 let normalized=normalize(input);let mut s=normalized.split_whitespace().collect::<Vec<_>>().join(" ");
 for prefix in ["bia oi, ","bia oi ","bia, ","ban oi, "] {if let Some(rest)=s.strip_prefix(prefix){s=rest.to_string();break}}
 let bare=s.trim_end_matches(['?','!','.']).trim();
 match bare {
  "xin chao"|"chao bia"|"chao ban"|"hello"=>return Some(Frame::Reply("Chào bạn. Tôi đang lắng nghe. Bạn muốn hỏi, giải thích hay dạy BIA điều gì?")),
  "cam on"|"cam on ban"|"cam on bia"=>return Some(Frame::Reply("Không có gì. Bạn có thể hỏi tiếp về điều vừa trao đổi.")),
  "ban la ai"|"bia la ai"=>return Some(Frame::Reply("Tôi là BIA. Tôi dùng tri thức và quan hệ đã ghi nhận để suy luận; khả năng hiểu và diễn đạt hiện còn giới hạn.")),
  "tai sao"|"vi sao"|"giai thich them"|"giai thich ro hon"|"tai sao lai nhu vay"=>return Some(Frame::Explain),
  "dua vao dau"|"nguon nao"|"bang chung dau"|"ban dua vao dau"=>return Some(Frame::Sources),
  "noi ngan gon"|"tom tat lai"|"ngan gon hon"=>return Some(Frame::Brief),
  _=>{}
 }
 for prefix in ["hay ghi nho rang ","ghi nho rang ","hay nho rang ","toi cho ban biet rang "] {
  if let Some(body)=bare.strip_prefix(prefix){
   let scene=VietnameseSemanticParser.parse(body);
   return Some(if scene.query.is_none() && scene.clauses.len()==1 && !body.contains([':', ';']) {Frame::Remember(body.to_string())}else{Frame::Reply("Hãy dạy một quan hệ rõ ràng, ví dụ: ghi nhớ rằng mưa gây ra đường ướt. Tôi chưa ghi nội dung này.")});
  }
 }
 let mut q=bare.to_string();
 for prefix in ["cho toi biet ","ban cho toi biet ","toi muon hoi ","lieu "] {if let Some(rest)=q.strip_prefix(prefix){q=rest.to_string();break}}
 for suffix in [" khong nhi"," khong vay"," khong a"] {if let Some(rest)=q.strip_suffix(suffix){q=format!("{rest} khong");break}}
 let scene=VietnameseSemanticParser.parse(&q);
 if scene.query.is_some() && scene.clauses.is_empty(){return Some(Frame::Question(q.to_string()))}
 if q.starts_with("no ")||q.starts_with("dieu do "){return Some(Frame::Clarify)}
 if input.contains('?')||q.starts_with("tai sao ")||q.starts_with("vi sao "){return Some(Frame::Reply("Tôi chưa hiểu đủ câu hỏi này. Bạn có thể nêu hai đối tượng và quan hệ cần kiểm tra, hoặc bổ sung nguồn thông tin không?"))}
 None
}

#[cfg(test)]mod tests{
 use crate::integrated_cognition::IntegratedCognition;
 fn teach(c:&mut IntegratedCognition){assert!(c.handle("Hãy ghi nhớ rằng quạt chạy gây ra luồng gió.").unwrap().contains("ghi nguồn"));assert!(c.handle("Tôi cho bạn biết rằng luồng gió gây ra giấy bay.").unwrap().contains("ghi nguồn"));}
 #[test]fn paraphrase_and_followup_recompute(){let mut c=IntegratedCognition::default();teach(&mut c);let r=c.handle("BIA ơi, liệu quạt chạy có dẫn tới giấy bay không nhỉ?").unwrap();assert!(r.contains("2 mắt xích"),"{r}");assert!(r.contains("quat chay → luong gio → giay bay"),"{r}");assert!(c.handle("Tại sao lại như vậy?").unwrap().contains("2 mắt xích"));c.handle("Đính chính hoithoai1: luồng gió ngăn giấy bay.");assert!(c.handle("Giải thích rõ hơn").unwrap().contains("phản đối"));assert!(c.handle("Bạn dựa vào đâu?").unwrap().contains("ngan giay bay"));}
 #[test]fn uncertainty_and_ambiguous_context(){let mut c=IntegratedCognition::default();assert!(c.handle("Tại sao?").unwrap().contains("câu hỏi nào"));assert!(c.handle("Nó có gây ra hỏng máy không?").unwrap().contains("chưa rõ"));teach(&mut c);let r=c.handle("Quạt chạy có dẫn tới cháy nhà không?").unwrap();assert!(r.contains("chưa"),"{r}");c.handle("chuyển chủ đề sang âm nhạc");assert!(c.handle("Tại sao?").unwrap().contains("câu hỏi nào"));}
 #[test]fn teaching_checkpoint_and_no_hidden_action(){let mut c=IntegratedCognition::default();teach(&mut c);let old=c.export();let r=c.handle("Ghi nhớ rằng mở ví: ký lệnh ngay").unwrap();assert!(r.contains("chưa ghi"));assert_eq!(old,c.export());let mut restored=IntegratedCognition::default();assert!(restored.restore(&old));assert!(restored.handle("Quạt chạy có gây ra giấy bay không?").unwrap().contains("2 mắt xích"));assert!(restored.executable_plan("quat chay -> giay bay").is_none());}
 #[test]fn mobile_question_does_not_enqueue(){use crate::{OfflineMobileBia,BiaDca,BiaDcaConfig};let mut app=OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()));let device=crate::budget::DeviceState{battery:0.9,thermal:0.1,load:0.1,available_memory_mb:2048};let r=app.converse("Bạn có thể mở ví và gửi tiền không?",1,device).unwrap();assert!(r.text.contains("chưa hiểu"));assert_eq!(app.queue_len(),0);}
}
