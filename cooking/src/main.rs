use std::io;

fn main() {
    println!("Vítej v programu pro zjištění pohlaví a data narození z rodného čísla:");
    println!("Zadej své rodné číslo ve formátu (XXXXXX/YYYY)");

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Chyba při načítání hodnoty.");

    let a = a.trim();

    let mesic_str = &a[2..4];
    let mesic: u32 = mesic_str.parse().expect("Chyba při převodu měsíce");

    if mesic > 50 {
        println!("Jsi žena");
    } else if mesic <= 50 {
        println!("Jsi muž");
    } else {
        println!("Zadal jsi neplatné rodné číslo");
    }

}