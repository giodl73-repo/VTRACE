import init, {evaluate} from './pkg/vtrace_package_inspector.js';
try {
  await init();
  self.onmessage=({data})=>{
    try { self.postMessage({type:'result',id:data.id,report:JSON.parse(evaluate(data.request))}); }
    catch(error) { self.postMessage({type:'failure',id:data.id,message:String(error)}); }
  };
  // The Rust admission contract supplies UI filenames; no JS rule catalog.
  const artifactNames=JSON.parse(evaluate('{"files":[]}')).accepted_artifact_names;
  self.postMessage({type:'ready',artifactNames});
} catch(error) { self.postMessage({type:'failure',message:String(error)}); }
