use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

/// Generates a 16x16 noise map for a given chunk using fractal noise with multiple octaves
/// to avoid repetitive patterns.
///
/// # Arguments
/// - `seed`: A number influencing the noise generation (e.g., 42).
/// - `chunk_x`: The X coordinate of the chunk.
/// - `chunk_z`: The Z coordinate of the chunk.
/// - `scale`: The noise scale (smaller = more detailed noise).
///
/// # Returns
/// - A **16x16 noise map** as `[[f64; 16]; 16]`, normalized between -64 and 324.
pub fn generate_normalized_noise_map(
    seed: u32,
    chunk_x: i32,
    chunk_z: i32,
    scale: f64,
) -> [[f64; 16]; 16] {
    // Spécifier explicitement que Fbm utilise Perlin comme bruit de base
    let fbm = Fbm::<Perlin>::new(seed).set_octaves(15);
    let mut noise_map = [[0.0; 16]; 16];

    // Define normalization range
    let min_range = -64.0;
    let max_range = 320.0;

    for x in 0..16 {
        for z in 0..16 {
            // Convert chunk-local coordinates to global world coordinates
            let world_x = (chunk_x * 16 + x as i32) as f64 * scale;
            let world_z = (chunk_z * 16 + z as i32) as f64 * scale;

            // Generate fractal noise value (with multiple octaves)
            let noise_value = fbm.get([world_x, world_z]);

            // Normalize noise from [-1,1] to [-64,324]
            let normalized_noise = (noise_value + 1.0) / 2.0 * (max_range - min_range) + min_range;

            // Store the normalized noise value
            noise_map[x][z] = normalized_noise;
        }
    }
    noise_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_normalized_noise_map() {
        let noise_map = generate_normalized_noise_map(456, 0, 0, 0.1);
        assert_eq!(noise_map.len(), 16);
        for row in noise_map.iter() {
            assert_eq!(row.len(), 16);
        }
    }
}
