#include <metal_stdlib>
using namespace metal;

kernel void two_array_addition(
    device uint* input_a [[buffer(0)]],
    device uint* input_b [[buffer(1)]],
    device uint* output [[buffer(2)]],
    uint2 gid [[thread_position_in_grid]]
) {
    output[gid.x] = input_a[gid.x] + input_b[gid.x];
}

// this is equal to:

kernel void two_array_addition_alias(
    constant uint* input_a [[buffer(0)]],
    constant uint* input_b [[buffer(1)]],
    device uint* output [[buffer(2)]],
    uint idx [[thread_position_in_grid]]
) {
    output[idx] = input_a[idx] + input_b[idx];
}