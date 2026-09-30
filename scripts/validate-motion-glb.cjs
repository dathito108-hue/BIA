const fs=require('fs');
const validator=require('gltf-validator');
(async()=>{const report=await validator.validateBytes(new Uint8Array(fs.readFileSync(process.argv[2])),{uri:'motion-v142.glb',maxIssues:1000});fs.writeFileSync(process.argv[3],JSON.stringify(report,null,2));console.log(JSON.stringify(report.issues));if(report.issues.numErrors!==0||report.issues.numWarnings!==0)process.exitCode=1;})().catch(e=>{console.error(e);process.exitCode=1;});
