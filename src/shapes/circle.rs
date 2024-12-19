use crate:: units::MetricUnit;
use std::io;



pub struct Circle {
    radius: f64,
    unit: MetricUnit,
}

impl Circle {

    pub fn yapi(yaricap:f64,birim:MetricUnit) -> Self {

        Circle{
            radius:yaricap,
            unit:birim,
        }

    }

    pub fn user_input() -> Self{


        let mut unit_input = String::new();
        let radius = loop {

            let mut radius_input = String::new();
            println!("lütfen yarıçap giriniz:");
            io::stdin().read_line(&mut radius_input).unwrap();

            match radius_input.trim().parse::<i32>() {
                Ok(number) => break number as f64,
                Err(_) => println!("Hatalı giriş! Lütfen geçerli bir sayı giriniz."),
            }




    };

        println!("lütfen geçerli bir birim giriniz");
        io::stdin().read_line(&mut unit_input).unwrap();
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
        Circle::yapi(radius,unit)

    }

    pub fn area(&self)->f64{

        3.14*self.radius*self.radius
    }

    pub fn perimeter(&self) ->f64{

        2.0*3.14*self.radius
    }

    pub fn display_area(&self){

        println!("dairenin alanı: {:.2} {} ",self.area(),self.unit.as_str());
    }

    pub fn display_perimeter(&self){

        println!("çemberin çevresi {:.2} {}",self.perimeter(),self.unit.as_str());

    }


}