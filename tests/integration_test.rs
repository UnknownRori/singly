use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

use lazy_static::lazy_static;
use rand::Rng;

use singly::Singleton;

struct ExamScore(Vec<i32>);

// ------ For Normal situation in Multi Threaded Environment ---------

#[test]
fn multi_thread_situation_reading() {
    let mut instance = Singleton::new();
    let scores: Vec<_> = (0..200)
        .map(|_| {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..100)
        })
        .collect();

    let expected_sum = scores.iter().fold(0, |acc, score| acc + score);
    let expected_mean = expected_sum / scores.len() as i32;

    instance.set(Arc::new(ExamScore(scores)));

    let scores_clone1 = Arc::clone(instance.get::<Arc<ExamScore>>());
    let scores_clone2 = Arc::clone(instance.get::<Arc<ExamScore>>());

    let thread1 = spawn(move || (*scores_clone1).0.iter().fold(0, |acc, score| acc + score));
    let thread2 = spawn(move || {
        (*scores_clone2).0.iter().fold(0, |acc, score| acc + score) / scores_clone2.0.len() as i32
    });

    let result = thread1.join().unwrap();
    assert_eq!(expected_sum, result);

    let result = thread2.join().unwrap();
    assert_eq!(expected_mean, result);
}

struct Counter(i32);

type ArcMutexCounter = Arc<Mutex<Counter>>;

#[test]
fn multi_thread_situation_counter() {
    let mut instance = Singleton::new();
    let counter = Arc::new(Mutex::new(Counter(0)));
    instance.set(counter);

    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(instance.get::<ArcMutexCounter>());
        let handle = spawn(move || {
            let mut counter = counter_clone.lock().unwrap();
            (*counter).0 += 1;
        });
        handles.push(handle);
    }

    let _ = handles
        .into_iter()
        .map(|handle| handle.join())
        .collect::<Result<Vec<_>, _>>();

    let counter = instance.get::<ArcMutexCounter>().lock().unwrap().0;
    assert_eq!(counter, 10);
}

// ------ For Static Usage in Multi Threaded Environment ---------

lazy_static! {
    static ref SINGLETON_INSTANCE: Mutex<Singleton> = Mutex::new(Singleton::new());
}

#[test]
fn static_test() {
    let counter = Counter(0);
    SINGLETON_INSTANCE.lock().unwrap().set(counter);

    let mut handles = vec![];
    for _ in 0..10 {
        let handle = spawn(move || {
            let mut instance = SINGLETON_INSTANCE.lock().unwrap();
            instance
                .try_get_mut::<Counter>()
                .map(|counter| (*counter).0 += 1);
        });
        handles.push(handle);
    }

    let _ = handles
        .into_iter()
        .map(|handle| handle.join())
        .collect::<Result<Vec<_>, _>>();

    let counter = SINGLETON_INSTANCE.lock().unwrap().get::<Counter>().0;
    assert_eq!(counter, 10);
}
