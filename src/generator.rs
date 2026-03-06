use crate::types::Carpenter930Params;

/// Generate a 512-byte Carpenter 930 .MEM file
pub fn generate_mem_file(params: &Carpenter930Params) -> [u8; 512] {
    let mut buffer = [0u8; 512];
    let mut offset = 0;

    // Helper to write u32 as little-endian
    let write_u32 = |buf: &mut [u8], off: &mut usize, val: u32| {
        buf[*off..*off + 4].copy_from_slice(&val.to_le_bytes());
        *off += 4;
    };

    // Records 1-34: Parameter values
    write_u32(&mut buffer, &mut offset, params.wire_length);
    write_u32(&mut buffer, &mut offset, params.len_correction);
    write_u32(&mut buffer, &mut offset, params.window_strip_length);
    write_u32(&mut buffer, &mut offset, params.trail_strip_length);
    write_u32(&mut buffer, &mut offset, params.trail_pull_length);
    write_u32(&mut buffer, &mut offset, params.full_eject);
    write_u32(&mut buffer, &mut offset, params.window_pull_length);
    write_u32(&mut buffer, &mut offset, params.lead_pull_length);
    write_u32(&mut buffer, &mut offset, params.lead_strip_length);
    write_u32(&mut buffer, &mut offset, params.trail_end_cut);
    write_u32(&mut buffer, &mut offset, params.blade_retract);
    write_u32(&mut buffer, &mut offset, params.blade_dwell);
    write_u32(&mut buffer, &mut offset, params.lead_cut);
    write_u32(&mut buffer, &mut offset, params.wire_type);
    write_u32(&mut buffer, &mut offset, params.wire_size);
    write_u32(&mut buffer, &mut offset, params.tube_size);
    write_u32(&mut buffer, &mut offset, params.blade_open);
    write_u32(&mut buffer, &mut offset, params.airburst_on_off);
    write_u32(&mut buffer, &mut offset, params.airburst_blast);
    write_u32(&mut buffer, &mut offset, params.wire_sensor);
    write_u32(&mut buffer, &mut offset, params.wire_coiler);
    write_u32(&mut buffer, &mut offset, params.wire_stacker);
    write_u32(&mut buffer, &mut offset, params.wire_marker);
    write_u32(&mut buffer, &mut offset, params.wire_list);
    write_u32(&mut buffer, &mut offset, params.work_light);
    write_u32(&mut buffer, &mut offset, params.blade_speed);
    write_u32(&mut buffer, &mut offset, params.feed_speed);
    write_u32(&mut buffer, &mut offset, params.pull_speed);
    write_u32(&mut buffer, &mut offset, params.batch_amount);
    write_u32(&mut buffer, &mut offset, params.number_of_batches);
    write_u32(&mut buffer, &mut offset, params.program_number);
    write_u32(&mut buffer, &mut offset, params.lead_strip_pressure);
    write_u32(&mut buffer, &mut offset, params.trail_strip_pressure);
    write_u32(&mut buffer, &mut offset, params.inch_mm);

    // Records 35-36: 0xFFFFFFFF
    write_u32(&mut buffer, &mut offset, 0xFFFFFFFF);
    write_u32(&mut buffer, &mut offset, 0xFFFFFFFF);

    // Records 37-38: 0x3F800000 (IEEE 754 for 1.0f)
    write_u32(&mut buffer, &mut offset, 0x3F800000);
    write_u32(&mut buffer, &mut offset, 0x3F800000);

    // Records 39-40: 0xFFFFFFFF
    write_u32(&mut buffer, &mut offset, 0xFFFFFFFF);
    write_u32(&mut buffer, &mut offset, 0xFFFFFFFF);

    // Records 41-49: 0x00000000 (9 records)
    for _ in 0..9 {
        write_u32(&mut buffer, &mut offset, 0x00000000);
    }

    // Records 50-60: 0xFFFFFFFF (11 records)
    for _ in 0..11 {
        write_u32(&mut buffer, &mut offset, 0xFFFFFFFF);
    }

    // Records 61-63: Program name (10 bytes) + 0x0000 (2 bytes) + padding
    // This is 12 bytes total, which spans 3 u32 records
    buffer[offset..offset + 10].copy_from_slice(&params.program_name);
    offset += 10;
    buffer[offset..offset + 2].copy_from_slice(&[0x00, 0x00]);
    offset += 2;

    // Record 64: Fixed value 0x0127C8A8
    write_u32(&mut buffer, &mut offset, 0x0127C8A8);

    // Records 65-128: 0xFFFFFFFF (64 records = 256 bytes)
    while offset < 512 {
        write_u32(&mut buffer, &mut offset, 0xFFFFFFFF);
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_size() {
        let params = Carpenter930Params::default();
        let mem = generate_mem_file(&params);
        assert_eq!(mem.len(), 512);
    }

    #[test]
    fn test_program_name() {
        let mut params = Carpenter930Params::new("TEST");
        params.wire_length = 500;
        params.len_correction = 500;

        let mem = generate_mem_file(&params);

        // Check program name at offset 240 (0xF0)
        assert_eq!(&mem[240..250], b"TEST      ");
        assert_eq!(mem[250], 0x00);
        assert_eq!(mem[251], 0x00);
    }

    #[test]
    fn test_fixed_values() {
        let params = Carpenter930Params::default();
        let mem = generate_mem_file(&params);

        // Record 35-36 at offset 136-143: 0xFFFFFFFF
        assert_eq!(&mem[136..140], &[0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(&mem[140..144], &[0xFF, 0xFF, 0xFF, 0xFF]);

        // Record 37-38 at offset 144-151: 0x3F800000 (1.0f)
        assert_eq!(&mem[144..148], &[0x00, 0x00, 0x80, 0x3F]);
        assert_eq!(&mem[148..152], &[0x00, 0x00, 0x80, 0x3F]);

        // Record 64 at offset 252-255: 0x0127C8A8
        assert_eq!(&mem[252..256], &[0xA8, 0xC8, 0x27, 0x01]);
    }
}
