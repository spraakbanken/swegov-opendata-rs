use crate::Spider;
use futures::stream::StreamExt;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::{mpsc, Barrier},
    time::{sleep, Instant},
};

pub struct Crawler {
    delay: Duration,
    crawling_concurrency: usize,
    processing_concurrency: usize,
}

impl Crawler {
    pub fn new(
        delay: Duration,
        crawling_concurrency: usize,
        processing_concurrency: usize,
    ) -> Self {
        Self {
            delay,
            crawling_concurrency,
            processing_concurrency,
        }
    }

    pub async fn run<T: Send + 'static, E: 'static>(
        &self,
        spider: Arc<dyn Spider<Item = T, Error = E>>,
    ) {
        tracing::info!("running spider '{}'", spider.name());
        let starting_time = Instant::now();
        let mut visited_urls = HashSet::<String>::new();
        let crawling_concurrency = self.crawling_concurrency;
        let crawling_queue_capacity = crawling_concurrency * 400;
        let processing_concurrency = self.processing_concurrency;
        let processing_queue_capacity = processing_concurrency * 10;
        let active_spiders = Arc::new(AtomicUsize::new(0));

        // Statistics
        let num_scrapings = Arc::new(AtomicUsize::new(0));
        let num_scrape_errors = Arc::new(AtomicUsize::new(0));
        let num_processings = Arc::new(AtomicUsize::new(0));
        let num_process_errors = Arc::new(AtomicUsize::new(0));

        let (urls_to_visit_tx, urls_to_visit_rx) = mpsc::channel(crawling_queue_capacity);
        let (items_tx, items_rx) = mpsc::channel(processing_queue_capacity);
        let (new_urls_tx, mut new_urls_rx) = mpsc::channel(crawling_queue_capacity);
        let barrier = Arc::new(Barrier::new(3));

        for url in spider.start_urls() {
            tracing::info!(start_url = url);
            visited_urls.insert(url.clone());
            let _ = urls_to_visit_tx.send(url).await;
        }

        self.launch_processors(
            processing_concurrency,
            num_processings.clone(),
            num_process_errors.clone(),
            spider.clone(),
            items_rx,
            barrier.clone(),
        );

        self.launch_scrapers(
            crawling_concurrency,
            num_scrapings.clone(),
            num_scrape_errors.clone(),
            spider.clone(),
            urls_to_visit_rx,
            new_urls_tx.clone(),
            items_tx,
            active_spiders.clone(),
            self.delay,
            barrier.clone(),
        );

        loop {
            if let Ok((visited_url, new_urls)) = new_urls_rx.try_recv() {
                visited_urls.insert(visited_url);

                for url in new_urls {
                    if !visited_urls.contains(&url) {
                        visited_urls.insert(url.clone());
                        tracing::debug!("queueing: {}", url);
                        let _ = urls_to_visit_tx.send(url).await;
                    }
                }
            }

            if new_urls_tx.capacity() == crawling_queue_capacity // new_urls channel is empty
            && urls_to_visit_tx.capacity() == crawling_queue_capacity // urls_to_visit channel is empty
            && active_spiders.load(Ordering::SeqCst) == 0
            {
                // no more work, we leave
                break;
            }

            sleep(Duration::from_millis(5)).await;
        }

        tracing::info!("crawler: control loop exited");

        // we drop the transmitter in order to close the stream
        drop(urls_to_visit_tx);

        // and then we wait for the streams to complete
        barrier.wait().await;

        let num_procs = num_processings.load(Ordering::Relaxed);
        let num_proc_errors = num_process_errors.load(Ordering::Relaxed);
        let num_scrapes = num_scrapings.load(Ordering::Relaxed);
        let num_scrap_errors = num_scrape_errors.load(Ordering::Relaxed);
        let total_running_time = format!("{:?}", starting_time.elapsed());
        tracing::info!(
            num_processings = num_procs,
            num_process_errors = num_proc_errors,
            num_scrapings = num_scrapes,
            num_scrape_errors = num_scrap_errors,
            running_time = total_running_time,
            "statistics"
        );
    }

    fn launch_processors<T: Send + 'static, E: 'static>(
        &self,
        concurrency: usize,
        num_processings: Arc<AtomicUsize>,
        num_process_errors: Arc<AtomicUsize>,
        spider: Arc<dyn Spider<Item = T, Error = E>>,
        items: mpsc::Receiver<T>,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(items)
                .for_each_concurrent(concurrency, |item| async {
                    num_processings.fetch_add(1, Ordering::SeqCst);
                    let _ = spider.process(item).await.map_err(|err| {
                        num_process_errors.fetch_add(1, Ordering::SeqCst);
                        // tracing::error!("Processing error: {:?}", err);
                        err
                    });
                })
                .await;

            barrier.wait().await;
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn launch_scrapers<T: Send + 'static, E: 'static>(
        &self,
        concurrency: usize,
        num_scrapings: Arc<AtomicUsize>,
        num_scrape_errors: Arc<AtomicUsize>,
        spider: Arc<dyn Spider<Item = T, Error = E>>,
        urls_to_visit: mpsc::Receiver<String>,
        new_urls_tx: mpsc::Sender<(String, Vec<String>)>,
        items_tx: mpsc::Sender<T>,
        active_spiders: Arc<AtomicUsize>,
        delay: Duration,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(urls_to_visit)
                .for_each_concurrent(concurrency, |queued_url| {
                    let queued_url = queued_url;
                    async {
                        active_spiders.fetch_add(1, Ordering::SeqCst);
                        let mut urls = Vec::new();
                        num_scrapings.fetch_add(1, Ordering::SeqCst);
                        let res = spider
                            .scrape(queued_url.clone())
                            .await
                            .map_err(|err| {
                                num_scrape_errors.fetch_add(1, Ordering::SeqCst);
                                err
                            })
                            .ok();

                        if let Some((items, new_urls)) = res {
                            for item in items {
                                let _ = items_tx.send(item).await;
                            }
                            urls = new_urls;
                        }

                        let _ = new_urls_tx.send((queued_url, urls)).await;
                        sleep(delay).await;
                        active_spiders.fetch_sub(1, Ordering::SeqCst);
                    }
                })
                .await;

            drop(items_tx);
            barrier.wait().await;
        });
    }
}
