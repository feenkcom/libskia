use glutin::{NotCurrent, PossiblyCurrent};
use skia_safe::gpu::gl::Interface;
use skia_safe::gpu::Context;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>, gpu_context: Option<&mut Context>);
}

impl<F: FnOnce(Option<&mut Context>)> FnBox for F {
    fn call_box(self: Box<F>, gpu_context: Option<&mut Context>) {
        (*self)(gpu_context)
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize, contexts: Option<Vec<glutin::Context<NotCurrent>>>) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        let mut contexts_vec = match contexts {
            None => vec![],
            Some(vec) => vec,
        };
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), contexts_vec.pop()));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(Option<&mut Context>) + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn join(mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct GpuContext {
    pub glutin_context: glutin::Context<PossiblyCurrent>,
    pub skia_context: skia_safe::gpu::Context,
}

impl GpuContext {
    pub fn from_headless_context(
        headless_context: Option<glutin::Context<NotCurrent>>,
    ) -> Option<Self> {
        match headless_context {
            None => None,
            Some(headless_context) => match unsafe { headless_context.make_current() } {
                Ok(current_context) => {
                    match Interface::new_load_with(|symbol| {
                        current_context.get_proc_address(symbol)
                    }) {
                        Some(interface) => match skia_safe::gpu::Context::new_gl(interface) {
                            None => None,
                            Some(skia_context) => Some(Self {
                                glutin_context: current_context,
                                skia_context,
                            }),
                        },
                        None => None,
                    }
                }
                Err(err) => {
                    println!("[GpuContext::from_headless_context] Error: {:?}", err);
                    None
                }
            },
        }
    }
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        context: Option<glutin::Context<NotCurrent>>,
    ) -> Worker {
        let thread = thread::spawn(move || {
            let mut gpu_context = GpuContext::from_headless_context(context);

            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        let context = match gpu_context.as_mut() {
                            None => None,
                            Some(gpu_context) => Some(&mut gpu_context.skia_context),
                        };
                        job.call_box(context);
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> usize {
        self.id
    }
}
