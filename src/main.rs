use std::io::{ErrorKind};
use std::path::Path;
use clap::{App, Arg};
use std::fs::OpenOptions;
use std::fs::{DirBuilder};
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    let matches = App::new("Rusty Do")
        .version("1.0")
        .author("Ikeoha Chidi. <someonenew@gmail.com>")
        .about("A todo written in the terminal")
        .arg(Arg::with_name("new")
			.short("n")
			.long("new")
			.value_name("STRING")
			.takes_value(true)
			.help("Creates a new todo"))
		.arg(Arg::with_name("list")
			.short("l")
			.long("list")
			.help("lists out all the tasks"))
		.arg(Arg::with_name("done")
			.short("d")
			.long("done")
			.value_name("NUMBER")
			.takes_value(true)
			.help("Sets a task to done"))
		.arg(Arg::with_name("undone")
			.short("u")
			.long("undone")
			.value_name("NUMBER")
			.takes_value(true)
			.help("Sets a task to undone"))
		.get_matches();
		
		if let Some(value) = matches.value_of("new") {
			create_file(&value);
		}
		if let Some(value) = matches.value_of("done") {
			set_to(true, value.parse::<usize>().unwrap());
		}
		if let Some(value) = matches.value_of("undone") {
			set_to(false, value.parse::<usize>().unwrap());
		}

		if matches.is_present("list") {
			list_tasks()
		}

}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Task {
	tasks: Vec<SubTask> 
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct SubTask {
	status: bool,
	text: String,
}

fn list_tasks() {
	let path = Path::new("./todo/todo.json");
	let file = OpenOptions::new()
		.read(true)
		.open(path);

	match file {
		Ok(mut reader) => {
			let mut content = String::new();
			reader.read_to_string(&mut content).unwrap();

			let result: Task = serde_json::from_str(&content).unwrap_or_default();
			if result.tasks.len() == 0 {
				println!("You have not tasks");
			} else {
				println!("Tasks");
				for i in result.tasks {
					let s = if i.status {"👏"} else {"👉"};
					println!("{} {}", s, i.text)
				}	
			}
		},
		Err(err) => {
			println!("an error occured {}", err);
			if err.kind() == ErrorKind::NotFound {
				DirBuilder::new()
					.recursive(true)
					.create(Path::new("./todo")).unwrap();

				println!("You haven't made any tasks");
			}
		}
	}
}

fn set_to(status: bool, index: usize) {
	let path = Path::new("./todo/todo.json");
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.append(true)
		.open(path);

	match file {
		Ok(mut reader) => {
			let mut content = String::new();

			reader.read_to_string(&mut content).unwrap();

			let mut result: Task = serde_json::from_str(&content).unwrap_or_default();

			if result.tasks.len() == 0 {
				println!("You have no tasks");
			} else {
				result.tasks[index].status = status;
				reader.set_len(0).unwrap();
				serde_json::to_writer(reader, &result).unwrap();
			}
		},
		Err(_) => {}
	}
}

fn create_file(task: &str) {
	let path = Path::new("./todo/todo.json");
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.append(true)
		.create(true)
		.open(path);

	match file {
		Ok(mut reader) => {
			let mut content = String::new();

			reader.read_to_string(&mut content).unwrap();

			let mut result: Task = serde_json::from_str(&content).unwrap_or_default();
			
			result.tasks.push(SubTask{
				status: false,
				text: task.to_owned() 
			});
			// clear out the file content
			reader.set_len(0).unwrap();
			serde_json::to_writer(reader, &result).unwrap();
		},
		Err(err) => {
			println!("error occured opening file: {}", err);
			if err.kind() == ErrorKind::NotFound {
				DirBuilder::new()
					.recursive(true)
					.create(Path::new("./todo")).unwrap();

				create_file(task)
			}
		}
	}
}