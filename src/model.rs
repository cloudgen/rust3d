//! Outline images to a glTF model and an HTML viewer.
//! requirement-domain-rust3d — one folder in, model.glb and viewer.html.
//! The image decode runs only when this folder has a supported image.

use image::GrayImage;
use serde_json::json;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub const SUPPORTED_INPUT_EXTENSIONS: &[&str] = &[".webp", ".png", ".jpg", ".jpeg"];
pub const OUTPUT_DIR_NAME: &str = "model";
pub const GLB_NAME: &str = "model.glb";
pub const VIEWER_NAME: &str = "viewer.html";
pub const DEFAULT_GRID: i64 = 96;
pub const MIN_GRID: i64 = 16;
pub const MAX_GRID: i64 = 160;

static DECODE_HITS: AtomicUsize = AtomicUsize::new(0);
static DECODE_HOOK: Mutex<Option<fn()>> = Mutex::new(None);

pub fn reset_decode_watch() {
    DECODE_HITS.store(0, Ordering::SeqCst);
    *DECODE_HOOK.lock().expect("decode hook") = None;
}

pub fn set_decode_hook(hook: fn()) {
    *DECODE_HOOK.lock().expect("decode hook") = Some(hook);
}

pub fn decode_hits() -> usize {
    DECODE_HITS.load(Ordering::SeqCst)
}

pub fn list_subfolders(folder: &Path) -> Vec<String> {
    let entries = match fs::read_dir(folder) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };
    let mut names = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.is_empty() || name.starts_with('.') {
            continue;
        }
        if entry.path().is_dir() {
            names.push(name);
        }
    }
    names.sort_by_key(|item| item.to_lowercase());
    names
}

pub fn list_images(folder: &Path) -> Vec<PathBuf> {
    let entries = match fs::read_dir(folder) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };
    let mut files = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|item| item.to_str()).unwrap_or("");
        let lower = name.to_lowercase();
        if SUPPORTED_INPUT_EXTENSIONS.iter().any(|ext| lower.ends_with(ext)) {
            files.push(path);
        }
    }
    files.sort_by_key(|item| {
        item.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_lowercase()
    });
    files
}

pub fn folder_for_pick(name: &str, place: Option<&Path>) -> Result<PathBuf, String> {
    let root = match place {
        Some(path) => path.to_path_buf(),
        None => std::env::current_dir().map_err(|err| err.to_string())?,
    };
    let root = root.canonicalize().unwrap_or(root);
    if name == "." {
        return Ok(root);
    }
    if name.is_empty() || name == ".." || name.contains('/') || name.contains('\\') || name != Path::new(name).file_name().and_then(|item| item.to_str()).unwrap_or("") {
        return Err(name.to_string());
    }
    Ok(root.join(name))
}

pub fn output_dir_for(folder: &Path) -> PathBuf {
    folder.join(OUTPUT_DIR_NAME)
}

/// The one sentence shown before a slow process starts.
/// A function returns it. It is not a module-level message constant.
pub fn selected_choice_line(choice: &str, process: &str) -> String {
    format!("{choice} has been selected. {process} takes time to finish.")
}

#[derive(Clone, Debug)]
struct ViewRow {
    file: String,
    azimuth: f64,
    elevation: f64,
}

fn read_views(path: &Path) -> Result<Vec<ViewRow>, String> {
    let raw = fs::read_to_string(path).map_err(|err| err.to_string())?;
    let parsed: serde_json::Value = serde_json::from_str(&raw).map_err(|err| err.to_string())?;
    let rows = if parsed.is_array() {
        parsed.as_array().cloned().unwrap_or_default()
    } else {
        parsed
            .get("views")
            .and_then(|item| item.as_array())
            .cloned()
            .ok_or_else(|| "views JSON needs a non-empty list".to_string())?
    };
    if rows.is_empty() {
        return Err("views JSON needs a non-empty list".to_string());
    }
    let mut parsed_rows = Vec::new();
    for row in rows {
        let object = row.as_object().ok_or_else(|| "each view must be an object".to_string())?;
        let name = object
            .get("file")
            .and_then(|item| item.as_str())
            .ok_or_else(|| "view file must be a basename".to_string())?;
        if name.is_empty()
            || name != Path::new(name).file_name().and_then(|item| item.to_str()).unwrap_or("")
            || name.contains('/')
            || name.contains('\\')
            || name == "."
            || name == ".."
        {
            return Err("view file must be a basename".to_string());
        }
        let azimuth = object
            .get("azimuth")
            .and_then(|item| item.as_f64())
            .ok_or_else(|| "view file must be a basename".to_string())?;
        let elevation = object
            .get("elevation")
            .and_then(|item| item.as_f64())
            .ok_or_else(|| "view file must be a basename".to_string())?;
        parsed_rows.push(ViewRow {
            file: name.to_string(),
            azimuth,
            elevation,
        });
    }
    Ok(parsed_rows)
}

fn equal_views(images: &[PathBuf]) -> Vec<ViewRow> {
    let count = images.len();
    if count == 0 {
        return Vec::new();
    }
    let step = 360.0 / count as f64;
    images
        .iter()
        .enumerate()
        .map(|(index, image)| ViewRow {
            file: image.file_name().and_then(|name| name.to_str()).unwrap_or("").to_string(),
            azimuth: step * index as f64,
            elevation: 0.0,
        })
        .collect()
}

fn ellipse_offsets() -> Vec<(i32, i32)> {
    let pattern = ["00100", "01110", "11111", "01110", "00100"];
    let mut offsets = Vec::new();
    for (y, row) in pattern.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '1' {
                offsets.push((x as i32 - 2, y as i32 - 2));
            }
        }
    }
    offsets
}

fn dilate(src: &[u8], width: usize, height: usize, kernel: &[(i32, i32)]) -> Vec<u8> {
    let mut dst = vec![0u8; width * height];
    for y in 0..height {
        for x in 0..width {
            let mut on = false;
            for (dx, dy) in kernel {
                let xx = x as i32 + dx;
                let yy = y as i32 + dy;
                if xx >= 0 && yy >= 0 && (xx as usize) < width && (yy as usize) < height {
                    if src[yy as usize * width + xx as usize] > 0 {
                        on = true;
                        break;
                    }
                }
            }
            if on {
                dst[y * width + x] = 255;
            }
        }
    }
    dst
}

fn erode(src: &[u8], width: usize, height: usize, kernel: &[(i32, i32)]) -> Vec<u8> {
    let mut dst = vec![0u8; width * height];
    for y in 0..height {
        for x in 0..width {
            let mut on = true;
            for (dx, dy) in kernel {
                let xx = x as i32 + dx;
                let yy = y as i32 + dy;
                if xx < 0 || yy < 0 || (xx as usize) >= width || (yy as usize) >= height {
                    on = false;
                    break;
                }
                if src[yy as usize * width + xx as usize] == 0 {
                    on = false;
                    break;
                }
            }
            if on {
                dst[y * width + x] = 255;
            }
        }
    }
    dst
}

fn outline_to_solid_mask(image_path: &Path) -> Result<(Vec<u8>, usize, usize), String> {
    let opened = image::ImageReader::open(image_path)
        .map_err(|_| "could not read image".to_string())?
        .with_guessed_format()
        .map_err(|_| "could not read image".to_string())?
        .decode()
        .map_err(|_| "could not read image".to_string())?;
    let gray: GrayImage = opened.into_luma8();
    let width = gray.width() as usize;
    let height = gray.height() as usize;
    if width == 0 || height == 0 {
        return Err("could not read image".to_string());
    }
    let mut thresh = vec![0u8; width * height];
    for y in 0..height {
        for x in 0..width {
            if gray.get_pixel(x as u32, y as u32).0[0] > 20 {
                thresh[y * width + x] = 255;
            }
        }
    }
    let kernel = ellipse_offsets();
    let closed = erode(&dilate(&thresh, width, height, &kernel), width, height, &kernel);
    let mut seen = vec![false; width * height];
    let mut stack = Vec::new();
    for x in 0..width {
        push_bg(&closed, width, &mut seen, &mut stack, x, 0);
        if height > 1 {
            push_bg(&closed, width, &mut seen, &mut stack, x, height - 1);
        }
    }
    for y in 0..height {
        push_bg(&closed, width, &mut seen, &mut stack, 0, y);
        if width > 1 {
            push_bg(&closed, width, &mut seen, &mut stack, width - 1, y);
        }
    }
    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let xx = x as i32 + dx;
            let yy = y as i32 + dy;
            if xx < 0 || yy < 0 || xx as usize >= width || yy as usize >= height {
                continue;
            }
            push_bg(&closed, width, &mut seen, &mut stack, xx as usize, yy as usize);
        }
    }
    let mut mask = vec![0u8; width * height];
    for index in 0..mask.len() {
        if !seen[index] {
            mask[index] = 255;
        }
    }
    Ok((mask, width, height))
}

fn push_bg(closed: &[u8], width: usize, seen: &mut [bool], stack: &mut Vec<(usize, usize)>, x: usize, y: usize) {
    let index = y * width + x;
    if seen[index] || closed[index] > 0 {
        return;
    }
    seen[index] = true;
    stack.push((x, y));
}

fn camera_axes(azimuth_deg: f64, elevation_deg: f64) -> ([f64; 3], [f64; 3]) {
    let azimuth = azimuth_deg.to_radians();
    let elevation = elevation_deg.to_radians();
    let view_dir = [
        azimuth.sin() * elevation.cos(),
        elevation.sin(),
        azimuth.cos() * elevation.cos(),
    ];
    let forward = [-view_dir[0], -view_dir[1], -view_dir[2]];
    let mut world_up = [0.0, 1.0, 0.0];
    if forward[0] * world_up[0] + forward[1] * world_up[1] + forward[2] * world_up[2] > 0.99
        || forward[0] * world_up[0] + forward[1] * world_up[1] + forward[2] * world_up[2] < -0.99
    {
        world_up = [0.0, 0.0, 1.0];
    }
    let mut right = cross(forward, world_up);
    normalize(&mut right);
    let mut up = cross(right, forward);
    normalize(&mut up);
    (right, up)
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn normalize(vector: &mut [f64; 3]) {
    let length = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    if length > 0.0 {
        vector[0] /= length;
        vector[1] /= length;
        vector[2] /= length;
    }
}

fn reconstruct_visual_hull(masks: &[(Vec<u8>, usize, usize, f64, f64)], grid: usize) -> (Vec<bool>, Vec<usize>) {
    let mut voxels = vec![true; grid * grid * grid];
    let mut axis = Vec::with_capacity(grid);
    if grid == 1 {
        axis.push(0.0);
    } else {
        for index in 0..grid {
            axis.push(-1.0 + 2.0 * index as f64 / (grid as f64 - 1.0));
        }
    }
    let mut counts = Vec::new();
    for (mask, width, height, azimuth, elevation) in masks {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut any = false;
        for y in 0..*height {
            for x in 0..*width {
                if mask[y * width + x] > 0 {
                    any = true;
                    min_x = min_x.min(x as i32);
                    max_x = max_x.max(x as i32);
                    min_y = min_y.min(y as i32);
                    max_y = max_y.max(y as i32);
                }
            }
        }
        if !any {
            voxels.fill(false);
            counts.push(0);
            continue;
        }
        let center_x = (min_x as f64 + max_x as f64) / 2.0;
        let center_y = (min_y as f64 + max_y as f64) / 2.0;
        let half = ((max_x - min_x) as f64 / 2.0)
            .max((max_y - min_y) as f64 / 2.0)
            .max(1.0);
        let (right, up) = camera_axes(*azimuth, *elevation);
        let mut keep = vec![false; grid * grid * grid];
        for ix in 0..grid {
            for iy in 0..grid {
                for iz in 0..grid {
                    let point = [axis[ix], axis[iy], axis[iz]];
                    let horizontal = point[0] * right[0] + point[1] * right[1] + point[2] * right[2];
                    let vertical = point[0] * up[0] + point[1] * up[1] + point[2] * up[2];
                    let column = (center_x + horizontal * half).round() as i32;
                    let row = (center_y - vertical * half).round() as i32;
                    let index = (ix * grid + iy) * grid + iz;
                    if column >= 0 && (column as usize) < *width && row >= 0 && (row as usize) < *height {
                        keep[index] = mask[row as usize * width + column as usize] > 0;
                    }
                }
            }
        }
        for (voxel, stay) in voxels.iter_mut().zip(keep) {
            *voxel = *voxel && stay;
        }
        counts.push(voxels.iter().filter(|item| **item).count());
    }
    (voxels, counts)
}

fn mesh_from_voxels(voxels: &[bool], grid: usize) -> Option<(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>)> {
    if !voxels.iter().any(|item| *item) {
        return None;
    }
    let step = if grid > 1 { 2.0 / (grid as f32 - 1.0) } else { 1.0 };
    let origin = |index: usize| -1.0 + step * index as f32 - step * 0.5;
    let at = |x: i32, y: i32, z: i32| -> bool {
        if x < 0 || y < 0 || z < 0 || x as usize >= grid || y as usize >= grid || z as usize >= grid {
            return false;
        }
        voxels[((x as usize) * grid + y as usize) * grid + z as usize]
    };
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let faces: [([i32; 3], [f32; 3]); 6] = [
        ([1, 0, 0], [1.0, 0.0, 0.0]),
        ([-1, 0, 0], [-1.0, 0.0, 0.0]),
        ([0, 1, 0], [0.0, 1.0, 0.0]),
        ([0, -1, 0], [0.0, -1.0, 0.0]),
        ([0, 0, 1], [0.0, 0.0, 1.0]),
        ([0, 0, -1], [0.0, 0.0, -1.0]),
    ];
    for x in 0..grid as i32 {
        for y in 0..grid as i32 {
            for z in 0..grid as i32 {
                if !at(x, y, z) {
                    continue;
                }
                let x0 = origin(x as usize);
                let y0 = origin(y as usize);
                let z0 = origin(z as usize);
                let x1 = x0 + step;
                let y1 = y0 + step;
                let z1 = z0 + step;
                for (delta, normal) in faces {
                    if at(x + delta[0], y + delta[1], z + delta[2]) {
                        continue;
                    }
                    let corners = face_corners(delta, x0, y0, z0, x1, y1, z1);
                    push_face(&mut positions, &mut normals, &mut indices, corners, normal);
                }
            }
        }
    }
    if positions.is_empty() {
        return None;
    }
    let count = positions.len() as f32;
    let mut mean = [0.0f32; 3];
    for point in &positions {
        mean[0] += point[0];
        mean[1] += point[1];
        mean[2] += point[2];
    }
    mean[0] /= count;
    mean[1] /= count;
    mean[2] /= count;
    let mut extent = 0.0f32;
    for point in &mut positions {
        point[0] -= mean[0];
        point[1] -= mean[1];
        point[2] -= mean[2];
        extent = extent.max((point[0] * point[0] + point[1] * point[1] + point[2] * point[2]).sqrt());
    }
    if extent > 0.0 {
        for point in &mut positions {
            point[0] /= extent;
            point[1] /= extent;
            point[2] /= extent;
        }
    }
    Some((positions, normals, indices))
}

fn face_corners(delta: [i32; 3], x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> [[f32; 3]; 4] {
    if delta[0] == 1 {
        return [[x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1]];
    }
    if delta[0] == -1 {
        return [[x0, y0, z1], [x0, y1, z1], [x0, y1, z0], [x0, y0, z0]];
    }
    if delta[1] == 1 {
        return [[x0, y1, z1], [x1, y1, z1], [x1, y1, z0], [x0, y1, z0]];
    }
    if delta[1] == -1 {
        return [[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]];
    }
    if delta[2] == 1 {
        return [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]];
    }
    [[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]]
}

fn push_face(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    corners: [[f32; 3]; 4],
    normal: [f32; 3],
) {
    for tri in [[0, 1, 2], [0, 2, 3]] {
        let base = positions.len() as u32;
        for corner in tri {
            positions.push(corners[corner]);
            normals.push(normal);
        }
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);
    }
}

fn pad4(mut blob: Vec<u8>, pad: u8) -> Vec<u8> {
    let extra = (4 - (blob.len() % 4)) % 4;
    blob.extend(std::iter::repeat(pad).take(extra));
    blob
}

pub fn write_glb(path: &Path, positions: &[[f32; 3]], normals: &[[f32; 3]], indices: &[u32]) -> io::Result<(usize, usize)> {
    let mut pos_bytes = Vec::with_capacity(positions.len() * 12);
    let mut nrm_bytes = Vec::with_capacity(normals.len() * 12);
    let mut idx_bytes = Vec::with_capacity(indices.len() * 4);
    let mut pos_min = [f32::MAX; 3];
    let mut pos_max = [f32::MIN; 3];
    for point in positions {
        for axis in 0..3 {
            pos_min[axis] = pos_min[axis].min(point[axis]);
            pos_max[axis] = pos_max[axis].max(point[axis]);
            pos_bytes.extend_from_slice(&point[axis].to_le_bytes());
        }
    }
    for normal in normals {
        for axis in 0..3 {
            nrm_bytes.extend_from_slice(&normal[axis].to_le_bytes());
        }
    }
    for index in indices {
        idx_bytes.extend_from_slice(&index.to_le_bytes());
    }
    let blob = {
        let mut joined = pos_bytes.clone();
        joined.extend_from_slice(&nrm_bytes);
        joined.extend_from_slice(&idx_bytes);
        joined
    };
    let document = json!({
        "asset": {"version": "2.0", "generator": "rust3d"},
        "scene": 0,
        "scenes": [{"nodes": [0]}],
        "nodes": [{"mesh": 0}],
        "materials": [{
            "doubleSided": true,
            "pbrMetallicRoughness": {
                "baseColorFactor": [0.72, 0.76, 0.8, 1.0],
                "metallicFactor": 0.05,
                "roughnessFactor": 0.55
            }
        }],
        "meshes": [{
            "primitives": [{
                "attributes": {"POSITION": 0, "NORMAL": 1},
                "indices": 2,
                "material": 0,
                "mode": 4
            }]
        }],
        "buffers": [{"byteLength": blob.len()}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": 0, "byteLength": pos_bytes.len(), "target": 34962},
            {"buffer": 0, "byteOffset": pos_bytes.len(), "byteLength": nrm_bytes.len(), "target": 34962},
            {"buffer": 0, "byteOffset": pos_bytes.len() + nrm_bytes.len(), "byteLength": idx_bytes.len(), "target": 34963}
        ],
        "accessors": [
            {"bufferView": 0, "componentType": 5126, "count": positions.len(), "type": "VEC3", "min": pos_min, "max": pos_max},
            {"bufferView": 1, "componentType": 5126, "count": normals.len(), "type": "VEC3"},
            {"bufferView": 2, "componentType": 5125, "count": indices.len(), "type": "SCALAR"}
        ]
    });
    let json_bytes = pad4(serde_json::to_vec(&document).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?, b' ');
    let bin_bytes = pad4(blob, 0);
    let total = 12 + 8 + json_bytes.len() + 8 + bin_bytes.len();
    let mut file = Vec::with_capacity(total);
    file.extend_from_slice(b"glTF");
    file.extend_from_slice(&2u32.to_le_bytes());
    file.extend_from_slice(&(total as u32).to_le_bytes());
    file.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    file.extend_from_slice(b"JSON");
    file.extend_from_slice(&json_bytes);
    file.extend_from_slice(&(bin_bytes.len() as u32).to_le_bytes());
    file.extend_from_slice(b"BIN\0");
    file.extend_from_slice(&bin_bytes);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, file)?;
    Ok((positions.len(), indices.len() / 3))
}

fn viewer_html(positions: &[[f32; 3]], normals: &[[f32; 3]], indices: &[u32]) -> String {
    let mut pos = Vec::new();
    let mut nrm = Vec::new();
    let mut idx = Vec::new();
    for point in positions {
        for axis in 0..3 {
            pos.extend_from_slice(&point[axis].to_le_bytes());
        }
    }
    for normal in normals {
        for axis in 0..3 {
            nrm.extend_from_slice(&normal[axis].to_le_bytes());
        }
    }
    for index in indices {
        idx.extend_from_slice(&index.to_le_bytes());
    }
    include_str!("viewer_page.html")
        .replace("__POS__", &b64(&pos))
        .replace("__NRM__", &b64(&nrm))
        .replace("__IDX__", &b64(&idx))
}

fn b64(bytes: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut index = 0;
    while index + 3 <= bytes.len() {
        let n = ((bytes[index] as u32) << 16) | ((bytes[index + 1] as u32) << 8) | bytes[index + 2] as u32;
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        out.push(TABLE[((n >> 6) & 63) as usize] as char);
        out.push(TABLE[(n & 63) as usize] as char);
        index += 3;
    }
    if index < bytes.len() {
        let remain = bytes.len() - index;
        let mut n = (bytes[index] as u32) << 16;
        if remain == 2 {
            n |= (bytes[index + 1] as u32) << 8;
        }
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        if remain == 2 {
            out.push(TABLE[((n >> 6) & 63) as usize] as char);
            out.push('=');
        } else {
            out.push('=');
            out.push('=');
        }
    }
    out
}

fn done(code: i32, mut lines: Vec<String>, announced: &[String]) -> (i32, Vec<String>) {
    if !announced.is_empty() && (lines.is_empty() || lines[0] != announced[0]) {
        lines.insert(0, announced[0].clone());
    }
    (code, lines)
}

pub fn convert_folder<F: FnMut(&str)>(
    folder: &str,
    output_dir: Option<&str>,
    views_path: Option<&str>,
    grid: Option<i64>,
    choice: &str,
    mut on_notice: Option<&mut F>,
) -> (i32, Vec<String>) {
    let announced: Vec<String> = Vec::new();
    let place = PathBuf::from(folder);
    if !place.is_dir() {
        return done(
            1,
            vec![
                format!("ERROR: {} is not a folder.", place.display()),
                "   Next: rust3d model".to_string(),
            ],
            &announced,
        );
    }
    let place = place.canonicalize().unwrap_or(place);
    let grid_res = grid.unwrap_or(DEFAULT_GRID);
    if !(MIN_GRID..=MAX_GRID).contains(&grid_res) {
        return done(
            1,
            vec![
                format!("ERROR: grid must be an integer from {MIN_GRID} to {MAX_GRID}."),
                format!("   Next: rust3d model --grid {DEFAULT_GRID}"),
            ],
            &announced,
        );
    }
    let images = list_images(&place);
    let view_file = if let Some(path) = views_path {
        Some(PathBuf::from(path))
    } else if place.join("views.json").is_file() {
        Some(place.join("views.json"))
    } else {
        None
    };
    let views = match view_file {
        Some(path) => {
            if !path.is_file() {
                return done(
                    1,
                    vec![
                        "ERROR: Could not read views: views file is missing".to_string(),
                        "   Next: rust3d model --views views.json".to_string(),
                    ],
                    &announced,
                );
            }
            match read_views(&path) {
                Ok(rows) => rows,
                Err(err) => {
                    return done(
                        1,
                        vec![
                            format!("ERROR: Could not read views: {err}"),
                            "   Next: rust3d model --views views.json".to_string(),
                        ],
                        &announced,
                    );
                }
            }
        }
        None => equal_views(&images),
    };
    if views.is_empty() {
        return done(
            0,
            vec![format!("No supported images found in {}", place.display())],
            &announced,
        );
    }
    let by_name: Vec<(String, PathBuf)> = images
        .iter()
        .map(|image| {
            (
                image.file_name().and_then(|name| name.to_str()).unwrap_or("").to_string(),
                image.clone(),
            )
        })
        .collect();
    let missing: Vec<&str> = views
        .iter()
        .filter(|row| !by_name.iter().any(|(name, _)| name == &row.file))
        .map(|row| row.file.as_str())
        .collect();
    if !missing.is_empty() && missing.len() == views.len() {
        return done(
            1,
            vec![
                format!("ERROR: None of the named outlines are in {}", place.display()),
                "   Next: rust3d model".to_string(),
            ],
            &announced,
        );
    }
    let dest = if let Some(path) = output_dir {
        PathBuf::from(path)
    } else {
        output_dir_for(&place)
    };
    if let Err(err) = fs::create_dir_all(&dest) {
        return done(
            1,
            vec![
                format!("ERROR: Could not create the output directory: {err}"),
                "   Next: rust3d model".to_string(),
            ],
            &announced,
        );
    }
    let sentence = selected_choice_line(choice, "Building the 3D model");
    let announced = vec![sentence.clone()];
    if let Some(notice) = on_notice.as_mut() {
        notice(&sentence);
    }
    if let Some(hook) = *DECODE_HOOK.lock().expect("decode hook") {
        hook();
    }
    DECODE_HITS.fetch_add(1, Ordering::SeqCst);
    let mut lines = vec![sentence];
    let mut failed = false;
    let mut masks = Vec::new();
    for row in &views {
        let image = by_name.iter().find(|(name, _)| name == &row.file);
        let Some((_, image)) = image else {
            failed = true;
            lines.push(format!("ERROR: Missing outline {}", row.file));
            continue;
        };
        let (mask, width, height) = match outline_to_solid_mask(image) {
            Ok(parts) => parts,
            Err(err) => {
                failed = true;
                lines.push(format!("ERROR: Failed on {}: {err}", row.file));
                continue;
            }
        };
        let foreground = mask.iter().filter(|pixel| **pixel > 0).count();
        if foreground < 50 {
            failed = true;
            lines.push(format!("ERROR: No closed outline in {}", row.file));
            continue;
        }
        lines.push(format!(
            "Mask {}: {foreground} foreground pixels, azimuth {}, elevation {}",
            row.file, row.azimuth, row.elevation
        ));
        masks.push((mask, width, height, row.azimuth, row.elevation, row.file.clone(), foreground));
    }
    if masks.is_empty() {
        lines.push("ERROR: No outline could be filled.".to_string());
        lines.push("   Next: rust3d model".to_string());
        return done(1, lines, &announced);
    }
    let hull_input: Vec<(Vec<u8>, usize, usize, f64, f64)> = masks
        .iter()
        .map(|(mask, width, height, azimuth, elevation, _, _)| {
            (mask.clone(), *width, *height, *azimuth, *elevation)
        })
        .collect();
    let (voxels, counts) = reconstruct_visual_hull(&hull_input, grid_res as usize);
    for (index, count) in counts.iter().enumerate() {
        lines.push(format!("Carved {}: {count} voxels remain.", masks[index].5));
    }
    let mesh = mesh_from_voxels(&voxels, grid_res as usize);
    let Some((positions, normals, indices)) = mesh else {
        lines.push("ERROR: The visual hull is empty. Check overlap and the view angles.".to_string());
        lines.push("   Next: rust3d model --views views.json".to_string());
        return done(1, lines, &announced);
    };
    let glb_path = dest.join(GLB_NAME);
    let html_path = dest.join(VIEWER_NAME);
    match write_glb(&glb_path, &positions, &normals, &indices) {
        Ok((vertex_count, triangle_count)) => {
            if let Err(err) = fs::write(&html_path, viewer_html(&positions, &normals, &indices)) {
                lines.push(format!("ERROR: Could not write the model: {err}"));
                lines.push("   Next: rust3d model".to_string());
                return done(1, lines, &announced);
            }
            lines.push(format!(
                "Saved model -> {} ({vertex_count} vertices, {triangle_count} triangles)",
                glb_path.file_name().and_then(|name| name.to_str()).unwrap_or(GLB_NAME)
            ));
            lines.push(format!(
                "Saved viewer -> {}",
                html_path.file_name().and_then(|name| name.to_str()).unwrap_or(VIEWER_NAME)
            ));
            lines.push(format!("Model saved in: {}", dest.display()));
            done(if failed { 1 } else { 0 }, lines, &announced)
        }
        Err(err) => {
            lines.push(format!("ERROR: Could not write the model: {err}"));
            lines.push("   Next: rust3d model".to_string());
            done(1, lines, &announced)
        }
    }
}

pub fn convert_main(argv: &[String]) -> i32 {
    let mut folder: Option<String> = None;
    let mut input_dir: Option<String> = None;
    let mut output: Option<String> = None;
    let mut views: Option<String> = None;
    let mut grid: Option<i64> = None;
    let mut index = 0;
    while index < argv.len() {
        let token = &argv[index];
        if token == "--input-dir" || token == "-i" {
            index += 1;
            input_dir = argv.get(index).cloned();
        } else if token == "--output" || token == "--output-dir" || token == "-o" {
            index += 1;
            output = argv.get(index).cloned();
        } else if token == "--views" {
            index += 1;
            views = argv.get(index).cloned();
        } else if token == "--grid" {
            index += 1;
            grid = argv.get(index).and_then(|item| item.parse().ok());
            if grid.is_none() {
                grid = Some(-1);
            }
        } else if token.starts_with('-') {
            eprintln!("ERROR: Unknown argument {token}.");
            eprintln!("   Next: ./convert --input-dir photos");
            return 1;
        } else if folder.is_none() {
            folder = Some(token.clone());
        } else {
            eprintln!("ERROR: Pass the folder once.");
            eprintln!("   Next: ./convert --input-dir photos");
            return 1;
        }
        index += 1;
    }
    if folder.is_some() && input_dir.is_some() {
        eprintln!("ERROR: Pass the folder once.");
        eprintln!("   Next: ./convert --input-dir photos");
        return 1;
    }
    let chosen = folder.or(input_dir).unwrap_or_else(|| ".".to_string());
    let mut spoken = Vec::new();
    let mut notice = |line: &str| {
        println!("{line}");
        let _ = io::stdout().flush();
        spoken.push(line.to_string());
    };
    let (code, lines) = convert_folder(
        &chosen,
        output.as_deref(),
        views.as_deref(),
        grid,
        "convert",
        Some(&mut notice),
    );
    let rest = &lines[spoken.len().min(lines.len())..];
    if !rest.is_empty() {
        println!("{}", rest.join("\n"));
    }
    code
}
