cbuffer Transform : register(b0)
{
    row_major float4x4 world_view_projection;
};

static const float cube_scale = 0.75f;

struct VertexInput
{
    float3 position : POSITION;
    float3 color : COLOR;
};

struct PixelInput
{
    float4 position : SV_POSITION;
    float3 color : COLOR;
};

PixelInput vertex_main(VertexInput input)
{
    PixelInput output;
    output.position = mul(float4(input.position * cube_scale, 1.0f), world_view_projection);
    output.color = input.color;
    return output;
}

float4 pixel_main(PixelInput input) : SV_TARGET
{
    return float4(input.color, 1.0f);
}
