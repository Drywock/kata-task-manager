use std::io;

fn main() {
  /* This does not exist */
  let mut input = String::new();
  let mut tasks = Vec::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read user input");
  let action = parse_action(&input);
  println!("{:?}", action);
  update_task_list(&mut tasks, action);
}

#[derive(Debug, PartialEq)]
enum Action {
  Add(String),
  Quit,
  Remove(usize),
  SetToDo(usize),
  SetDone(usize),
}

fn parse_action(input: &str) -> Action {
  let mut parts = input.split_whitespace();
  let action_str = parts.next();
  let value = parts.next();
  match action_str {
    Some("+") => Action::Add(String::from(value.unwrap())),
    Some("-") => {
      let value: Result<usize, _> = value.unwrap().parse();
      Action::Remove(value.unwrap() - 1)
    }
    Some("o") => {
      let value: Result<usize, _> = value.unwrap().parse();
      Action::SetToDo(value.unwrap() - 1)
    }
    Some("x") => {
      let value: Result<usize, _> = value.unwrap().parse();
      Action::SetDone(value.unwrap() - 1)
    }
    Some("q") => Action::Quit,
    _ => Action::Quit,
  }
}

#[derive(Debug, PartialEq)]
struct Task {
  description: String,
  is_done: bool,
}

fn update_task_list(tasks: &mut Vec<Task>, action: Action) {
  match action {
    Action::Add(description) => tasks.push(Task {
      description,
      is_done: false,
    }),
    Action::SetDone(id) => {
      if let Some(task) = tasks.get_mut(id) {
        task.is_done = true
      }
    }
    Action::SetToDo(id) => {
      if let Some(task) = tasks.get_mut(id) {
        task.is_done = false
      }
    }
    Action::Remove(id) => {
      tasks.remove(id);
    }
    Action::Quit => (),
  }
}

fn get_string_representation_from_task_list(tasks: &Vec<Task>) -> String {
  if tasks.is_empty() {
    String::from("No task yet")
  } else {
    let mut result = String::new();
    let mut index = 1;
    for task in tasks {
      let is_done_char = if task.is_done { "x" } else { " " };
      result += &format!("{} [{}] {}\n", index, is_done_char, task.description);
      index += 1;
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_parse_quit_action() {
    assert_eq!(Action::Quit, parse_action("q"))
  }

  #[test]
  fn should_parse_add_action() {
    assert_eq!(Action::Add(String::from("tests")), parse_action("+ tests"))
  }

  #[test]
  fn should_parse_remove_action() {
    assert_eq!(Action::Remove(0), parse_action("- 1"))
  }

  #[test]
  fn should_parse_set_todo_action() {
    assert_eq!(Action::SetToDo(0), parse_action("o 1"))
  }

  #[test]
  fn should_parse_set_done_action() {
    assert_eq!(Action::SetDone(0), parse_action("x 1"))
  }

  #[test]
  fn should_update_list_add() {
    let mut tasks: Vec<Task> = Vec::new();
    let expected_task = Task {
      description: String::from("Test"),
      is_done: false,
    };
    update_task_list(&mut tasks, Action::Add(String::from("Test")));
    assert_eq!(tasks.get(0).unwrap(), &expected_task)
  }

  #[test]
  fn should_update_list_done() {
    let mut tasks: Vec<Task> = Vec::new();
    tasks.push(Task {
      description: String::from("Test"),
      is_done: false,
    });
    let expected_task = Task {
      description: String::from("Test"),
      is_done: true,
    };
    update_task_list(&mut tasks, Action::SetDone(0));
    assert_eq!(tasks.get(0).unwrap(), &expected_task)
  }

  #[test]
  fn should_update_list_to_do() {
    let mut tasks: Vec<Task> = Vec::new();
    tasks.push(Task {
      description: String::from("Test"),
      is_done: true,
    });
    let expected_task = Task {
      description: String::from("Test"),
      is_done: false,
    };
    update_task_list(&mut tasks, Action::SetToDo(0));
    assert_eq!(tasks.get(0).unwrap(), &expected_task)
  }

  #[test]
  fn should_update_list_remove() {
    let mut tasks: Vec<Task> = Vec::new();
    tasks.push(Task {
      description: String::from("Test"),
      is_done: true,
    });
    update_task_list(&mut tasks, Action::Remove(0));
    assert_eq!(tasks.len(), 0);
  }
  #[test]
  fn should_display_list() {
    let mut tasks: Vec<Task> = Vec::new();
    tasks.push(Task {
      description: String::from("Test 1"),
      is_done: true,
    });
    tasks.push(Task {
      description: String::from("Test 2"),
      is_done: false,
    });
    assert_eq!(
      String::from("1 [x] Test 1\n2 [ ] Test 2\n"),
      get_string_representation_from_task_list(&tasks)
    );
  }

  #[test]
  fn should_display_empty_list() {
    let tasks: Vec<Task> = Vec::new();
    assert_eq!(
      String::from("No task yet"),
      get_string_representation_from_task_list(&tasks)
    )
  }
}
