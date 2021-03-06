#![macro_use]

macro_rules! gt0 {
    ($rshift: expr) => {
        if ($rshift) & 0x1 > 0 {true} else {false}
    }
}

macro_rules! to_u16 {
    ($data: expr, $start: expr) => {
        ($data[$start + 1] as u16) | (($data[$start] as u16) << 8)
    }
}

macro_rules! to_u32 {
    ($data: expr, $start: expr) => {
        ($data[$start + 3] as u32) | (($data[$start + 2] as u32) << 8) |
        (($data[$start + 1] as u32) << 16) |
        (($data[$start] as u32) << 24)
    }
}

macro_rules! to_ptr {
    ($data: expr, $start: expr) => {
        $data[($start as usize) + 1] as u32
    }
}

macro_rules! u16_to_ptr {
    ($data: expr) => {
        ($data & 0xff) as u32
    }
}
