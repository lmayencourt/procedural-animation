#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// Needed to access the globals variable like time
#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals;

fn random2( p: vec2<f32> ) -> vec2<f32> {
    return fract(sin(vec2(dot(p,vec2(127.1,311.7)),dot(p,vec2(269.5,183.3))))*43758.5453);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let u_resolution = vec2<f32>(100.);
    var st = in.uv.xy/u_resolution.xy;
    st.x *= u_resolution.x/u_resolution.y;
    var color = vec3<f32>(.0);

    // Scale
    st *= 2000.;

    // Tile the space
    let i_st : vec2<f32> = floor(st);
    let f_st = fract(st);

    var m_dist = 1.;  // minimum distance

    var m_neighbor = vec2<f32>(0.0);
    var m_diff = vec2<f32>(0.0);

    for (var y = -1; y <= 1; y+=1) {
        for (var x = -1; x <= 1; x+=1) {
            // Neighbor place in the grid
            let neighbor = vec2<f32>(f32(x), f32(y));

            // Random position from current + neighbor place in the grid
            let position = i_st + neighbor;
            var point = random2(position);

			// Animate the point
            point = 0.5 + 0.5*sin(globals.time + 6.2831*point);

			// Vector between the pixel and the point
            var diff = neighbor + point - f_st;

            // Distance to the point
            let dist = length(diff);

            if dist < m_dist {
                m_dist = dist;
                m_diff = diff;
                m_neighbor = neighbor;
            }
        }
    }

    m_dist = 8.0;
    // second pass, distance to border
     for (var y = -2; y <= 2; y+=1) {
        for (var x = -2; x <= 2; x+=1) {
            var neighbor = m_neighbor + vec2<f32>(f32(x), f32(y));
            var point = random2(i_st + neighbor);
            point = 0.5 + 0.5*sin(globals.time + 6.2831*point);

            // Vector between the pixel and the point
            var diff = neighbor + point - f_st;

            if ( dot(m_diff-diff,m_diff-diff)>0.00001 ) {
                m_dist = min(m_dist, dot( 0.5*(m_diff+diff), normalize(diff-m_diff) ));
            }
        }
    }

    var voronoi = vec3<f32>(m_dist, m_diff);

    // draw borders
    color = mix(vec3<f32>(1.0), color, smoothstep(0.01, 0.06, voronoi.x));

    return vec4<f32>(color,0.5);
}
