
 //Introducing project workflow
 #[derive(Debug, Serialize, Deserialize)]
 pub struct Workflow {
     id: i32,
     name: String,
     stages: Vec<WorkflowStage>,
     description: Option<String>,
     owner_id: i32, // ID of the user who created the workflow
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct WorkflowStage {
     id: i32,
     workflow_id: i32,
     name: String,
     description: Option<String>,
     status: WorkflowStageStatus,
     conditions: Vec<WorkflowCondition>, // Conditions for entering the stage
     actions: Vec<WorkflowAction>, // Actions to be performed when entering the stage
 }
 
 
 #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
 pub enum WorkflowStageStatus {
     Active,
     Completed,
     Skipped,
 }
 
 #[derive(Debug, Serialize, Deserialize)]
 pub struct WorkflowGraph {
     nodes: Vec<WorkflowStage>,
     edges: Vec<(i32, i32)>, // Edges representing transitions between stages
 }