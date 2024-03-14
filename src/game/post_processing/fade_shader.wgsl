struct Popr {
    factor: f32,
    time: f32,
    distortion: f32,
    vignette: f32,
};

@group(0) @binding(0)
var<uniform> popr: Popr;

@group(1) @binding(0)
var tex: texture_2d<f32>;
@group(1) @binding(1)
var tex_sampler: sampler;

@group(2) @binding(0)
var paper_tex: texture_2d<f32>;
@group(2) @binding(1)
var paper_tex_sampler: sampler;

struct InstanceInput{
    @location(5) position: vec2<f32>,
    @location(6) scale: vec2<f32>,
    @location(7) rotation: f32,
}

struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput{
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>{
    var uv = vec2<f32>(in.uv.x, 1.0 - in.uv.y);

    let paper_offset = floor(popr.time / 1.2) / 4.0; 
    let paper_uv = fract(uv + paper_offset);
    var paper = textureSample(paper_tex, paper_tex_sampler, paper_uv).xy;
    paper = (paper - 0.5) * 2.0;
    var diffuse = dot(normalize(paper), vec2<f32>(1.0, 1.0));
    diffuse = 1.0 - (diffuse / 80.0) * popr.distortion;
    
    let color = textureSample(tex, tex_sampler, uv + paper * 0.01 * popr.distortion).xyz;
    let vignette = 1.0 - pow(popr.distortion * length(uv - 0.5) * 0.1, 0.33) * popr.vignette;

    return vec4<f32>(color * popr.factor * diffuse * vignette, 1.0);
}