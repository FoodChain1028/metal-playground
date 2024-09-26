use core::mem;
use metal::{Buffer, MTLSize};
use objc::rc::autoreleasepool;
use std::time::Instant;
use two_array_addition::abstractions::state::MetalState;

fn main() {
    const SIZE: u32 = 1 << 29;
    println!("This is expected to run for a while... please wait...");

    println!("Generating input arrays...");
    let input_a = (1..SIZE).collect::<Vec<u32>>();
    println!("Generating input arrays...");
    let input_b = input_a.clone().iter().map(|x| x * 2).collect::<Vec<u32>>();

    println!("Generating output array...");
    let output_c = vec![0; (SIZE - 1) as usize];

    let start_expected = Instant::now();
    println!("Generating expected output...");
    let expected_output = (1..SIZE).map(|x| x + x * 2).collect::<Vec<u32>>();
    let duration_expected = start_expected.elapsed();

    let state = MetalState::new(None).unwrap();

    // allocating buffers
    let buffer_alloc_start = Instant::now();
    let buffer_a = state.alloc_buffer_data(&input_a);
    let buffer_b = state.alloc_buffer_data(&input_b);
    let buffer_c = state.alloc_buffer_data(&output_c);
    let buffer_alloc_duration = buffer_alloc_start.elapsed();
    println!(
        "Duration for allocating buffers: {:?}",
        buffer_alloc_duration
    );

    println!("Executing kernel 1...");
    let start_1 = Instant::now();
    let result_1 = execute_kernel(
        "two_array_addition_2",
        &state,
        &buffer_a,
        &buffer_b,
        &buffer_c,
    );
    let duration_1 = start_1.elapsed();

    println!("Executing kernel 2...");
    let start_2 = Instant::now();
    let result_2 = execute_kernel(
        "two_array_addition_2",
        &state,
        &buffer_a,
        &buffer_b,
        &buffer_c,
    );
    let duration_2 = start_2.elapsed();

    println!("Duration expected: {:?}", duration_expected);
    println!("Duration for kernel 1: {:?}", duration_1);
    println!("Duration for kernel 2: {:?}", duration_2);

    assert_eq!(result_1, expected_output);
    assert_eq!(result_2, expected_output);
    println!("You have successfully run the kernels!");
}

fn execute_kernel(
    name: &str,
    state: &MetalState,
    input_a: &Buffer,
    input_b: &Buffer,
    output_c: &Buffer,
) -> Vec<u32> {
    // assert!(input_a.len() == input_b.len() && input_a.len() == output_c.len());
    // let len = input_a.len() as u64;
    let len = input_a.length() as u64 / mem::size_of::<u32>() as u64;

    // 1. Init the MetalState
    //    - we inited it

    // 2. Set up Pipeline State
    let pipeline = state.setup_pipeline(name).unwrap();

    // 3. Allocate the buffers for A, B, and C
    //     - we allocated outside of this function

    let mut result: &[u32] = &[];

    autoreleasepool(|| {
        // 4. Create the command buffer & command encoder
        let (command_buffer, command_encoder) = state.setup_command(
            &pipeline,
            Some(&[(0, input_a), (1, input_b), (2, output_c)]),
        );

        // 5. command encoder dispatch the threadgroup size and num of threads per threadgroup
        let threadgroup_count = MTLSize::new((len + 256 - 1) / 256, 1, 1);
        let thread_per_threadgroup = MTLSize::new(256, 1, 1);

        // let grid_size = MTLSize::new(len, 1, 1);
        // let threadgroup_count = MTLSize::new(pipeline.max_total_threads_per_threadgroup(), 1, 1);
        command_encoder.dispatch_thread_groups(threadgroup_count, thread_per_threadgroup);

        command_encoder.end_encoding();

        command_buffer.commit();
        command_buffer.wait_until_completed();

        // 6. Copy the result back to the host
        let start = Instant::now();
        result = MetalState::retrieve_contents::<u32>(output_c);
        let duration = start.elapsed();
        println!("Duration for copying result back to host: {:?}", duration);
    });

    result.to_vec()
}
