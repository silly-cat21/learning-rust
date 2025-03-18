use std::io;

fn main() {
    println!("Začátek programu.");

    let u: i64 = 152; // explicitní deklarace proměnné int 64 bitů
    let w: f32 = 3.125; // explicitní deklarace floatu pro 64 bitů

    println!("Číslo u je: {} v integeru, číslo w je: {} ve float", u, w);

    // Změnitelnost proměnných
    let _c1 = 6; // nejde přepsat v kódu
    let mut c2 = 12; // je změnitelná (mutable)

    println!("Číslo 2 před změnou je: {}", c2);
    c2 = 16;
    println!("Číslo 2 po změně je: {}", c2);

    // Jednoduché operace
    let n = 90;
    let p = 20;

    println!("Součet čísla {} a {} je: {}", n, p, n + p);
    println!("Rozdíl čísla {} a {} je: {}", n, p, n - p);
    println!("Součin čísla {} a {} je: {}", n, p, n * p);
    println!("Podíl čísla {} a {} je: {}", n, p, n / p);
    println!("Modulo čísel, neboli zbytek, je {} % {} = {}", n, p, n % p); // modulo je zbytek po dělení

    // Konstanty
    const PI: f64 = 3.1415926535; // musí být velkým písmenem, VŽDY DATOVÝ TYP
    println!("Číslo pí je: {} s přesností 10 míst.", PI);

    // Podmínky, kalkulačka
    let x = 5;
    let y = '*';
    let z = 2;

    let vysl;

    if y == '+' {
        vysl = x + z;
        println!("Výsledek je: {}", vysl);
    } else if y == '-' {
        vysl = x - z;
        println!("Výsledek je: {}", vysl);
    } else if y == '*' {
        vysl = x * z;
        println!("Výsledek je: {}", vysl);
    } else if y == '/' {
        vysl = x / z;
        println!("Výsledek je: {}", vysl);
    } else {
        println!("Zadal jsi špatný operátor");
    }

    // Konec kalkulačky
    let q = 0;

    if q > 0 {
        println!("Číslo je kladné");
    } else if q < 0 {
        println!("Číslo je záporné");
    } else {
        println!("Číslo je rovno nule");
    }

    // If vrací hodnotu do oprávnění
    let vek = 18;
    let opravneni = if vek >= 18 { "Dospělý" } else { "Nezletilý" };
    println!("Jsi {}", opravneni);

    // Nekonečný cyklus loop
    let mut pocitadlo = 0;
    loop {
        pocitadlo += 1;

        if pocitadlo == 7 {
            break; // ukončení cyklu
        }
        println!("Počet: {}", pocitadlo);
    }

    let h = loop {
        let cis = 5;
        break cis * 2; // break vrací hodnotu
    };

    println!("Výstup z break je: {}", h);

    // Cyklus while
    let mut b = 10;
    while b < 20 {
        b += 1;
        println!("Číslo: {}", b);
    }

    // For loop
    for i in 1..5 {
        println!("i je: {}", i);
    }

    // Statické pole
    let pole: [i32; 5] = [10, 30, 20, 2, 0]; // středník pro datový typ a velikost
    println!("První číslo v poli je: {}", pole[0]);
    println!("Poslední číslo v poli je: {}", pole[4]);

    // Vektory, mohou být mutable
    let mut vektor = vec![6, 28, 80, 72];
    vektor.push(8); // přidání k vektoru na konec
    vektor.pop(); // odebere poslední prvek z vektoru

    println!("Délka vektoru je: {}", vektor.len());

    vektor.remove(0); // odstranění nultého indexu
    println!("Po odstranění indexu 1 je délka {}", vektor.len());

    println!("Obsah vektoru po přidání hodnoty je: {:?}", vektor);

    // Iterace ve vektoru
    for f in vektor {
        println!("Číslo je: {}", f);
    }

    // Řetězce (string)
    let text = "ahoj světe";
    println!("{}", text);

    let text1 = "ahoj"; // &str
    let prevedeni = String::from(text1);
    println!("Výpis převedeného stringu je {}", prevedeni);

    let mut zprava = String::from("ahoj");
    zprava.push_str(" světe"); // přidání dalšího textu k dynamickému stringu
    zprava.push('!'); // přidání pouze jednoho znaku
    println!("{}", zprava);

    let neco = "     bitcoin to the moon    ".to_string();
    let xd = neco.trim(); // trim vrací dynamický String
    println!("Ořezaný text je: {}", xd);

    let prvni = String::from("hello");
    let druhy = String::from(" rust!");
    println!("{}{}", prvni, druhy);

    // Funkce dělení
    let v = deleni(10, 5);
    println!("Výsledek funkce dělení je: {}", v);

    // Kalkulačka
    let res: f64 = komplet_kalkulacka();
    println!("Výsledek je: {}", res);
}

fn deleni(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        println!("Dělit nulou nelze");
        return 0; // když je dělitel nula, fce vrací nulu
    }
    num1 / num2 // jinak se vrací výsledek, nesmí být středník (návratová hodnota je poslední výraz)
}

fn komplet_kalkulacka() -> f64 { // funkce vrací 64 bitový float
    println!("Zadej první hodnotu: ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Chyba");
    let a: f64 = a.trim().parse().unwrap();

    println!("Zadej operaci:");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Chyba");
    let op = op.trim();

    println!("Zadej druhou hodnotu:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Chyba");
    let b: f64 = b.trim().parse().unwrap();

    let mut vysl = 0.0;

    if op == "+" {
        vysl = a + b;
    } else if op == "-" {
        vysl = a - b;
    } else if op == "*" {
        vysl = a * b;
    } else if op == "/" {
        if b == 0.0 {
            println!("Nulou dělit nelze!");
            return 0.0;
        }
        vysl = a / b;
    } else {
        println!("Zadal jsi špatnou operaci :(");
        return 0.0;
    }

    return vysl;
}
