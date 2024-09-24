use metal::MTLSize;
use objc::rc::autoreleasepool;
use two_array_addition::abstractions::state::MetalState;

fn main() {
    const SIZE: u32 = 1 << 26;
    // input_a = [1, 2, 3, ..., 2^{30}]
    // input_b = [2, 4, 6, ..., 2 * 2^{30}]
    let input_a = (1..SIZE).collect::<Vec<u32>>();
    let input_b = input_a.clone().iter().map(|x| x * 2).collect::<Vec<u32>>();

    let output_c = vec![0; (SIZE - 1) as usize];
    let expected_output = (1..SIZE).map(|x| x + x * 2).collect::<Vec<u32>>();

    let result = execute_kernel("two_array_addition", &input_a, &input_b, &output_c);
    let result_alias = execute_kernel("two_array_addition_alias", &input_a, &input_b, &output_c);

    assert_eq!(result, expected_output);
    assert_eq!(result_alias, expected_output);
}

fn execute_kernel(name: &str, input_a: &[u32], input_b: &[u32], output_c: &[u32]) -> Vec<u32> {
    assert!(input_a.len() == input_b.len() && input_a.len() == output_c.len());

    let len = input_a.len() as u64;

    // 1. Init the MetalState
    let state = MetalState::new(None).unwrap();

    // 2. Set up Pipeline State
    let pipeline = state.setup_pipeline(name).unwrap();

    // 3. Allocate the buffers for A, B, and C
    let buffer_a = state.alloc_buffer_data::<u32>(input_a);
    let buffer_b = state.alloc_buffer_data::<u32>(input_b);
    let buffer_c = state.alloc_buffer_data::<u32>(output_c);

    let mut result: Vec<u32> = vec![];

    autoreleasepool(|| {
        // 4. Create the command buffer & command encoder
        let (command_buffer, command_encoder) = state.setup_command(
            &pipeline,
            Some(&[(0, &buffer_a), (1, &buffer_b), (2, &buffer_c)]),
        );

        // 5. command encoder dispatch the threadgroup size and num of threads per threadgroup
        let threadgroup_size = MTLSize::new(len, 1, 1);
        let num_threads_per_threadgroup = MTLSize::new(1, 1, 1);

        command_encoder.dispatch_threads(threadgroup_size, num_threads_per_threadgroup);

        command_encoder.end_encoding();

        command_buffer.commit();
        command_buffer.wait_until_completed();

        // 6. Copy the result back to the host
        result = MetalState::retrieve_contents::<u32>(&buffer_c);
    });

    result
}
