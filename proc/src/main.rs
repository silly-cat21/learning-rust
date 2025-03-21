use std::io;

fn main(){

    println!("zacatek programu.");

    let u: i64 = 152; // explicitni deklarace promenne int 64 bitu
    let w: f32 = 3.125; // explicitni deklarace floatu pro 64 bitu

    println!("cislo u je: {} v integeru, cislo w je: {} ve float", u, w);

    // zmenitelnost promennych

    let c1 = 6; // nejde prepsat v kodu
    let mut c2 = 12; // je zmenitelna (mutable)

    println!("cislo 2 pred zmenou je: {}", c2 );

    c2 = 16;
    println!("cislo 2 po zmene je {}", c2);

    // jednoduche operace

    let n = 90;
    let p = 20;

    println!("soucet cisla {} a {} je: {}", n,p,n+p);
    println!("rozdil cisla {} a {} je: {}", n,p,n-p);
    println!("soucin cisla {} a {} je: {}", n,p,n*p);
    println!("podil cisla {} a {} je: {}", n,p,n/p);
    println!("modulo cisel, neboli zbytek, je {} % {} = {}", n, p, n % p); // modulo je zbytek po deleni

    // konstanty 

    const PI: f64 = 3.1415926535; // musi byt velkym pismenem, VZDY DATOVY TYP

    println!("číslo pí je: {} s přesností 10 míst.", PI);

    //podminky, kalkulacka

    let x = 5;
    let y = '*';
    let z = 2;

    let vysl;

    if y == '+'{
        vysl = x + z;
        println!("vysledek je: {}", vysl);
    } 
    else if y == '-'{
        vysl = x - z;
        println!("vysledek je: {}", vysl);
    }
    else if y == '*'{
        vysl = x * z;
        println!("vysledek je: {}", vysl);
    }
    else if y == '/'{
        vysl = x / z;
        println!("vysledek je: {}", vysl);
    }
    else {
        println!("zadal jsi spatny operator");
    }

    //konec kalkulacky
    
    let q = 0;

    if q > 0{
        println!("cislo je kladne");
    }
    else if q < 0{
        println!("cislo je zaporne");
    }
    else {
        println!("cislo je rovno nule");
    }


    




}