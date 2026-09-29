use crate::planning::PlanStep;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Authority { ObserveOnly, Reversible, ExternalWrite, Irreversible }

#[derive(Clone, Debug, PartialEq)]
pub struct ActionProposal {
    pub step:PlanStep,
    pub authority:Authority,
    pub rationale_confidence:f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionDecision { Allowed(ActionProposal), NeedsApproval(ActionProposal), Denied(&'static str) }

#[derive(Clone, Debug)]
pub struct CuTranPolicy {
    pub allow_external_write:bool,
    pub allow_irreversible:bool,
    pub min_confidence:f32,
}

impl Default for CuTranPolicy {
    fn default()->Self{Self{allow_external_write:false,allow_irreversible:false,min_confidence:0.55}}
}

impl CuTranPolicy {
    pub fn evaluate(&self, proposal:ActionProposal)->ActionDecision{
        if proposal.rationale_confidence<self.min_confidence{return ActionDecision::Denied("low confidence")}
        match proposal.authority {
            Authority::ObserveOnly|Authority::Reversible=>ActionDecision::Allowed(proposal),
            Authority::ExternalWrite if self.allow_external_write=>ActionDecision::Allowed(proposal),
            Authority::ExternalWrite=>ActionDecision::NeedsApproval(proposal),
            Authority::Irreversible if self.allow_irreversible=>ActionDecision::NeedsApproval(proposal),
            Authority::Irreversible=>ActionDecision::Denied("irreversible action not authorized"),
        }
    }
}
