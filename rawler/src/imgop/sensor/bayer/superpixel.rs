// SPDX-License-Identifier: LGPL-2.1
// Copyright 2021 Daniel Vogelbacher <daniel@chaospixel.com>

use crate::imgop::{clip01, sensor::bayer::BayerPattern, Dim2};
use rayon::prelude::*;

/// Debayer image by using superpixel method.
/// Each output pixel RGB tuple is generated by 4 pixels from input.
/// The result image is 1/4 of size.
///
/// Before debayer, WB coefficents are applied. If you don't won't WB correction,
/// just supply 1.0 as factor.
pub fn debayer_superpixel(
  pixels: &Vec<u16>,
  pattern: BayerPattern,
  dim: Dim2,
  black_level: &[f32; 4],
  white_level: &[f32; 4],
  wb_coeff: &[f32; 4],
) -> (Vec<[f32; 3]>, usize, usize) {
  let scale_f = |p: [u16; 4]| -> [f32; 4] {
    [
      ((p[0] as f32 - black_level[0]) / (white_level[0] - black_level[0])) * wb_coeff[0],
      ((p[1] as f32 - black_level[1]) / (white_level[1] - black_level[1])) * wb_coeff[1],
      ((p[2] as f32 - black_level[2]) / (white_level[2] - black_level[2])) * wb_coeff[2],
      ((p[3] as f32 - black_level[3]) / (white_level[3] - black_level[3])) * wb_coeff[3],
    ]
  };
  let rgb = match pattern {
    BayerPattern::RGGB => pixels
      .par_chunks_exact(dim.w * 2)
      .map(|s| {
        let (r1, r2) = s.split_at(dim.w);
        r1.chunks_exact(2)
          .zip(r2.chunks_exact(2))
          .map(|(a, b)| {
            let scaled = scale_f([a[0], a[1], b[0], b[1]]);
            [clip01(scaled[0]), clip01((scaled[1] + scaled[2]) / 2.0), clip01(scaled[3])]
          })
          .collect::<Vec<_>>()
      })
      .flatten()
      .collect(),
    BayerPattern::BGGR => pixels
      .par_chunks_exact(dim.w * 2)
      .map(|s| {
        let (r1, r2) = s.split_at(dim.w);
        r1.chunks_exact(2)
          .zip(r2.chunks_exact(2))
          .map(|(a, b)| {
            let scaled = scale_f([a[0], a[1], b[0], b[1]]);
            [clip01(scaled[3]), clip01((scaled[1] + scaled[2]) / 2.0), clip01(scaled[0])]
          })
          .collect::<Vec<_>>()
      })
      .flatten()
      .collect(),
    BayerPattern::GBRG => pixels
      .par_chunks_exact(dim.w * 2)
      .map(|s| {
        let (r1, r2) = s.split_at(dim.w);
        r1.chunks_exact(2)
          .zip(r2.chunks_exact(2))
          .map(|(a, b)| {
            let scaled = scale_f([a[0], a[1], b[0], b[1]]);
            [clip01(scaled[2]), clip01((scaled[0] + scaled[3]) / 2.0), clip01(scaled[1])]
          })
          .collect::<Vec<_>>()
      })
      .flatten()
      .collect(),
    BayerPattern::GRBG => pixels
      .par_chunks_exact(dim.w * 2)
      .map(|s| {
        let (r1, r2) = s.split_at(dim.w);
        r1.chunks_exact(2)
          .zip(r2.chunks_exact(2))
          .map(|(a, b)| {
            let scaled = scale_f([a[0], a[1], b[0], b[1]]);
            [clip01(scaled[1]), clip01((scaled[0] + scaled[3]) / 2.0), clip01(scaled[2])]
          })
          .collect::<Vec<_>>()
      })
      .flatten()
      .collect(),
  };
  (rgb, dim.w >> 1, dim.h >> 1)
}
