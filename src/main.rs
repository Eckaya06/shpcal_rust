use std::io;
use shpcal_rust::{Circle, Rectangle, Square, Triangle};

fn main() {
    loop {


            println!("Geometrik Şekil işlemleri:");
            println!("1:  Kare");
            println!("2:  Dikdörtgen");
            println!("3:  Daire");
            println!("4:  Üçgen");

            let mut secim = String::new();
            io::stdin().read_line(&mut secim).unwrap();

            match secim.trim() {
                "1" =>  handle_square(),
                "2" => handle_rectangle(),
                "3" => handle_circle(),
                "4" => handle_triangle(),

                _ => {
                    println!("geçersiz işlem lütfen mevcut işlemlerden birini seçiniz");
                    continue;
                }

            }


        println!("yapmak istediğiniz başka bir işlem var mı y/n ?");
        let mut user_input =String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        match user_input.trim() {
            "y"=>continue,
            "n"=> {
                println!("program kapatılıyor....");
                break
            },

            _=> panic!("geçersiz işlem"),





        }







    }
}


/*fn main() {
    let mut devam = true;
    while devam {
        println!("Geometrik Şekil İşlemleri:");
        println!("1: Kare");
        println!("2: Dikdörtgen");
        println!("3: Daire");
        println!("4: Üçgen");
        println!("Çıkmak için 'q' tuşlayın.");

        let mut secim = String::new();
        io::stdin().read_line(&mut secim).unwrap();

        match secim.trim() {
            "1" => handle_square(),
            "2" => handle_rectangle(),
            "3" => handle_circle(),
            "4" => handle_triangle(),
            "q" => {
                println!("Program kapatılıyor...");
                devam = false;
            }
            _ => println!("Geçersiz işlem, lütfen tekrar deneyin."),
        }

        if devam {
            println!("Başka bir işlem yapmak ister misiniz? (y/n):");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();

            devam = match user_input.trim() {
                "y" => true,
                "n" => false,
                _ => {
                    println!("Geçersiz giriş, işlem devam ediyor...");
                    true
                }
            };
        }
    }
}*/



fn handle_square() {
    let user_obj = Square::user_input();
    println!("yapmak istediğiniz işlemi seçiniz:");
    println!("area or perimeter");
    let mut user_input = String::new();
    io::stdin().read_line( &mut user_input).unwrap();

    if user_input.trim() == "area" {
        user_obj.display_area();
    }
    else {
        user_obj.display_perimeter();

    }






}
fn handle_rectangle(){
    let user_obj = Rectangle::user_input();
    println!("yapmak istediğiniz işlemi seçiniz:");
    println!("area or perimeter");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    if user_input.trim() == "area" {
        user_obj.display_area()
    }
    else {

        user_obj.display_perimeter()
    }

}

fn handle_circle(){

    let user_obj = Circle::user_input();
    println!("yapmak istediğiniz işlemi seçiniz:");
    println!("area or perimeter");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    match user_input.trim(){
        "area"=> user_obj.display_area(),
        "perimeter"=>user_obj.display_perimeter(),
        _=> panic!("geçersiz işlem"),

    }


}

fn handle_triangle(){

    let user_obj = Triangle::user_input();

    let mut user_input = String::new();
    println!("yapmak istediğiniz işlemi seçiniz:");
    println!("area or perimeter");
    io::stdin().read_line(&mut user_input).unwrap();
    match user_input.trim() {

        "area" => user_obj.display_area(),
        "perimeter"=>user_obj.display_perimeter(),
        _=> panic!("geçersiz işlem"),
    }







}
