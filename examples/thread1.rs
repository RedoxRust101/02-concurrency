use anyhow::{anyhow, Ok, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    // 释放tx，否则rx会一直等待
    drop(tx);

    // 创建consumers
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!(
                "consumer: Msg {{ idx: {:?}, value: {:?} }}",
                msg.idx, msg.value
            );
        }
        println!("consumer: done");
        42
    });

    // 等待consumer线程结束
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Consumer thread panicked: {:?}", e))?;
    println!("secret: {}", secret);

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = (rand::random::<u8>() as u64) * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        // random exit the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    // more things to do
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
