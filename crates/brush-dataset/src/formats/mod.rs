use crate::{
    brush_vfs::BrushVfs,
    splat_import::{load_splat_from_ply, SplatMessage},
    Dataset, LoadDataseConfig, WasmNotSend,
};
use brush_render::Backend;
use std::pin::Pin;
use tokio_stream::Stream;

pub mod colmap;
pub mod nerfstudio;

pub trait DynStream<Item>: Stream<Item = Item> + WasmNotSend {}
impl<Item, T: Stream<Item = Item> + WasmNotSend> DynStream<Item> for T {}
pub type DataStream<T> = Pin<Box<dyn DynStream<anyhow::Result<T>> + 'static>>;

pub async fn load_dataset<B: Backend>(
    mut vfs: BrushVfs,
    load_args: &LoadDataseConfig,
    device: &B::Device,
) -> anyhow::Result<(DataStream<SplatMessage<B>>, DataStream<Dataset>)> {
    let stream = nerfstudio::read_dataset(vfs.clone(), load_args, device).await;

    let stream = match stream {
        Ok(s) => Ok(s),
        Err(_) => colmap::load_dataset::<B>(vfs.clone(), load_args, device).await,
    };

    let stream = match stream {
        Ok(stream) => stream,
        Err(e) => anyhow::bail!(
            "Couldn't parse dataset as any format. Only some formats are supported. {e}"
        ),
    };

    // If there's an initial ply file, override the init stream with that.
    let path: Vec<_> = vfs
        .file_names()
        .filter(|x| x.extension().is_some_and(|ext| ext == "ply"))
        .collect();

    let init_stream = if path.len() == 1 {
        let main_path = path.first().expect("unreachable").to_path_buf();
        log::info!("Using ply {main_path:?} as initial point cloud.");

        let reader = vfs.open_path(&main_path).await?;
        Box::pin(load_splat_from_ply(
            reader,
            load_args.subsample_points,
            device.clone(),
        ))
    } else {
        stream.0
    };

    Ok((init_stream, stream.1))
}
