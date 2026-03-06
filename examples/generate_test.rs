use carpenter930::Carpenter930Params;
use carpenter930::generate_mem_file;

fn main() {
    // Recreate the TEST example
    let mut params = Carpenter930Params::new("TEST");

    // Set parameters from the example file
    params.wire_length = 500;
    params.len_correction = 500;
    params.window_strip_length = 0;
    params.trail_strip_length = 25;
    params.trail_pull_length = 16;
    params.full_eject = 0x8000;  // 32768
    params.window_pull_length = 0;
    params.lead_pull_length = 16;
    params.lead_strip_length = 25;
    params.trail_end_cut = 46;
    params.blade_retract = 10;
    params.blade_dwell = 25;
    params.lead_cut = 46;
    params.wire_type = 0;
    params.wire_size = 18;
    params.tube_size = 3;
    params.blade_open = 450;
    params.airburst_on_off = 0;
    params.airburst_blast = 0;
    params.wire_sensor = 64;
    params.wire_coiler = 0;
    params.wire_stacker = 0;
    params.wire_marker = 0;
    params.wire_list = 0;
    params.work_light = 0;
    params.blade_speed = 3;
    params.feed_speed = 5;
    params.pull_speed = 3;
    params.batch_amount = 2;
    params.number_of_batches = 1;
    params.program_number = 3;
    params.lead_strip_pressure = 205;
    params.trail_strip_pressure = 205;
    params.inch_mm = 0;

    let mem_data = generate_mem_file(&params);

    // Write to test output
    std::fs::write("test.mem", &mem_data).expect("Failed to write test file");

    println!("Generated test.mem");
    println!("Compare with: od -A x -t x1z test.mem");
}
