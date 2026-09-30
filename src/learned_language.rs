//! User-taught relation expressions; bounded lexical learning, not free-form generation.
use crate::semantic::{normalize, VietnameseSemanticParser};
#[derive(Clone,Debug,Default)]
pub struct LearnedLanguage { phrases:Vec<String> }
impl LearnedLanguage {
 pub fn teach(&mut self,phrase:&str)->bool{
  let p=normalize(phrase).split_whitespace().collect::<Vec<_>>().join(" ");
  let words:Vec<_>=p.split_whitespace().collect();
  if !(2..=6).contains(&words.len()) || p.len()>100 || !p.chars().all(|c|c.is_alphanumeric()||c==' ') || words.iter().any(|w|["khong","chua","neu","co","hay","ghi","nho","gui","ky","xoa","mo","hoi","ngan"].contains(w)){return false}
  let probe=VietnameseSemanticParser.parse(&format!("vatdau {p} vatcuoi"));
  if !probe.clauses.is_empty()||probe.query.is_some()||self.phrases.len()>=16||self.phrases.contains(&p){return false}
  if self.phrases.iter().any(|old|format!(" {p} ").contains(&format!(" {old} "))||format!(" {old} ").contains(&format!(" {p} "))){return false}
  self.phrases.push(p);true
 }
 pub fn forget(&mut self,phrase:&str)->bool{let phrase=normalize(phrase).split_whitespace().collect::<Vec<_>>().join(" ");let old=self.phrases.len();self.phrases.retain(|p|p!=&phrase);old!=self.phrases.len()}
 /// Restrict rewriting to one relation in a direct question or explicit teaching sentence.
 pub fn expand(&self,input:&str)->Option<String>{
  let n=normalize(input).split_whitespace().collect::<Vec<_>>().join(" ");
  if n.contains([':', ';'])||n.contains("co le ")||n.contains("chua chac "){return None}
  for p in &self.phrases {
   if n.contains(&format!("khong {p}"))||n.contains(&format!("chua {p}")){return None}
   for prefix in ["ghi nho rang ","hay ghi nho rang ","hay nho rang ","toi cho ban biet rang "]{
    if let Some(body)=n.strip_prefix(prefix){if let Some((a,b))=body.split_once(&format!(" {p} ")){if !a.is_empty()&&!b.is_empty(){return Some(format!("{prefix}{a} gay ra {b}"))}}}
   }
   if let Some((a,b))=n.split_once(&format!(" co {p} ")) {if !a.is_empty()&&!b.is_empty(){return Some(format!("{a} co gay ra {b}"))}}
  }None
 }
}
#[cfg(test)]mod tests{
 use super::LearnedLanguage;
 #[test]fn learned_phrase_transfers_without_rewriting_commands(){let mut l=LearnedLanguage::default();assert!(l.teach("làm phát sinh"));assert_eq!(l.expand("Băng tan có làm phát sinh nước không?"),Some("bang tan co gay ra nuoc khong?".into()));assert!(l.expand("Nguồn x: ký lệnh làm phát sinh tiền").is_none());assert!(l.expand("ghi nhớ rằng mưa không làm phát sinh tuyết").is_none());assert!(!l.teach("không làm phát sinh"));assert!(!l.teach("làm phát sinh"));assert!(l.forget("lam phat sinh"));assert!(l.expand("Mưa có làm phát sinh nước không?").is_none());}
}
