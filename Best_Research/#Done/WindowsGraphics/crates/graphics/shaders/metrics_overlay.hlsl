struct VertexInput
{
    float2 position : POSITION;
    float4 colour : COLOR;
};

struct PixelInput
{
    float4 position : SV_POSITION;
    float4 colour : COLOR;
};

PixelInput vertex_main(VertexInput input)
{
    PixelInput output;
    output.position = float4(input.position, 0.0f, 1.0f);
    output.colour = input.colour;
    return output;
}

float4 pixel_main(PixelInput input) : SV_TARGET
{
    return input.colour;
}
