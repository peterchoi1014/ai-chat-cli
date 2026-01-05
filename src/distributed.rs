use anyhow::Result;
use repartir::{Pool, task::{Task, Backend}};

pub struct DistributedAI {
    pool: Pool,
}

impl DistributedAI {
    pub async fn new() -> Result<Self> {
        // Configure for distributed execution
        let pool = Pool::builder()
            .cpu_workers(4)
            // In future versions, add remote workers:
            // .remote_worker("192.168.1.100:8080")
            // .remote_worker("192.168.1.101:8080")
            .build()?;

        Ok(Self { pool })
    }

    pub async fn parallel_inference(&self, prompts: Vec<String>) -> Result<Vec<String>> {
        let mut results = Vec::new();

        for prompt in prompts {
            let task = Task::builder()
                .binary("/bin/sh")
                .arg("-c")
                .arg(format!(
                    "curl -s http://localhost:11434/api/generate -d '{{\"model\":\"llama3.2:1b\",\"prompt\":\"{}\",\"stream\":false}}'",
                    prompt.replace('\'', "'\\''")
                ))
                .backend(Backend::Cpu)
                .build()?;

            let result = self.pool.submit(task).await?;
            
            if result.is_success() {
                results.push(result.stdout_str()?.to_string());
            }
        }

        Ok(results)
    }
}
