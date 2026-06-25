cbuffer TemporalAntialiasingConstants : register(b0)
{
    float2 render_target_size;
    float history_weight;
    uint is_history_valid;
};

Texture2D current_frame : register(t0);
Texture2D history_frame : register(t1);
SamplerState linear_clamp_sampler : register(s0);

struct PixelInput
{
    float4 position : SV_POSITION;
    float2 texture_coordinate : TEXCOORD0;
};

PixelInput vertex_main(uint vertex_identifier : SV_VertexID)
{
    PixelInput output;
    float2 position = float2(
        vertex_identifier == 2 ? 3.0f : -1.0f,
        vertex_identifier == 1 ? 3.0f : -1.0f
    );

    output.position = float4(position, 0.0f, 1.0f);
    output.texture_coordinate = float2(
        (position.x + 1.0f) * 0.5f,
        1.0f - ((position.y + 1.0f) * 0.5f)
    );
    return output;
}

float3 sample_current_frame(float2 texture_coordinate)
{
    return current_frame.Sample(linear_clamp_sampler, texture_coordinate).rgb;
}

float4 pixel_main(PixelInput input) : SV_TARGET
{
    float4 current_colour = current_frame.Sample(linear_clamp_sampler, input.texture_coordinate);

    if (is_history_valid == 0)
    {
        return current_colour;
    }

    float2 texel_size = 1.0f / render_target_size;
    float3 neighbourhood_minimum = current_colour.rgb;
    float3 neighbourhood_maximum = current_colour.rgb;

    [unroll]
    for (int horizontal_offset = -1; horizontal_offset <= 1; horizontal_offset++)
    {
        [unroll]
        for (int vertical_offset = -1; vertical_offset <= 1; vertical_offset++)
        {
            float2 offset = float2(horizontal_offset, vertical_offset) * texel_size;
            float3 neighbour_colour = sample_current_frame(input.texture_coordinate + offset);
            neighbourhood_minimum = min(neighbourhood_minimum, neighbour_colour);
            neighbourhood_maximum = max(neighbourhood_maximum, neighbour_colour);
        }
    }

    float3 history_colour = history_frame.Sample(linear_clamp_sampler, input.texture_coordinate).rgb;
    history_colour = clamp(history_colour, neighbourhood_minimum, neighbourhood_maximum);
    float3 resolved_colour = lerp(current_colour.rgb, history_colour, history_weight);
    return float4(resolved_colour, current_colour.a);
}
