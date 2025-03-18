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

    // if vraci hodnotu do opravneni

    let vek = 18;

    let opravneni = if vek >= 18 { "Dospělý "} else { "Nezletilý" };

    println!("Jsi {}", opravneni);

    // nekonecny cyklus loop

    let mut pocitadlo = 0;

    loop{
        pocitadlo += 1;

        if pocitadlo == 7{
            break; // ukonceni cyklu
        }
        println!("počet: {}", pocitadlo);

    }

    let h = loop{
      let cis = 5;

      break cis * 2; // break vraci hodnotu

    };

    println!("vystup z break je: {}", h);

    // cyklus while
    let mut b = 10;

    while b < 20{
        b += 1;
        println!("číslo: {}", b);
    }

    // for loop
    for i in 1..5{
        println!("i je: {}", i);
    }

     //staticky pole

     let pole: [i32; 5] = [10, 30, 20, 2, 0]; // strednik pro datoveho typu a velikosti

     println!("prvni cislo v poli je: {}", pole[0]); // vypsani z pole
     println!("posledni cislo v poli je: {}", pole[4]);
 
     // vektory, muzou byt mutable
 
     let mut vektor = vec![6, 28, 80, 72]; // vytvoreni vektoru
     vektor.push(8); // prida k vektoru nakonec hodnotu
 
     vektor.pop(); //odebere posledni prvek z vektoru
     
     println!("delka vektoru je: {}", vektor.len());
 
     vektor.remove(0); // odstraneni nulteho indexu
     println!("po odstraneni indexu 1 je delka {}", vektor.len());
 
     println!("obsah vektoru po pridani hodnoty je: {:?}", vektor); // vypsani celeho vektoru
 
     // iterace ve vektoru
 
     for f in vektor{
         println!("číslo je: {}", f);
     }

    // retezce (string)

    let text = "ahoj svete"; // tohle je nezmenitelny string &str
    println!("{}", text);

    // string (zmenitelny)

    let text1 = "ahoj"; // &str
    let prevedeni = String::from(text1);
    println!("vypis prevedeneho stringu je {}", prevedeni);

 
}