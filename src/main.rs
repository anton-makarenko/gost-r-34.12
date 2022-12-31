use gost_r_3412::Gost;

fn main() {
    let key = [0; 32];
    let gost = Gost::new(key);
    let data = [0; 16];
    println!("{:#04X?}", data);
    let encrypted = gost.encrypt(data);
    println!("{:#04X?}", encrypted);
    let decrypted = gost.decrypt(encrypted);
    println!("{:#04X?}", decrypted);
}