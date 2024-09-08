#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// Needed to access the globals variable like time
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

    // Parameters that affect the speed of the foam
    //
    let foam_speed_x = 0.2;
    let foam_speed_y = 0.07;
    // moving coordinate
    let timed_uv = in.uv.xy + vec2(sin(globals.time * foam_speed_x), cos(globals.time * foam_speed_y));

    // Parameters that affect the size fo the foam
    //
    // First component of the water foam
    // Smaller values result in bigger patch
    let noise_scale = 7.0;
    // limit value for the foam in the noise texture
    // small value result in bigger foams
    let edge = 0.5;
    // speed of the variation of size fo the foam
    let edge_size_speed = 0.5;
    // Size of the variation of size
    let edge_size_scale = 0.2;
    let edge_offset = 0.;
    let timed_edge = edge + (sin(globals.time * edge_size_speed) + edge_offset) * edge_size_scale;
    // width of the foam lines
    let edge_width = 0.06;

    // Multi-level perlin noise
    var noise_value = perlin_noise_2d(timed_uv*noise_scale);
    noise_value -= perlin_noise_2d(timed_uv*noise_scale*2.0) * 0.4 * sin(globals.time* .8);
    noise_value -= perlin_noise_2d(timed_uv*noise_scale*3.0) * 0.2 * sin(globals.time* .4);

    // Wave top
    var wave_top = step(timed_edge, noise_value);

    // edge of the foam
    let foam_offset = 0.2;
    var foam_edges = step(timed_edge -foam_offset, noise_value);
    foam_edges -= step(timed_edge -foam_offset + edge_width*1.0, noise_value);

    let foam = wave_top + foam_edges;

    // Affect the noise to the pixel color
    var color = vec4(0., 0., 0., 0.);
    if foam >= 1. {
      color = vec4(foam, foam, foam, 0.4);
    }

    return color;
}