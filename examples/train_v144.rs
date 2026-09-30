use bia_core::{general_training,integrated_cognition::IntegratedCognition};
fn main()->std::io::Result<()> {
 let directory=std::env::args().nth(1).unwrap_or_else(||"training-output".into());
 std::fs::create_dir_all(&directory)?;
 let run=general_training::train(&IntegratedCognition::default());
 std::fs::write(format!("{directory}/training.txt"),general_training::curriculum().join("\n"))?;
 std::fs::write(format!("{directory}/report.txt"),&run.report)?;
 std::fs::write(format!("{directory}/checkpoint.bia-cognition"),run.candidate.export())?;
 println!("{}",run.report);
 assert!(run.qualified,"Curriculum gate failed; do not activate");Ok(())
}
