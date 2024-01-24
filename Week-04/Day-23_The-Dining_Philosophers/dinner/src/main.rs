use std::sync::{Arc, Mutex};
use std::thread;
use semaphore::Semaphore;

struct Philosopher {
    id: u32,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    for _ in 0..100 {
        take_dinner();
    }
}

fn take_dinner() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophers = vec![
        crete_philosopher(1, 0, 1),
        crete_philosopher(2, 1, 2),
        crete_philosopher(3, 2, 3),
        crete_philosopher(4, 3, 4),
        crete_philosopher(5, 4, 0),
    ];

    let sem = Arc::new(Semaphore::new(philosophers.len() - 1, ()));
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|item| {
            let table = table.clone();
            let sem = sem.clone();

            thread::spawn(move || loop {
                if sem.try_access().is_ok() {
                    wants_to_eat(&item, &table, pick_left_fork, pick_right_fork, eat, put_left_fork, put_right_fork);
                    break;
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
fn crete_philosopher(id: u32, left:usize, right: usize) -> Philosopher {
    Philosopher { id, left, right, }
}

fn pick_left_fork(philosopher: &Philosopher, table: &Table) {
    let _unused = table.forks[philosopher.left].lock().unwrap();
    print!("{:?},",vec![philosopher.id,1,1]);
}

fn pick_right_fork(philosopher: &Philosopher, table: &Table) {
    let _unused = table.forks[philosopher.right].lock().unwrap();
    print!("{:?},",vec![philosopher.id,2,1]);
}

fn eat(philosopher: &Philosopher) {
    print!("{:?},",vec![philosopher.id,2,3]);
}

fn put_left_fork(philosopher: &Philosopher) {
    print!("{:?},",vec![philosopher.id,1,2]);
}

fn put_right_fork(philosopher: &Philosopher) {
    print!("{:?},",vec![philosopher.id,2,2]);
}

fn wants_to_eat(philosopher: &Philosopher, table: &Table,
    pick_left_fork: fn(&Philosopher, &Table),
    pick_right_fork: fn(&Philosopher, &Table),
    eat: fn(&Philosopher),
    put_left_fork: fn(&Philosopher),
    put_right_fork: fn(&Philosopher),
) {
    pick_left_fork(philosopher, table);
    pick_right_fork(philosopher,table);
    eat(philosopher);
    put_left_fork(philosopher);
    put_right_fork(philosopher);
}
