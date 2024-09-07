#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// #import bevy_pbr::mesh_view_bindings::globals

#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals;

fn permute_four(x: vec4<f32>) -> vec4<f32> { return ((x * 34. + 1.) * x) % vec4<f32>(289.); }
fn fade_two(t: vec2<f32>) -> vec2<f32> { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn perlin_noise_2d(P: vec2<f32>) -> f32 {
  var Pi: vec4<f32> = floor(P.xyxy) + vec4<f32>(0., 0., 1., 1.);
  let Pf = fract(P.xyxy) - vec4<f32>(0., 0., 1., 1.);
  Pi = Pi % vec4<f32>(289.); // To avoid truncation effects in permutation
  let ix = Pi.xzxz;
  let iy = Pi.yyww;
  let fx = Pf.xzxz;
  let fy = Pf.yyww;
  let i = permute_four(permute_four(ix) + iy);
  var gx: vec4<f32> = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
  let gy = abs(gx) - 0.5;
  let tx = floor(gx + 0.5);
  gx = gx - tx;
  var g00: vec2<f32> = vec2<f32>(gx.x, gy.x);
  var g10: vec2<f32> = vec2<f32>(gx.y, gy.y);
  var g01: vec2<f32> = vec2<f32>(gx.z, gy.z);
  var g11: vec2<f32> = vec2<f32>(gx.w, gy.w);
  let norm = 1.79284291400159 - 0.85373472095314 *
      vec4<f32>(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
  g00 = g00 * norm.x;
  g01 = g01 * norm.y;
  g10 = g10 * norm.z;
  g11 = g11 * norm.w;
  let n00 = dot(g00, vec2<f32>(fx.x, fy.x));
  let n10 = dot(g10, vec2<f32>(fx.y, fy.y));
  let n01 = dot(g01, vec2<f32>(fx.z, fy.z));
  let n11 = dot(g11, vec2<f32>(fx.w, fy.w));
  let fade_xy = fade_two(Pf.xy);
  let n_x = mix(vec2<f32>(n00, n01), vec2<f32>(n10, n11), vec2<f32>(fade_xy.x));
  let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
  return 2.3 * n_xy;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // let color_a = vec3(0.282, 0.51, 1.0);
    // let color_b = vec3(0.725, 0.816, 0.698);
    // let mixed = mix(color_a, color_b, f);
    
    // return vec4<f32>(in.uv, 0.0, 1.0);
    // return vec4<f32>(in.world_position.x, , 1.0);

    // UV is the "clip" position from 0 to 1 of the "vertex", which is the pixel equivalent
    // This draw a green red to yellow square.
    // return vec4<f32>(in.uv.x, in.uv.y, 0.0, 1.0);
    
    let speed = 0.08;
    let time = globals.time * speed;
    var color = vec4<f32>(0., 0., 0., 0.);
    for(var i:u32=20; i<21; i++) {
        let timed_uv = in.uv.xy + vec2<f32>(sin(time), cos(time));
        let noise = perlin_noise_2d(timed_uv * f32(i));
        color += vec4<f32>(noise, noise, noise*1.7, noise);
    }

    // let noise_value = perlin_noise_2d(in.uv.xy*20.0);
    // let color = vec4<f32>(noise_value, noise_value, noise_value*1.7, noise_value);
    return color;

    // return vec4(1.0, t_1, 0.0, 1.0);

    // return vec4(1.0, 1.0, 1.0, 0.5);
}