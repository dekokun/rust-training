use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is get lock {}.", self.name, self.left);
        println!("{} is get lock {}.", self.name, self.right);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(2000));
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(())],
    });
    let philosophers = vec![Philosopher::new("Judith Butler", 0, 1),
                            Philosopher::new("Gilles Deleuze", 1, 2),
                            Philosopher::new("Karl Marx", 2, 3),
                            Philosopher::new("Emma Goldman", 3, 4),
                            Philosopher::new("Michel Foucault", 0, 4)];
    let handles: Vec<_> = philosophers.into_iter()
        .map(|p| {
            let table = table.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(110));
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
