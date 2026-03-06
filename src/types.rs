/// Parameters for Carpenter 930 wire cutter memory file
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Carpenter930Params {
    // Records 1-15: Wire dimensions and cutting parameters
    pub wire_length: u32,              // Record 1
    pub len_correction: u32,           // Record 2
    pub window_strip_length: u32,      // Record 3
    pub trail_strip_length: u32,       // Record 4
    pub trail_pull_length: u32,        // Record 5
    pub full_eject: u32,               // Record 6 (bool: 0 or 0x8000)
    pub window_pull_length: u32,       // Record 7
    pub lead_pull_length: u32,         // Record 8
    pub lead_strip_length: u32,        // Record 9
    pub trail_end_cut: u32,            // Record 10
    pub blade_retract: u32,            // Record 11
    pub blade_dwell: u32,              // Record 12
    pub lead_cut: u32,                 // Record 13
    pub wire_type: u32,                // Record 14 (0=stranded, 1=solid, 2=flat)
    pub wire_size: u32,                // Record 15

    // Records 16-22: Additional equipment parameters
    pub tube_size: u32,                // Record 16
    pub blade_open: u32,               // Record 17
    pub airburst_on_off: u32,          // Record 18 (0=off, 256=on)
    pub airburst_blast: u32,           // Record 19 (0=off, 128=on)
    pub wire_sensor: u32,              // Record 20 (0=off, 64=on)
    pub wire_coiler: u32,              // Record 21 (always 0)
    pub wire_stacker: u32,             // Record 22 (always 0)

    // Records 23-28: Additional features and speeds
    pub wire_marker: u32,              // Record 23 (always 0)
    pub wire_list: u32,                // Record 24 (always 0)
    pub work_light: u32,               // Record 25 (always 0)
    pub blade_speed: u32,              // Record 26 (1-5)
    pub feed_speed: u32,               // Record 27 (1-5)
    pub pull_speed: u32,               // Record 28 (1-5)

    // Records 29-34: Batch and pressure settings
    pub batch_amount: u32,             // Record 29
    pub number_of_batches: u32,        // Record 30
    pub program_number: u32,           // Record 31
    pub lead_strip_pressure: u32,      // Record 32
    pub trail_strip_pressure: u32,     // Record 33
    pub inch_mm: u32,                  // Record 34 (0=inch, 1=mm)

    // Program name: 10 characters max, space-padded
    pub program_name: [u8; 10],
}

impl Default for Carpenter930Params {
    fn default() -> Self {
        Self {
            wire_length: 0,
            len_correction: 0,
            window_strip_length: 0,
            trail_strip_length: 0,
            trail_pull_length: 0,
            full_eject: 0,
            window_pull_length: 0,
            lead_pull_length: 0,
            lead_strip_length: 0,
            trail_end_cut: 0,
            blade_retract: 0,
            blade_dwell: 0,
            lead_cut: 0,
            wire_type: 0,
            wire_size: 0,
            tube_size: 0,
            blade_open: 0,
            airburst_on_off: 0,
            airburst_blast: 0,
            wire_sensor: 0,
            wire_coiler: 0,
            wire_stacker: 0,
            wire_marker: 0,
            wire_list: 0,
            work_light: 0,
            blade_speed: 3,
            feed_speed: 3,
            pull_speed: 3,
            batch_amount: 1,
            number_of_batches: 1,
            program_number: 0,
            lead_strip_pressure: 0,
            trail_strip_pressure: 0,
            inch_mm: 0,
            program_name: *b"          ", // 10 spaces
        }
    }
}

impl Carpenter930Params {
    /// Create new params with a program name
    pub fn new(name: &str) -> Self {
        let mut params = Self::default();
        params.set_program_name(name);
        params
    }

    /// Set program name (truncates to 10 chars, pads with spaces)
    pub fn set_program_name(&mut self, name: &str) {
        self.program_name = *b"          "; // Reset to spaces
        let bytes = name.as_bytes();
        let len = bytes.len().min(10);
        self.program_name[..len].copy_from_slice(&bytes[..len]);
    }
}
