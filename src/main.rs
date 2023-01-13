use gost_r_3412::Gost;

fn main() {
    let mut key = [0; 32];
    // key[0] = 8;
    let gost = Gost::new(key);
    let data = [0; 16];
    // println!("{:#04X?}", data);
    let encrypted = gost.encrypt(data);
    println!("{:?}", encrypted);
    let decrypted = gost.decrypt(encrypted);
    println!("{:?}", decrypted);
}