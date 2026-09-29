use crate::action::{ActionDecision, ActionProposal, CuTranPolicy};
use crate::budget::DeviceState;
use crate::core::BiaDca;
use crate::language::VietnameseGate;
use crate::types::CognitiveMoment;

#[derive(Clone, Debug, PartialEq)]
pub struct MobileReply {
    pub text:String,
    pub moment:CognitiveMoment,
}

pub struct OfflineMobileBia {
    pub bia:BiaDca,
    pub language:VietnameseGate,
    pub policy:CuTranPolicy,
}

impl OfflineMobileBia {
    pub fn new(bia:BiaDca)->Self{
        Self{bia,language:VietnameseGate,policy:CuTranPolicy::default()}
    }

    pub fn converse(&mut self,text:&str,timestamp:u64,device:DeviceState)->Option<MobileReply>{
        let mut ps=self.language.perceive(text,timestamp);
        let focus=ps.pop()?;
        for p in ps { self.bia.observe(p); }
        let moment=self.bia.contemplate(focus,device,0.7);
        let reply=self.language.express(&moment.recognition);
        Some(MobileReply{text:reply,moment})
    }

    pub fn authorize(&self,proposal:ActionProposal)->ActionDecision{
        self.policy.evaluate(proposal)
    }
}
