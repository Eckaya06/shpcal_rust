pub use crate::units::MetricUnit;
use std::io;

#[derive(Debug)]
pub struct Square{
    length_side:f64,
    unit :MetricUnit,
}

impl Square {

    pub fn yapi(kenar:f64, birim: MetricUnit) -> Self{

        Square {
            length_side : kenar,
            unit : birim,
        }
    }


    pub fn user_input() -> Self{


        let mut unit_input = String::new();

        let length_side= loop {

            let mut length_side_input = String::new();
            println!("lütfen karenin kenar uzunluğunu giriniz:");
            io::stdin().read_line(&mut length_side_input).unwrap();

            match length_side_input.trim().parse::<i32>() {
                Ok(number) => break number as f64,
                Err(_) => println!("Hatalı giriş! Lütfen geçerli bir sayı giriniz."),
            }

        };

        let unit = loop {
            println!("lütfen geçerli bir birim giriniz (mm,cm,m):");
            io::stdin().read_line(&mut unit_input).unwrap();


            match unit_input.trim() {
                "mm" => break MetricUnit::Millimeter,
                "cm" => break MetricUnit::Centimeter,
                "m" => break MetricUnit::Meter,
                _ => {
                    println!("geçersiz birim");
                    unit_input.clear();
                }
            }
        };
        Square::yapi(length_side,unit)


    }

    pub fn area(&self)->f64{
        self.length_side*self.length_side
    }

    pub fn perimeter(&self)->f64{
        self.length_side*4.00
    }

    pub fn display_area(&self){
        println!("alan değeri: {:.2} {}² ",self.area(),self.unit.as_str());
    }

    pub fn display_perimeter(self){
        println!("çevre değeri: {:.2} {} ",self.perimeter(),self.unit.as_str());


    }








}








