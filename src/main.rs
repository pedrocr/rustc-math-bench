extern crate time;
extern crate simd;
use simd::f32x4;

fn incr(val: f32, incr: f32) -> f32 {
  let newval = val+incr;

  if newval > 1.0 { 0.0 } else { newval }
}

fn main() {
  let num_pixels = 100000000; // 100MP image
  // CAM to RGB matrix taken from the Sony A77V
  let matrix: [[f32;4];3] = [
    [0.75925,      0.21533987,  0.020964727, 0.0],
    [0.28515315,   1.0129657,  -0.2981189,   0.0],
    [0.021620918, -0.19883351,  1.177143,    0.0],
  ];

  let mut inb = vec![0f32; num_pixels*4];

  // Fill in the input buffer with different rgb values
  let mut r = 0f32;
  let mut g = 0f32;
  let mut b = 0f32;
  for pix in inb.chunks_mut(4) {
    r = incr(r, 0.01);
    g = incr(g, 0.02);
    b = incr(b, 0.03);

    pix[0] = r;
    pix[1] = g;
    pix[2] = b;
    pix[3] = 0f32;
  }

  let mut out = vec![0f32; num_pixels*3];
  let from_time = time::precise_time_ns();
  for (pixin, pixout) in inb.chunks(4).zip(out.chunks_mut(3)) {
    let r = pixin[0];
    let g = pixin[1];
    let b = pixin[2];
    let e = pixin[3];

    pixout[0] = r * matrix[0][0] + g * matrix[0][1] + b * matrix[0][2] + e * matrix[0][3];
    pixout[1] = r * matrix[1][0] + g * matrix[1][1] + b * matrix[1][2] + e * matrix[1][3];
    pixout[2] = r * matrix[2][0] + g * matrix[2][1] + b * matrix[2][2] + e * matrix[2][3];
  }
  let to_time = time::precise_time_ns();
  let mut sum = 0f64;
  for v in out {
    sum += v as f64;
  }
  println!("{:.2} ms/megapixel (sum is {})",
         ((to_time - from_time) as f32)/((num_pixels as f32)),
         sum);

  let mut out = vec![0f32; num_pixels*3];
  let from_time = time::precise_time_ns();
  let x_rgb = f32x4::load(&matrix[0], 0);
  let y_rgb = f32x4::load(&matrix[1], 0);
  let z_rgb = f32x4::load(&matrix[2], 0);
  for (pixin, pixout) in inb.chunks(4).zip(out.chunks_mut(3)) {
    let rgb = f32x4::load(&pixin, 0);
    let x_comps = rgb * x_rgb;
    let y_comps = rgb * y_rgb;
    let z_comps = rgb * z_rgb;
    pixout[0] = x_comps.extract(0) + x_comps.extract(1) + x_comps.extract(2);
    pixout[1] = y_comps.extract(0) + y_comps.extract(1) + y_comps.extract(2);
    pixout[2] = z_comps.extract(0) + z_comps.extract(1) + z_comps.extract(2);
  }
  let to_time = time::precise_time_ns();
  let mut sum = 0f64;
  for v in out {
    sum += v as f64;
  }
  println!("{:.2} ms/megapixel (sum is {}) (explicit simd)",
         ((to_time - from_time) as f32)/((num_pixels as f32)),
         sum);
}
