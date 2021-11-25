use futures::channel::mpsc;
use futures::stream::Stream;
use gloo_timers::callback::Timeout;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

// ------ BackoffStream ------

/// [Truncated exponential backoff](https://cloud.google.com/storage/docs/exponential-backoff)
#[derive(Debug)]
pub struct BackoffStream {
    max_seconds: u32,
    retries: usize,
    timeout: Timeout,
    tick_sender: Rc<mpsc::UnboundedSender<()>>,
    tick_receiver: mpsc::UnboundedReceiver<()>,
}

impl BackoffStream {
    pub fn new(max_seconds: u32) -> Self {
        let (tick_sender, tick_receiver) = mpsc::unbounded();
        let tick_sender = Rc::new(tick_sender);

        let retries = 0;
        Self {
            max_seconds,
            retries,
            timeout: start_timeout(wait_time(retries, max_seconds), &tick_sender),
            tick_sender,
            tick_receiver,
        }
    }
}

impl Stream for BackoffStream {
    type Item = usize;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match Stream::poll_next(Pin::new(&mut self.tick_receiver), cx) {
            Poll::Ready(Some(_)) => {
                self.retries += 1;
                self.timeout =
                    start_timeout(wait_time(self.retries, self.max_seconds), &self.tick_sender);
                Poll::Ready(Some(self.retries))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn wait_time(retries: usize, max_seconds: u32) -> u32 {
    let retries = u32::try_from(retries).unwrap_or(u32::max_value());
    let random_ms = SmallRng::from_entropy().gen_range(0..=1000);

    let duration = 2_u32
        .saturating_pow(retries)
        .saturating_mul(1000)
        .saturating_add(random_ms);
    let max_duration = max_seconds.saturating_mul(1000);

    u32::min(duration, max_duration)
}

fn start_timeout(ms: u32, tick_sender: &Rc<mpsc::UnboundedSender<()>>) -> Timeout {
    let tick_sender = Rc::clone(tick_sender);
    Timeout::new(ms, move || {
        tick_sender.unbounded_send(()).expect("send backoff tick");
    })
}
