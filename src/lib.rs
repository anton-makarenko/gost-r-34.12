const PI: [u8; 256] = [0xFC, 0xEE, 0xDD, 0x11, 0xCF, 0x6E, 0x31, 0x16, 0xFB,  0xC4, 0xFA, 0xDA, 0x23, 0xC5, 0x04, 0x4D,
    0xE9, 0x77, 0xF0, 0xDB, 0x93, 0x2E, 0x99, 0xBA, 0x17, 0x36, 0xF1, 0xBB, 0x14, 0xCD, 0x5F, 0xC1,
    0xF9, 0x18, 0x65, 0x5A, 0xE2, 0x5C, 0xEF, 0x21, 0x81, 0x1C, 0x3C, 0x42, 0x8B, 0x01, 0x8E, 0x4F,
    0x05, 0x84, 0x02, 0xAE, 0xE3, 0x6A, 0x8F, 0xA0, 0x06, 0x0B, 0xED, 0x98, 0x7F, 0xD4, 0xD3, 0x1F,
    0xEB, 0x34, 0x2C, 0x51, 0xEA, 0xC8, 0x48, 0xAB, 0xF2, 0x2A, 0x68, 0xA2, 0xFD, 0x3A, 0xCE, 0xCC,
    0xB5, 0x70, 0x0E, 0x56, 0x08, 0x0C, 0x76, 0x12, 0xBF, 0x72, 0x13, 0x47, 0x9C, 0xB7, 0x5D, 0x87,
    0x15, 0xA1, 0x96, 0x29, 0x10, 0x7B, 0x9A, 0xC7, 0xF3, 0x91, 0x78, 0x6F, 0x9D, 0x9E, 0xB2, 0xB1,
    0x32, 0x75, 0x19, 0x3D, 0xFF, 0x35, 0x8A, 0x7E, 0x6D, 0x54, 0xC6, 0x80, 0xC3, 0xBD, 0x0D, 0x57,
    0xDF, 0xF5, 0x24, 0xA9, 0x3E, 0xA8, 0x43, 0xC9, 0xD7, 0x79, 0xD6, 0xF6, 0x7C, 0x22, 0xB9, 0x03,
    0xE0, 0x0F, 0xEC, 0xDE, 0x7A, 0x94, 0xB0, 0xBC, 0xDC, 0xE8, 0x28, 0x50, 0x4E, 0x33, 0x0A, 0x4A,
    0xA7, 0x97, 0x60, 0x73, 0x1E, 0x00, 0x62, 0x44, 0x1A, 0xB8, 0x38, 0x82, 0x64, 0x9F, 0x26, 0x41,
    0xAD, 0x45, 0x46, 0x92, 0x27, 0x5E, 0x55, 0x2F, 0x8C, 0xA3, 0xA5, 0x7D, 0x69, 0xD5, 0x95, 0x3B,
    0x07, 0x58, 0xB3, 0x40, 0x86, 0xAC, 0x1D, 0xF7, 0x30, 0x37, 0x6B, 0xE4, 0x88, 0xD9, 0xE7, 0x89,
    0xE1, 0x1B, 0x83, 0x49, 0x4C, 0x3F, 0xF8, 0xFE, 0x8D, 0x53, 0xAA, 0x90, 0xCA, 0xD8, 0x85, 0x61,
    0x20, 0x71, 0x67, 0xA4, 0x2D, 0x2B, 0x09, 0x5B, 0xCB, 0x9B, 0x25, 0xD0, 0xBE, 0xE5, 0x6C, 0x52,
    0x59, 0xA6, 0x74, 0xD2, 0xE6, 0xF4, 0xB4, 0xC0, 0xD1, 0x66, 0xAF, 0xC2, 0x39, 0x4B, 0x63, 0xB6];

const R_PI: [u8; 256] = [0xA5, 0x2D, 0x32, 0x8F, 0x0E, 0x30, 0x38, 0xC0, 0x54, 0xE6, 0x9E, 0x39, 0x55, 0x7E, 0x52, 0x91,
    0x64, 0x03, 0x57, 0x5A, 0x1C, 0x60, 0x07, 0x18, 0x21, 0x72, 0xA8, 0xD1, 0x29, 0xC6, 0xA4, 0x3F,
    0xE0, 0x27, 0x8D, 0x0C, 0x82, 0xEA, 0xAE, 0xB4, 0x9A, 0x63, 0x49, 0xE5, 0x42, 0xE4, 0x15, 0xB7,
    0xC8, 0x06, 0x70, 0x9D, 0x41, 0x75, 0x19, 0xC9, 0xAA, 0xFC, 0x4D, 0xBF, 0x2A, 0x73, 0x84, 0xD5,
    0xC3, 0xAF, 0x2B, 0x86, 0xA7, 0xB1, 0xB2, 0x5B, 0x46, 0xD3, 0x9F, 0xFD, 0xD4, 0x0F, 0x9C, 0x2F,
    0x9B, 0x43, 0xEF, 0xD9, 0x79, 0xB6, 0x53, 0x7F, 0xC1, 0xF0, 0x23, 0xE7, 0x25, 0x5E, 0xB5, 0x1E,
    0xA2, 0xDF, 0xA6, 0xFE, 0xAC, 0x22, 0xF9, 0xE2, 0x4A, 0xBC, 0x35, 0xCA, 0xEE, 0x78, 0x05, 0x6B,
    0x51, 0xE1, 0x59, 0xA3, 0xF2, 0x71, 0x56, 0x11, 0x6A, 0x89, 0x94, 0x65, 0x8C, 0xBB, 0x77, 0x3C,
    0x7B, 0x28, 0xAB, 0xD2, 0x31, 0xDE, 0xC4, 0x5F, 0xCC, 0xCF, 0x76, 0x2C, 0xB8, 0xD8, 0x2E, 0x36,
    0xDB, 0x69, 0xB3, 0x14, 0x95, 0xBE, 0x62, 0xA1, 0x3B, 0x16, 0x66, 0xE9, 0x5C, 0x6C, 0x6D, 0xAD,
    0x37, 0x61, 0x4B, 0xB9, 0xE3, 0xBA, 0xF1, 0xA0, 0x85, 0x83, 0xDA, 0x47, 0xC5, 0xB0, 0x33, 0xFA,
    0x96, 0x6F, 0x6E, 0xC2, 0xF6, 0x50, 0xFF, 0x5D, 0xA9, 0x8E, 0x17, 0x1B, 0x97, 0x7D, 0xEC, 0x58,
    0xF7, 0x1F, 0xFB, 0x7C, 0x09, 0x0D, 0x7A, 0x67, 0x45, 0x87, 0xDC, 0xE8, 0x4F, 0x1D, 0x4E, 0x04,
    0xEB, 0xF8, 0xF3, 0x3E, 0x3D, 0xBD, 0x8A, 0x88, 0xDD, 0xCD, 0x0B, 0x13, 0x98, 0x02, 0x93, 0x80,
    0x90, 0xD0, 0x24, 0x34, 0xCB, 0xED, 0xF4, 0xCE, 0x99, 0x10, 0x44, 0x40, 0x92, 0x3A, 0x01, 0x26,
    0x12, 0x1A, 0x48, 0x68, 0xF5, 0x81, 0x8B, 0xC7, 0xD6, 0x20, 0x0A, 0x08, 0x00, 0x4C, 0xD7, 0x74];

const L_VECTOR: [u8; 16] = [1, 148, 32, 133, 16, 194, 192, 1, 251, 1, 192, 194, 16, 133, 32, 148];

pub struct Gost {
    key: [u8; 32],
    round_consts: [[u8; 16]; 32],
    round_keys: [[u8; 16]; 10],
}

impl Gost {
    pub fn new(key: [u8; 32]) -> Gost {
        let mut gost = Gost {
            key,
            round_consts: [[0; 16]; 32],
            round_keys: [[0; 16]; 10],
        };
        let mut left_part: [u8; 16] = [0; 16];
        let mut right_part: [u8; 16] = [0; 16];
        left_part.copy_from_slice(&key[..16]);
        right_part.copy_from_slice(&key[16..]);
        gost.init_round_keys(left_part, right_part);
        gost
    }

    fn init_round_consts(&mut self) {
        let mut round_num: [[u8; 16]; 32] = [[0; 16]; 32];
        for i in 0..32 {
            round_num[i][0] = (i + 1) as u8;
        }
        for i in 0..32 {
            self.round_consts[i] = l(round_num[i]);
        }
    }

    fn init_round_keys(&mut self, left: [u8; 16], right: [u8; 16]) {
        self.init_round_consts();
        self.round_keys[0].copy_from_slice(&left);
        self.round_keys[1].copy_from_slice(&right);
        let mut cur_round: [[u8; 16]; 2] = [left, right];
        for i in 0..4 {
            for j in 0..8 {
                cur_round = feistel_round(cur_round[0], cur_round[1], self.round_consts[j + 8 * i]);
                self.round_keys[2 * i + 2] = cur_round[0];
                self.round_keys[2 * i + 3] = cur_round[1];
            }
        }
    }

    pub fn encrypt(&self, input_block: [u8; 16]) -> [u8; 16] {
        let mut output_block: [u8; 16] = [0; 16];
        output_block.copy_from_slice(&input_block);
        for i in 0..9 {
            output_block = xor(output_block, self.round_keys[i]);
            output_block = s(output_block);
            output_block = l(output_block);
        }
        output_block
    }

    pub fn decrypt(&self, input_block: [u8; 16]) -> [u8; 16] {
        let mut output_block: [u8; 16] = [0; 16];
        output_block.copy_from_slice(&input_block);
        for i in 0..9 {
            output_block = xor(output_block, self.round_keys[i]);
            output_block = r_s(output_block);
            output_block = r_l(output_block);
        }
        output_block
    }
}

fn r(input: [u8; 16]) -> [u8; 16] {
    let mut a15: u8 = 0;
    let mut output: [u8; 16] = [0; 16];
    for i in (0..16).rev() {
        if i == 0 {
            output[15] = input[i];
        }
        else {
            output[i - 1] = input[i];
        }
        a15 ^= multiply_gf(input[i], L_VECTOR[i]);
    }
    output[15] = a15;
    output
}

fn l(input: [u8; 16]) -> [u8; 16] {
    let mut output: [u8; 16] = [0; 16];
    output.copy_from_slice(&input);
    for _ in 0..16 {
        output = r(input);
    }
    output
}

fn s(input: [u8; 16]) -> [u8; 16] {
    let mut output: [u8; 16] = [0; 16];
    for i in 0..16 {
        output[i] = PI[input[i] as usize];
    }
    output
}

fn multiply_gf(mut left: u8, mut right: u8) -> u8 {
    let mut result: u8 = 0;
    let mut h_bit: u8;
    for _ in 1..8 {
        if (right & 1) == 1 {
            result ^= left;
        }
        h_bit = left & 0x80;
        left <<= 1;
        if h_bit < 0 {
            left ^= 0xC3;
        }
        right >>= 1;
    }
    result
}

fn xor(left: [u8; 16], right: [u8; 16]) -> [u8; 16] {
    let mut result: [u8; 16] = [0; 16];
    for i in 0..16 {
        result[i] = left[i] ^ right[i];
    }
    result
}

fn feistel_round(in_left: [u8; 16], in_right: [u8; 16], round_const: [u8; 16]) -> [[u8; 16]; 2] {
    let mut temp: [u8; 16];
    let mut out_right: [u8; 16] = [0; 16];
    out_right.copy_from_slice(&in_left);
    temp = xor(in_left, round_const);
    temp = s(temp);
    temp = l(temp);
    let out_left = xor(temp, in_right);
    [out_left, out_right]
}

fn r_s(input: [u8; 16]) -> [u8; 16] {
    let mut output: [u8; 16] = [0; 16];
    for i in 0..16 {
        output[i] = R_PI[input[i] as usize];
    }
    output
}

fn r_r(input: [u8; 16]) -> [u8; 16] {
    let mut a0 = input[15];
    let mut output = [0; 16];
    for i in 1..16 {
        output[i] = input[i - 1];
        a0 ^= multiply_gf(output[i], L_VECTOR[i]);
    }
    output[0] = a0;
    output
}

fn r_l(input: [u8; 16]) -> [u8; 16] {
    let mut output: [u8; 16] = [0; 16];
    output.copy_from_slice(&input);
    for _ in 0..16 {
        output = r_r(input);
    }
    output
}