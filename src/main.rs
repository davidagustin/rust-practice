use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rust-todo")]
#[command(about = "A simple CLI to-do list application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task to the to-do list
    Add {
        /// The task description
        description: String,
    },
    /// List all tasks
    List {
        /// Show only completed tasks
        #[arg(short, long)]
        completed: bool,
        /// Show only pending tasks
        #[arg(short, long)]
        pending: bool,
    },
    /// Mark a task as complete
    Complete {
        /// The ID of the task to complete
        id: usize,
    },
    /// Delete a task
    Delete {
        /// The ID of the task to delete
        id: usize,
    },
    /// Clear all tasks
    Clear {
        /// Confirm clearing all tasks
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    created_at: String,
}

struct TodoList {
    tasks: Vec<Task>,
    file_path: PathBuf,
}

impl TodoList {
    fn new() -> Self {
        let file_path = Self::get_data_path();
        let tasks = Self::load_tasks(&file_path);
        TodoList { tasks, file_path }
    }

    fn get_data_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(home);
        path.push(".rust-todo.json");
        path
    }

    fn load_tasks(path: &PathBuf) -> Vec<Task> {
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
                Err(_) => vec![],
            }
        } else {
            vec![]
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn add_task(&mut self, description: String) -> Result<(), Box<dyn std::error::Error>> {
        let id = self
            .tasks
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0)
            + 1;
        let task = Task {
            id,
            description,
            completed: false,
            created_at: chrono::Local::now().to_rfc3339(),
        };
        self.tasks.push(task);
        self.save()?;
        println!("✓ Task added successfully!");
        Ok(())
    }

    fn list_tasks(&self, show_completed: bool, show_pending: bool) {
        let filtered_tasks: Vec<&Task> = if show_completed {
            self.tasks.iter().filter(|t| t.completed).collect()
        } else if show_pending {
            self.tasks.iter().filter(|t| !t.completed).collect()
        } else {
            self.tasks.iter().collect()
        };

        if filtered_tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        println!("\n📋 Your To-Do List:\n");
        for task in filtered_tasks {
            let status = if task.completed { "✓" } else { " " };
            let checkbox = if task.completed { "[x]" } else { "[ ]" };
            println!("{} {} {} - {}", checkbox, status, task.id, task.description);
        }
        println!();
    }

    fn complete_task(&mut self, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if task.completed {
                println!("Task {} is already completed.", id);
            } else {
                task.completed = true;
                self.save()?;
                println!("✓ Task {} marked as complete!", id);
            }
        } else {
            println!("Task with ID {} not found.", id);
        }
        Ok(())
    }

    fn delete_task(&mut self, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() < initial_len {
            self.save()?;
            println!("✓ Task {} deleted successfully!", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
        Ok(())
    }

    fn clear_all(&mut self, confirmed: bool) -> Result<(), Box<dyn std::error::Error>> {
        if !confirmed {
            println!("⚠️  This will delete all tasks. Use --yes to confirm.");
            return Ok(());
        }
        let count = self.tasks.len();
        self.tasks.clear();
        self.save()?;
        println!("✓ Cleared {} task(s).", count);
        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list = TodoList::new();

    let result = match cli.command {
        Commands::Add { description } => todo_list.add_task(description),
        Commands::List { completed, pending } => {
            todo_list.list_tasks(completed, pending);
            Ok(())
        }
        Commands::Complete { id } => todo_list.complete_task(id),
        Commands::Delete { id } => todo_list.delete_task(id),
        Commands::Clear { yes } => todo_list.clear_all(yes),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
