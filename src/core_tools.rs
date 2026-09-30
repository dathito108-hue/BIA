//! Core-owned admission and bounded receipts for native tool executors.
//! This is not wallet authority and does not claim autonomous planning.
#[derive(Clone, Debug, PartialEq)]
pub struct Receipt { pub id:u64, pub skill:String, pub outcome:u8 }
#[derive(Default)]
pub struct CoreTools { next:u64, active:Vec<Receipt>, receipts:Vec<Receipt> }
pub const SKILLS:&[&str]=&["product.compile","image.render","scene.render","sculpt.apply","motion.export","image.train","image.generate","market.analyze","market.quality","dex.read","dex.prepare","solana.quote"];
impl CoreTools {
    pub fn begin(&mut self, skill:&str)->Option<u64>{
        if !SKILLS.contains(&skill) || self.active.len()>=32 {return None}
        self.next=self.next.checked_add(1)?;
        if self.next>i64::MAX as u64 {return None}
        self.active.push(Receipt{id:self.next,skill:skill.into(),outcome:0});Some(self.next)
    }
    pub fn finish(&mut self,id:u64,success:bool)->bool{
        let Some(i)=self.active.iter().position(|r|r.id==id) else{return false};
        let mut r=self.active.remove(i);r.outcome=if success{1}else{2};self.record(r);true
    }
    fn record(&mut self,r:Receipt){self.receipts.push(r);if self.receipts.len()>32{self.receipts.remove(0);}}
    pub fn busy(&self)->bool{!self.active.is_empty()}
    pub fn report(&self)->String{
        let mut s=String::from("BIA: một runtime điều phối. Công cụ đăng ký (không phải thước đo trí tuệ):\n");
        s.push_str(&SKILLS.join(", "));s.push_str("\nGame: trạng thái trong lõi; thao tác máy: hàng đợi và quyền riêng. Ví: vẫn cần quy trình phê duyệt riêng.\n1=hoàn tất hàm, 2=lỗi, 3=bị gián đoạn; không chứng minh chất lượng hoặc lợi nhuận.\n");
        for r in &self.receipts {s.push_str(&format!("{} {} {}\n",r.id,r.skill,r.outcome));}
        s.push_str(&format!("Đang thực thi: {}",self.active.len()));s
    }
    pub fn export(&self)->String{
        let mut s=format!("T143\t{}\n",self.next);
        for r in self.receipts.iter().chain(self.active.iter()) {s.push_str(&format!("U143\t{}\t{}\t{}\n",r.id,r.skill,r.outcome));}s
    }
    pub fn restore(text:&str)->Option<Self>{
        let mut state=Self::default();let mut header=false;let mut ids=std::collections::HashSet::new();
        for line in text.lines(){let p:Vec<_>=line.split('\t').collect();match p[0]{
            "T143"=>{if header||p.len()!=2{return None}header=true;state.next=p[1].parse().ok()?;if state.next>i64::MAX as u64{return None}},
            "U143"=>{if !header||p.len()!=4||!SKILLS.contains(&p[2])||ids.len()>=64{return None}let id=p[1].parse().ok()?;let outcome:u8=p[3].parse().ok()?;if id==0||id>state.next||!ids.insert(id)||outcome>3{return None}state.record(Receipt{id,skill:p[2].into(),outcome:if outcome==0{3}else{outcome}});},_=>{}
        }}Some(state)
    }
    // Imported old snapshots must not reuse live-process tickets.
    pub fn retain_counter(&mut self,previous:&Self){self.next=self.next.max(previous.next);}
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn authority_and_receipts(){let mut s=CoreTools::default();assert!(s.begin("solana.sign").is_none());let id=s.begin("image.render").unwrap();assert!(!s.finish(id+1,true));assert!(s.finish(id,false));assert!(!s.finish(id,true));assert_eq!(s.receipts[0].outcome,2);}
 #[test]fn interrupted_restore_and_bounds(){let mut s=CoreTools::default();for _ in 0..32{s.begin("scene.render").unwrap();}assert!(s.begin("scene.render").is_none());let mut r=CoreTools::restore(&s.export()).unwrap();assert!(!r.busy());assert!(r.receipts.iter().all(|r|r.outcome==3));assert_eq!(r.begin("image.train"),Some(33));assert!(CoreTools::restore("T143\t1\nU143\t1\tsolana.sign\t1").is_none());}
}
