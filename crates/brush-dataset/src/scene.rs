use brush_render::{bounding_box::BoundingBox, camera::Camera};
use burn::{
    prelude::Backend,
    tensor::{Tensor, TensorData},
};
use glam::{Affine3A, Vec3, vec3};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ViewType {
    Train,
    Eval,
    Test,
}

#[derive(Debug, Clone)]
pub struct SceneView {
    pub path: String,
    pub camera: Camera,
    pub image: Arc<image::DynamicImage>,
    pub img_type: ViewImageType,
}

// Encapsulates a multi-view scene including cameras and the splats.
// Also provides methods for checkpointing the training process.
#[derive(Debug, Clone)]
pub struct Scene {
    pub views: Arc<Vec<SceneView>>,
}

fn camera_distance_penalty(cam_local_to_world: Affine3A, reference: Affine3A) -> f32 {
    let mut penalty = 0.0;
    for off_x in [-1.0, 0.0, 1.0] {
        for off_y in [-1.0, 0.0, 1.0] {
            let offset = vec3(off_x, off_y, 1.0);
            let cam_pos = cam_local_to_world.transform_point3(offset);
            let ref_pos = reference.transform_point3(offset);
            penalty += (cam_pos - ref_pos).length();
        }
    }
    penalty
}

fn find_two_smallest(v: Vec3) -> (f32, f32) {
    let mut arr = v.to_array();
    arr.sort_by(|a, b| a.partial_cmp(b).expect("NaN"));
    (arr[0], arr[1])
}

impl Scene {
    pub fn new(views: Vec<SceneView>) -> Self {
        Self {
            views: Arc::new(views),
        }
    }

    // Returns the extent of the cameras in the scene.
    pub fn bounds(&self) -> BoundingBox {
        self.adjusted_bounds(0.0, 0.0)
    }

    // Returns the extent of the cameras in the scene, taking into account
    // the near and far plane of the cameras.
    pub fn adjusted_bounds(&self, cam_near: f32, cam_far: f32) -> BoundingBox {
        let (min, max) = self.views.iter().fold(
            (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
            |(min, max), view| {
                let cam = &view.camera;
                let pos1 = cam.position + cam.rotation * Vec3::Z * cam_near;
                let pos2 = cam.position + cam.rotation * Vec3::Z * cam_far;
                (min.min(pos1).min(pos2), max.max(pos1).max(pos2))
            },
        );
        BoundingBox::from_min_max(min, max)
    }

    pub fn get_nearest_view(&self, reference: Affine3A) -> Option<usize> {
        self.views
            .iter()
            .enumerate() // This will give us (index, view) pairs
            .min_by(|(_, a), (_, b)| {
                let score_a = camera_distance_penalty(a.camera.local_to_world(), reference);
                let score_b = camera_distance_penalty(b.camera.local_to_world(), reference);
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(index, _)| index) // We return the index instead of the camera
    }

    pub fn estimate_extent(&self) -> Option<f32> {
        if self.views.len() < 5 {
            None
        } else {
            // TODO: This is really sensitive to outliers.
            let bounds = self.bounds();
            let smallest = find_two_smallest(bounds.extent * 2.0);
            Some(smallest.0.hypot(smallest.1))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewImageType {
    Alpha,
    Masked,
}

// Converts an image to a train sample. The tensor will be a floating point image with a [0, 1] image.
//
// This assume the input image has un-premultiplied alpha, whereas the output has pre-multiplied alpha.
pub fn view_to_sample<B: Backend>(view: &SceneView, device: &B::Device) -> Tensor<B, 3> {
    let image = &view.image;
    let (w, h) = (image.width(), image.height());

    let tensor_data = if image.color().has_alpha() {
        // Assume image has un-multiplied alpha and convert it to pre-multiplied.
        let mut rgba = image.to_rgba32f();
        if view.img_type == ViewImageType::Alpha {
            for pixel in rgba.pixels_mut() {
                let a = pixel[3];
                pixel[0] *= a;
                pixel[1] *= a;
                pixel[2] *= a;
            }
        }
        TensorData::new(rgba.into_vec(), [h as usize, w as usize, 4])
    } else {
        TensorData::new(image.to_rgb32f().into_vec(), [h as usize, w as usize, 3])
    };

    Tensor::from_data(tensor_data, device)
}

#[derive(Clone, Debug)]
pub struct SceneBatch<B: Backend> {
    pub gt_image: Tensor<B, 3>,
    pub gt_view: SceneView,
}
