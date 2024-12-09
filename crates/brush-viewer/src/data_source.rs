use std::pin::Pin;

use anyhow::Context;
use async_fn_stream::try_fn_stream;

use ::tokio::io::AsyncRead;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tokio_util::{bytes::Bytes, io::StreamReader};
use tokio_with_wasm::alias as tokio;

#[derive(Debug)]
pub enum DataSource {
    PickFile,
    PickDirectory,
    Url(String),
}

type DataRead = Pin<Box<dyn AsyncRead + Send>>;

impl DataSource {
    pub fn into_reader(self) -> anyhow::Result<impl AsyncRead + Send> {
        let (send, rec) = ::tokio::sync::mpsc::channel(16);

        // Spawn the data reading.
        tokio::spawn(async move {
            let stream = try_fn_stream(|emitter| async move {
                match self {
                    DataSource::PickFile => {
                        let picked = rrfd::pick_file()
                            .await
                            .map_err(|_| std::io::ErrorKind::NotFound)?;
                        let data = picked.read().await;
                        emitter.emit(Bytes::from_owner(data)).await;
                    }
                    DataSource::PickDirectory => {
                        let picked = rrfd::pick_directory()
                            .await
                            .map_err(|_| std::io::ErrorKind::NotFound)?;
                        let data = picked;
                        let mut bytes = "BRUSH_PATH".as_bytes().to_vec();
                        let path_bytes = data
                            .to_str()
                            .context("invalid path")
                            .map_err(|_| std::io::ErrorKind::InvalidData)?
                            .as_bytes();
                        bytes.extend(path_bytes);
                        emitter.emit(Bytes::from_owner(bytes)).await;
                    }
                    DataSource::Url(url) => {
                        let mut url = url.to_owned();
                        if !url.starts_with("http://") && !url.starts_with("https://") {
                            url = format!("https://{}", url);
                        }
                        let mut response = reqwest::get(url)
                            .await
                            .map_err(|_| std::io::ErrorKind::InvalidInput)?
                            .bytes_stream();

                        while let Some(bytes) = response.next().await {
                            let bytes = bytes.map_err(|_| std::io::ErrorKind::ConnectionAborted)?;
                            emitter.emit(bytes).await;
                        }
                    }
                };
                anyhow::Result::<(), std::io::Error>::Ok(())
            });

            let mut stream = std::pin::pin!(stream);

            while let Some(data) = stream.next().await {
                if send.send(data).await.is_err() {
                    break;
                }
            }
        });

        let reader = StreamReader::new(ReceiverStream::new(rec));
        Ok(reader)
    }
}
