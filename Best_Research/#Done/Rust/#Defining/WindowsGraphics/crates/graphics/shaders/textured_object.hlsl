cbuffer Transform : register(b0)
{
    row_major float4x4 world_view_projection;
};

Texture2D base_color_texture : register(t0);
SamplerState base_color_sampler : register(s0);

struct VertexInput
{
    float3 position : POSITION;
    float3 color : COLOR;
};

struct PixelInput
{
    float4 position : SV_POSITION;
    float3 color : COLOR;
    float3 object_position : TEXCOORD0;
};

PixelInput vertex_main(VertexInput input)
{
    PixelInput output;
    output.position = mul(float4(input.position, 1.0f), world_view_projection);
    output.color = input.color;
    output.object_position = input.position;
    return output;
}

float2 cube_face_texture_coordinate(float3 object_position)
{
    float3 absolute_position = abs(object_position);

    if (absolute_position.z >= absolute_position.x && absolute_position.z >= absolute_position.y)
    {
        return object_position.z < 0.0f ? float2(1.0f / 6.0f, 0.25f) : float2(0.5f, 0.25f);
    }

    if (absolute_position.x >= absolute_position.y)
    {
        return object_position.x < 0.0f ? float2(5.0f / 6.0f, 0.25f) : float2(1.0f / 6.0f, 0.75f);
    }

    return object_position.y > 0.0f ? float2(0.5f, 0.75f) : float2(5.0f / 6.0f, 0.75f);
}

float4 pixel_main(PixelInput input) : SV_TARGET
{
    float3 texture_color = base_color_texture.Sample(base_color_sampler, cube_face_texture_coordinate(input.object_position)).rgb;
    return float4(input.color * texture_color, 1.0f);
}
