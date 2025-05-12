mod error;
mod http;
mod types;

use std::path::PathBuf;
use std::sync::Arc;

use crate::http::{
    download, get_file_size, make_http_client, segregrated_download, simple_download, Client,
};
pub use error::{MetalinkDownloadError, Result};
use http::request_range;
use reqwest_middleware::ClientWithMiddleware;
use tokio::io::AsyncSeekExt;
use tokio::io::AsyncWriteExt;
use types::CheckSum;
use types::ChunkMetaData;
use types::ProgressUpdate;
use types::{FilePlan, Plan};

use anyhow::{anyhow, Context};
use futures::StreamExt;
use log::info;
use tokio::task::JoinHandle;

pub async fn download_metalink(
    metalink_file: PathBuf,
    target_dir: PathBuf,
    user_agent: String,
    verify_chunk_checksums: bool,
) -> Result<()> {
    log::info!("==========Start Metalink Download==========");
    let plan = Plan::new(metalink_file, &target_dir)?.minimize_plan()?;

    let client = make_http_client(user_agent)?;
    let total_size = plan.total_size;
    let (prog_tx, prog_rx) = tokio::sync::mpsc::unbounded_channel::<ProgressUpdate>();
    let progress_reporter: JoinHandle<Result<()>> =
        tokio::spawn(async move { progress_reporter_task(prog_rx, total_size).await });

    let tracker = tokio_util::task::TaskTracker::new();
    for file in plan.files {
        let cloned_file = file.clone();
        let cloned_tx = prog_tx.clone();
        let cloned_client = client.clone();
        tracker.spawn(async move {
            let _ = download_file_task(
                &cloned_client,
                &cloned_file,
                &cloned_tx,
                verify_chunk_checksums,
            )
            .await;
        });
    }
    tracker.close();
    tracker.wait().await;

    prog_tx
        .send(ProgressUpdate::Finished)
        .with_context(|| "Failed to send finish progress command")?;
    progress_reporter
        .await
        .with_context(|| "Progress Reporter failed")??;

    Ok(())
}

async fn download_file_task(
    client: &Client,
    file: &FilePlan,
    tx: &tokio::sync::mpsc::UnboundedSender<ProgressUpdate>,
    verify_chunk_checksums: bool,
) -> Result<()> {
    log::info!("Start downloading: {:?}", file.target_file);
    if let Some(chunks) = file.chunks.as_ref() {
        download(
            client,
            file.url.clone(),
            file.target_file.clone(),
            chunks,
            Some(tx.clone()),
            verify_chunk_checksums,
        )
        .await
        .with_context(|| format!("Parallel download of {:?} failed", file.target_file))?;
    } else {
        simple_download(client, file.url.clone(), file.target_file.clone())
            .await
            .with_context(|| format!("Simple download of {:?} failed", file.target_file))?;
    }
    log::info!("Finish downloading: {:?}", file.target_file);
    Ok(())
}

async fn progress_reporter_task(
    mut _prog_rx: tokio::sync::mpsc::UnboundedReceiver<ProgressUpdate>,
    _total_size: u64,
) -> Result<()> {
    // let pb = ProgressBar::new(total_size);
    // pb.set_style(
    //         ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    //             .unwrap()
    //             .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    //             .progress_chars("#>-"));
    // let mut bytes_downloaded = 0;
    // while let Some(cmd) = prog_rx.recv().await {
    //     match cmd {
    //         ProgressUpdate::Progressed(bytes) => {
    //             bytes_downloaded += bytes;
    //             pb.set_position(bytes_downloaded);
    //         }
    //         ProgressUpdate::Finished => break,
    //     }
    // }
    //
    // pb.finish_with_message("Download Finished");
    // Ok(())
    Ok(())
}

pub async fn plan(metalink_file: PathBuf, target_dir: PathBuf) -> Result<()> {
    info!("File: {metalink_file:?}, Target: {target_dir:?}");
    let plan = Plan::new(metalink_file, &target_dir)?;
    println!("{plan:#?}");

    let minimized_plan = plan.minimize_plan()?;
    println!("{minimized_plan:#?}");
    Ok(())
}

const ONE_MB: u64 = 1_048_576;
const DEFAULT_CHUNK_SIZE: u64 = 512000;

pub async fn download_file(
    url: url::Url,
    target_dir: PathBuf,
    user_agent: String,
    max_threads: u16,
) -> Result<()> {
    let client = make_http_client(user_agent)?;
    let url = reqwest::Url::parse(url.as_str())?;
    let path = PathBuf::from(url.path());
    let file_name = path
        .file_name()
        .ok_or(anyhow!("Unable to extract file path from url"))?;
    let target_file = target_dir.join(file_name);

    match get_file_size(&client, url.clone()).await? {
        Some(size) => {
            if size <= ONE_MB {
                simple_download(&client, url.clone(), target_file).await
            } else {
                let ranges = ChunkMetaData::calculate_ranges(size, ONE_MB, &target_file);
                segregrated_download(
                    &client,
                    url.clone(),
                    target_file,
                    size,
                    &ranges,
                    None,
                    max_threads,
                )
                .await
            }
        }
        None => simple_download(&client, url.clone(), target_file).await,
    }
}

#[derive(Debug)]
enum DownloadState {
    Created,
    Running,
    Paused,
    Finished,
}

#[derive(Debug)]
enum DownloadStateChangeCommand {
    Init,
    Run,
    Pause,
    Resume,
    Cancel,
}

pub trait EventHandler {
    fn on_download_initialized(&self, total_download_size: u64);

    fn on_download_started(&self);

    fn on_download_progressed(&self, bytes_done: u64, total_bytes: u64);

    fn on_download_paused(&self);

    fn on_download_resumed(&self);

    fn on_download_failed(&self, error: MetalinkDownloadError);

    fn on_download_succeeded(&self);

    fn on_download_cancelled(&self);
}

#[derive(Debug, Default)]
struct DefaultHandler;

impl EventHandler for DefaultHandler {
    fn on_download_initialized(&self, total_download_size: u64) {
        log::info!(
            "Download with size {} bytes initialized",
            total_download_size
        );
    }

    fn on_download_progressed(&self, bytes_done: u64, total_bytes: u64) {
        log::info!("{} of {} bytes downloaded", bytes_done, total_bytes);
    }

    fn on_download_failed(&self, error: MetalinkDownloadError) {
        log::error!("Download failed: {}", error);
    }

    fn on_download_started(&self) {
        log::info!("Download started");
    }

    fn on_download_succeeded(&self) {
        log::info!("Download succeeded");
    }

    fn on_download_cancelled(&self) {
        log::info!("Download cancelled");
    }

    fn on_download_paused(&self) {
        log::info!("Download paused");
    }

    fn on_download_resumed(&self) {
        log::info!("Download resumed");
    }
}

pub struct Download {
    metalink_file: PathBuf,
    verify_chunk_checksums: bool,
    target_dir: PathBuf,
    user_agent: Option<String>,
    max_concurrent_threads: u64,
    event_handler: Arc<Box<dyn EventHandler + Send + Sync>>,
    state_change_cmd_rx: tokio::sync::watch::Receiver<DownloadStateChangeCommand>,
    state_change_cmd_tx: tokio::sync::watch::Sender<DownloadStateChangeCommand>,
    download_task: Option<tokio::task::JoinHandle<Result<()>>>,
    max_retries: u64,
}

impl Download {
    pub fn builder() -> DownloadBuilder {
        DownloadBuilder::default()
    }

    pub async fn start(&mut self) -> Result<()> {
        let metalink_file = self.metalink_file.clone();
        let target_dir = self.target_dir.clone();
        let user_agent = self.user_agent.clone();
        let max_concurrent_tasks = self.max_concurrent_threads;
        let state_change_cmd_rx = self.state_change_cmd_rx.clone();
        let verify_chunk_checksums = self.verify_chunk_checksums;
        let max_retries = self.max_retries;
        let event_handler = self.event_handler.clone();
        self.download_task = Some(tokio::spawn(async move {
            download_task(
                metalink_file,
                verify_chunk_checksums,
                target_dir,
                user_agent,
                max_concurrent_tasks,
                max_retries,
                state_change_cmd_rx,
                event_handler,
            )
            .await
        }));
        self.state_change_cmd_tx
            .send(DownloadStateChangeCommand::Run)
            .map_err(|e| anyhow!("{}", e))?;
        Ok(())
    }

    pub async fn pause(&self) -> Result<()> {
        self.state_change_cmd_tx
            .send(DownloadStateChangeCommand::Pause)
            .map_err(|e| anyhow!("{}", e))?;
        Ok(())
    }

    pub async fn resume(&mut self) -> Result<()> {
        self.state_change_cmd_tx
            .send(DownloadStateChangeCommand::Resume)
            .map_err(|e| anyhow!("{}", e))?;
        Ok(())
    }

    pub async fn cancel(&mut self) -> Result<()> {
        self.state_change_cmd_tx
            .send(DownloadStateChangeCommand::Cancel)
            .map_err(|e| anyhow!("{}", e))?;
        Ok(())
    }

    pub async fn wait_for_completion(self) -> Result<()> {
        if let Some(download_task) = self.download_task {
            download_task
                .await
                .with_context(|| "Failed to wait for download task to complete")?
        } else {
            Ok(())
        }
    }
}

#[derive(Default)]
pub struct DownloadBuilder {
    metalink_file: Option<PathBuf>,
    target_dir: Option<PathBuf>,
    user_agent: Option<String>,
    verify_chunk_checksums: bool,
    max_concurrent_threads: Option<u64>,
    max_retries: Option<u64>,
    event_handler: Option<Box<dyn EventHandler + Send + Sync>>,
}

impl DownloadBuilder {
    pub fn with_max_retries(mut self, max_retries: u64) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn with_target_dir(mut self, path: PathBuf) -> Self {
        self.target_dir = Some(path);
        self
    }

    pub fn with_user_agent(mut self, agent: &str) -> Self {
        self.user_agent = Some(agent.to_string());
        self
    }

    pub fn with_metalink_file(mut self, metalink_file: PathBuf) -> Self {
        self.metalink_file = Some(metalink_file);
        self
    }

    pub fn with_chunk_checksum_verification(mut self) -> Self {
        self.verify_chunk_checksums = true;
        self
    }

    pub fn with_max_concurrent_threads(mut self, threads: u64) -> Self {
        self.max_concurrent_threads = Some(threads);
        self
    }

    pub fn with_event_handler(
        mut self,
        handler: impl EventHandler + 'static + Send + Sync,
    ) -> Self {
        self.event_handler = Some(Box::new(handler));
        self
    }

    pub fn build(self) -> Result<Download> {
        if let None = self.target_dir {
            return Err(MetalinkDownloadError::Other(anyhow!(
                "Missing call to with_target_dir."
            )));
        }

        let (tx, rx) = tokio::sync::watch::channel(DownloadStateChangeCommand::Init);

        match self.metalink_file {
            Some(metalink_file) => Ok(Download {
                metalink_file,
                verify_chunk_checksums: self.verify_chunk_checksums,
                target_dir: self.target_dir.unwrap(),
                user_agent: self.user_agent,
                max_concurrent_threads: self.max_concurrent_threads.unwrap_or(1),
                state_change_cmd_tx: tx,
                state_change_cmd_rx: rx,
                download_task: None,
                max_retries: self.max_retries.unwrap_or(0),
                event_handler: Arc::new(
                    self.event_handler
                        .unwrap_or(Box::new(DefaultHandler::default())),
                ),
            }),
            None => Err(MetalinkDownloadError::Other(anyhow!(
                "with_metalink_file needs to be called"
            ))),
        }
    }
}

#[derive(Debug)]
enum WorkerState {
    Created,
    Running,
    Paused,
    Finished,
}

#[derive(Debug)]
struct DownloadChunkData {
    start: u64,
    end: u64,
    url: reqwest::Url,
    target_file: PathBuf,
    checksum: Option<CheckSum>,
}

#[derive(Debug)]
struct DownloadChunkResult {
    start: u64,
    end: u64,
    target_file: PathBuf,
    data: bytes::Bytes,
}

#[derive(Debug)]
struct DownloadWorker {
    state_change_receiver: tokio::sync::watch::Receiver<DownloadStateChangeCommand>,
    chunk_metadata_receiver: std::pin::Pin<Box<async_channel::Receiver<DownloadChunkData>>>,
    result_data_sender: tokio::sync::mpsc::UnboundedSender<DownloadChunkResult>,
    error_data_sender: tokio::sync::mpsc::UnboundedSender<DownloadChunkData>,
    client: ClientWithMiddleware,
    state: WorkerState,
}

impl DownloadWorker {
    pub fn new(
        state_change_receiver: tokio::sync::watch::Receiver<DownloadStateChangeCommand>,
        chunk_metadata_receiver: async_channel::Receiver<DownloadChunkData>,
        result_data_sender: tokio::sync::mpsc::UnboundedSender<DownloadChunkResult>,
        error_data_sender: tokio::sync::mpsc::UnboundedSender<DownloadChunkData>,
        client: ClientWithMiddleware,
    ) -> Self {
        Self {
            state_change_receiver,
            chunk_metadata_receiver: Box::pin(chunk_metadata_receiver),
            result_data_sender,
            error_data_sender,
            client,
            state: WorkerState::Created,
        }
    }

    pub async fn spawn(&mut self) {
        loop {
            match self.state {
                WorkerState::Created => match self.state_change_receiver.changed().await {
                    Ok(()) => match *self.state_change_receiver.borrow_and_update() {
                        DownloadStateChangeCommand::Run => self.state = WorkerState::Running,
                        DownloadStateChangeCommand::Cancel => self.state = WorkerState::Finished,
                        _ => (),
                    },
                    Err(_) => self.state = WorkerState::Finished,
                },
                WorkerState::Running => {
                    tokio::select! {
                        biased;
                        changed = self.state_change_receiver.changed() => {
                            match changed {
                                Ok(()) => {
                                    let cmd = self.state_change_receiver.borrow_and_update();
                                    match *cmd {
                                        DownloadStateChangeCommand::Pause => self.state = WorkerState::Paused,
                                        DownloadStateChangeCommand::Cancel => self.state = WorkerState::Finished,
                                        _ => (),
                                    }
                                }
                                // State changed watch sender has been dropped so get out of here
                                Err(_) => self.state = WorkerState::Finished,
                            }
                        }
                        chunk_metadata = self.chunk_metadata_receiver.next() => {
                            match chunk_metadata {
                                Some(metadata) => {
                                    match self.download_chunk(&metadata).await {
                                        Ok(chunk_result) => self.result_data_sender.send(chunk_result).expect("Result receiver was closed or dropped"),
                                        Err(_) => self.error_data_sender.send(metadata).expect("Error receiver was closed or dropped"),
                                    }
                                },
                                // chunk_metadata channel is closed and empty -> we are done
                                _ => self.state = WorkerState::Finished,
                            }
                        }
                    }
                }
                WorkerState::Paused => match self.state_change_receiver.changed().await {
                    Ok(()) => match *self.state_change_receiver.borrow_and_update() {
                        DownloadStateChangeCommand::Resume => self.state = WorkerState::Running,
                        DownloadStateChangeCommand::Cancel => self.state = WorkerState::Finished,
                        _ => (),
                    },
                    Err(_) => self.state = WorkerState::Finished,
                },
                WorkerState::Finished => {
                    log::info!("Worker finished and shutting down");
                    break;
                }
            }
        }
    }

    async fn download_chunk(&self, chunk: &DownloadChunkData) -> Result<DownloadChunkResult> {
        let data = request_range(&self.client, &chunk.url, chunk.start, chunk.end)
            .await?
            .bytes()
            .await?;
        if let Some(ref checksum) = chunk.checksum {
            if !checksum.validate_checksum(&data) {
                return Err(MetalinkDownloadError::Other(anyhow!(
                    "Checksum Validation Failed"
                )));
            }
        }
        Ok(DownloadChunkResult {
            start: chunk.start,
            end: chunk.end,
            target_file: chunk.target_file.clone(),
            data,
        })
    }
}

async fn download_task(
    metalink_file: PathBuf,
    verify_chunk_checksums: bool,
    target_dir: PathBuf,
    user_agent: Option<String>,
    max_concurrent_tasks: u64,
    max_retries: u64,
    mut state_change_cmd_rx: tokio::sync::watch::Receiver<DownloadStateChangeCommand>,
    event_handler: Arc<Box<dyn EventHandler + Send + Sync>>,
) -> Result<()> {
    log::info!("Download Task start");
    let mut state = DownloadState::Created;
    let mut retries = 0;
    let mut chunks_todo = 0;

    let plan = Plan::new(metalink_file, &target_dir)?.minimize_plan()?;
    let client = make_http_client(
        user_agent
            .unwrap_or(concat!("lib-metalink-download/", env!("CARGO_PKG_VERSION")).to_owned()),
    )?;

    let (data_tx, mut data_rx) = tokio::sync::mpsc::unbounded_channel();
    let (error_tx, mut error_rx) = tokio::sync::mpsc::unbounded_channel();
    let (state_change_tx, state_change_rx) =
        tokio::sync::watch::channel(DownloadStateChangeCommand::Init);
    let (chunk_tx, chunk_rx) = async_channel::unbounded();

    // spawn workers
    let mut worker_set = tokio::task::JoinSet::new();
    for _ in 0..max_concurrent_tasks {
        let cloned_data_tx = data_tx.clone();
        let cloned_error_tx = error_tx.clone();
        let cloned_state_change_rx = state_change_rx.clone();
        let cloned_chunk_rx = chunk_rx.clone();
        let cloned_client = client.clone();
        worker_set.spawn(async move {
            let mut worker = DownloadWorker::new(
                cloned_state_change_rx,
                cloned_chunk_rx,
                cloned_data_tx,
                cloned_error_tx,
                cloned_client,
            );
            worker.spawn().await;
        });
    }

    event_handler.on_download_initialized(plan.total_size);

    // submit work
    for file in plan.files {
        if let Some(chunks) = file.chunks {
            chunks_todo += chunks.len();
            for chunk in chunks {
                let checksum = if !verify_chunk_checksums {
                    None
                } else {
                    chunk.checksum
                };

                chunk_tx
                    .send(DownloadChunkData {
                        start: chunk.start,
                        end: chunk.end,
                        url: file.url.clone(),
                        target_file: file.target_file.clone(),
                        checksum,
                    })
                    .await
                    .with_context(|| {
                        format!(
                            "Error while submitting chunk for file {:?}",
                            file.target_file
                        )
                    })?;
            }
        } else {
            let file_size = if let Some(file_size) = file.file_size {
                file_size
            } else {
                if let Some(file_size) = get_file_size(&client, file.url.clone())
                    .await
                    .with_context(|| format!("Unable to detect filesize of {}", file.url))?
                {
                    file_size
                } else {
                    return Err(MetalinkDownloadError::Other(anyhow!(
                        "Unable to detect filesize of {}",
                        file.url
                    )));
                }
            };

            let chunks =
                ChunkMetaData::calculate_ranges(file_size, DEFAULT_CHUNK_SIZE, &file.target_file);
            chunks_todo += chunks.len();
            for chunk in chunks {
                chunk_tx
                    .send(DownloadChunkData {
                        start: chunk.start,
                        end: chunk.end,
                        url: file.url.clone(),
                        target_file: file.target_file.clone(),
                        checksum: None,
                    })
                    .await
                    .with_context(|| {
                        format!("Error while submitting file {:?}", file.target_file)
                    })?;
            }
        }
    }

    log::info!("Total chunks: {}", chunks_todo);
    if chunks_todo == 0 {
        return Ok(());
    }

    let mut bytes_downloaded: u64 = 0;
    // process results and pause/resume/cancel requests
    loop {
        match state {
            DownloadState::Created => match state_change_cmd_rx.changed().await {
                Ok(()) => match *state_change_cmd_rx.borrow_and_update() {
                    DownloadStateChangeCommand::Run => {
                        state_change_tx
                            .send(DownloadStateChangeCommand::Run)
                            .with_context(|| "state change channel was closed")?;
                        state = DownloadState::Running;
                        event_handler.on_download_started();
                    }
                    DownloadStateChangeCommand::Cancel => {
                        state_change_tx
                            .send(DownloadStateChangeCommand::Cancel)
                            .with_context(|| "state change channel was closed")?;
                        state = DownloadState::Finished;
                        event_handler.on_download_cancelled();
                    }
                    _ => (),
                },
                // state change sender dropped cancel workers and get out
                Err(e) => {
                    state_change_tx
                        .send(DownloadStateChangeCommand::Cancel)
                        .with_context(|| "state change channel was closed")?;
                    state = DownloadState::Finished;
                    event_handler.on_download_failed(MetalinkDownloadError::Other(anyhow!(
                        "Download Failed: {}",
                        e
                    )));
                }
            },
            DownloadState::Running => {
                tokio::select! {
                    changed = state_change_cmd_rx.changed() => {
                        match changed {
                            Ok(()) => {
                                match *state_change_cmd_rx.borrow_and_update() {
                                    DownloadStateChangeCommand::Pause => {
                                        state_change_tx.send(DownloadStateChangeCommand::Pause).with_context(|| "state change channel was closed")?;
                                        state = DownloadState::Paused;
                                        event_handler.on_download_paused();
                                    }
                                    DownloadStateChangeCommand::Cancel => {
                                        state_change_tx.send(DownloadStateChangeCommand::Cancel).with_context(|| "state change channel was closed")?;
                                        state = DownloadState::Finished;
                                        event_handler.on_download_cancelled();
                                    }
                                    _ => ()
                                }
                            }
                            Err(e) => {
                                state_change_tx
                                    .send(DownloadStateChangeCommand::Cancel)
                                    .with_context(|| "state change channel was closed")?;
                                state = DownloadState::Finished;
                                event_handler.on_download_failed(MetalinkDownloadError::Other(anyhow!(
                                    "Download Failed: {}",
                                    e
                                )));
                            }
                        }
                    },
                    result = data_rx.recv() => {
                        match result {
                            Some(result) => {
                                write_chunk(&result).await?;
                                bytes_downloaded += result.data.len() as u64;
                                event_handler.on_download_progressed(bytes_downloaded, plan.total_size);
                                chunks_todo -= 1;
                                if chunks_todo == 0 {
                                    // Everything seems to be done. Finish
                                    state = DownloadState::Finished;
                                    event_handler.on_download_succeeded();
                                }
                            }
                            None => state = DownloadState::Finished,
                        }
                    }
                    error = error_rx.recv() => {
                        match error {
                            Some(chunk) => {
                                if retries > max_retries {
                                    log::error!("Max retries reached. Cancelling workers");
                                    state_change_tx
                                        .send(DownloadStateChangeCommand::Cancel)
                                        .with_context(|| "state change channel was closed")?;
                                    state = DownloadState::Finished;
                                    event_handler.on_download_failed(MetalinkDownloadError::Other(anyhow!(
                                        "Download Failed: Maximum number of retries exceeded",
                                    )));
                                } else {
                                    retries += 1;
                                    chunk_tx
                                        .send(chunk)
                                        .await
                                        .with_context(|| format!("Error while resubmitting chunk"))?;
                                }
                            }
                            None => state = DownloadState::Finished,
                        }
                    }
                }
            }
            DownloadState::Paused => match state_change_cmd_rx.changed().await {
                Ok(()) => match *state_change_cmd_rx.borrow_and_update() {
                    DownloadStateChangeCommand::Resume => {
                        state_change_tx
                            .send(DownloadStateChangeCommand::Resume)
                            .with_context(|| "state change channel was closed")?;
                        state = DownloadState::Running;
                        event_handler.on_download_resumed();
                    }
                    DownloadStateChangeCommand::Cancel => {
                        state_change_tx
                            .send(DownloadStateChangeCommand::Cancel)
                            .with_context(|| "state change channel was closed")?;
                        state = DownloadState::Finished;
                        event_handler.on_download_cancelled();
                    }
                    _ => (),
                },
                // state change sender dropped cancel workers and get out
                Err(e) => {
                    state_change_tx
                        .send(DownloadStateChangeCommand::Cancel)
                        .with_context(|| "state change channel was closed")?;
                    state = DownloadState::Finished;
                    event_handler.on_download_failed(MetalinkDownloadError::Other(anyhow!(
                        "Download Failed: {}",
                        e
                    )));
                }
            },
            DownloadState::Finished => {
                log::info!("Closing chunk channel");
                chunk_tx.close();
                state_change_tx
                    .send(DownloadStateChangeCommand::Cancel)
                    .with_context(|| "state change channel was closed")?;
                break;
            }
        }
    }

    Ok(())
}

async fn write_chunk(chunk_result: &DownloadChunkResult) -> Result<()> {
    tokio::fs::create_dir_all(chunk_result.target_file.parent().unwrap())
        .await
        .with_context(|| {
            format!(
                "Failed to create directory: {:?}",
                chunk_result.target_file.parent().unwrap()
            )
        })?;

    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(chunk_result.target_file.clone())
        .await
        .with_context(|| format!("Failed to open file {:?}", chunk_result.target_file))?;

    file.seek(std::io::SeekFrom::Start(chunk_result.start))
        .await
        .with_context(|| "Unable to seek position")?;

    file.write_all(&chunk_result.data)
        .await
        .with_context(|| "Failed to write data")?;

    file.sync_all()
        .await
        .with_context(|| "Failed to sync data to disk")?;

    Ok(())
}
