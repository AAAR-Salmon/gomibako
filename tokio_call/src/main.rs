use std::fs::File;
use std::io::{stdout, Write};
use std::time::{self, Duration};

use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize, Deserialize, Clone)]
struct Member {
    name: String,
    age: u32,
    instrument: String,
}

impl Member {
    async fn introduce(&self) {
        let str = format!(
            "{}，{}歳❗ {}担当です❗❗",
            &self.name, &self.age, &self.instrument
        );
        let dur = time::Duration::from_millis(100);
        typewriter(str, dur).await;
    }
}

async fn typewriter(str: String, dur: Duration) {
    let mut stdout = stdout();
    for c in str.chars() {
        print!("{}", c);
        stdout.flush().unwrap();
        tokio::time::sleep(dur).await;
    }
    println!();
}

#[tokio::main]
async fn main() {
    let file = File::open("members.yml").unwrap();
    let members: Vec<Member> = serde_yaml::from_reader(file).unwrap();

    let mut tasks = Vec::with_capacity(members.len());
    for (i, member) in members.iter().enumerate() {
        let member = member.clone();
        let task = tokio::spawn(async move {
            let sec = time::Duration::from_secs(i.try_into().unwrap());
            tokio::time::sleep(sec).await;
            member.introduce().await;
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}
