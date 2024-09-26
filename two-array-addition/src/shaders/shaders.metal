#include <metal_stdlib>
using namespace metal;

kernel void two_array_addition_1(
    constant uint* a [[buffer(0)]],
    constant uint* b [[buffer(1)]],
    device uint* c [[buffer(2)]],
    uint2 gid [[thread_position_in_grid]]
) {
    c[gid.x] = a[gid.x] + b[gid.x];
}

// this is equal to:

kernel void two_array_addition_2(
    constant uint* a [[buffer(0)]],
    constant uint* b [[buffer(1)]],
    device uint* c [[buffer(2)]],
    uint idx [[thread_position_in_grid]]
) {
    c[idx] = a[idx] + b[idx];
}