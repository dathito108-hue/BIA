use crate::types::Phenomenon;

#[derive(Clone, Debug)]
pub struct Seed {
    pub signature:Vec<f32>, pub meaning:u32, pub strength:f32, pub utility:f32,
    pub repetitions:u32, pub last_seen:u64,
}
#[derive(Clone, Debug)]
pub struct SeedMemory { capacity:usize, seeds:Vec<Seed> }
impl SeedMemory {
    pub fn new(capacity:usize)->Self{assert!(capacity>0);Self{capacity,seeds:Vec::new()}}
    pub fn imprint(&mut self,p:&Phenomenon,meaning:u32,utility:f32){
        if let Some(s)=self.seeds.iter_mut().find(|s|s.meaning==meaning && similarity(&s.signature,&p.features)>0.92){
            s.repetitions=s.repetitions.saturating_add(1);
            s.strength=(s.strength*0.90+0.10*p.confidence).clamp(0.0,1.0);
            s.utility=(s.utility*0.85+0.15*utility).clamp(-1.0,1.0);
            s.last_seen=p.timestamp; return
        }
        if self.seeds.len()>=self.capacity{
            let i=self.seeds.iter().enumerate().min_by(|(_,a),(_,b)| retention(a).partial_cmp(&retention(b)).unwrap_or(std::cmp::Ordering::Equal)).map(|(i,_)|i).unwrap_or(0);
            self.seeds.remove(i);
        }
        self.seeds.push(Seed{signature:p.features.clone(),meaning,strength:p.confidence,utility:utility.clamp(-1.0,1.0),repetitions:1,last_seen:p.timestamp});
    }
    pub fn recall(&self,p:&Phenomenon,limit:usize)->Vec<Seed>{
        let mut xs:Vec<(f32,&Seed)>=self.seeds.iter().map(|s|(similarity(&s.signature,&p.features)*(0.5+0.5*s.strength)*(0.75+0.25*s.utility.max(0.0)),s)).collect();
        xs.sort_by(|a,b|b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        xs.into_iter().take(limit).filter(|(x,_)|*x>0.05).map(|(_,s)|s.clone()).collect()
    }
    pub fn len(&self)->usize{self.seeds.len()}
    pub fn is_empty(&self)->bool{self.seeds.is_empty()}
}
fn retention(s:&Seed)->f32{s.strength*0.55+s.utility.max(0.0)*0.25+(s.repetitions as f32).ln_1p()*0.20}
fn similarity(a:&[f32],b:&[f32])->f32{
    if a.is_empty()||b.is_empty(){return 0.0}
    let n=a.len().min(b.len()); let(mut dot,mut aa,mut bb)=(0.0,0.0,0.0);
    for i in 0..n{dot+=a[i]*b[i];aa+=a[i]*a[i];bb+=b[i]*b[i];}
    if aa==0.0||bb==0.0{0.0}else{(dot/(aa.sqrt()*bb.sqrt())).clamp(-1.0,1.0).max(0.0)}
}
