use std::io;
use crate::MetricUnit;
pub struct Triangle {
    side_a:f64,
    side_b:f64,
    side_c:f64,
    height_c:f64,
    unit:MetricUnit,
}


impl Triangle {

    pub fn yapi(kenar1:f64,kenar2:f64,kenar3:f64,yükseklik:f64,birim:MetricUnit)->Self{

        Triangle {
            side_a:kenar1,
            side_b:kenar2,
            side_c:kenar3,
            height_c:yükseklik,
            unit:birim,
        }

    }
    pub fn user_input()->Self{
        let mut a_input = String::new();
        let mut b_input=String::new();
        let mut c_input= String::new();
        let mut height_input = String::new();
        let mut unit_input =String::new();

        println!("a kenarı giriniz:");
        io::stdin().read_line(&mut a_input).unwrap();
        let side_a = a_input.trim().parse().unwrap();

        println!("b kenarı giriniz:");
        io::stdin().read_line(&mut b_input).unwrap();
        let side_b =b_input.trim().parse().unwrap();

        println!("c kenarı giriniz:");
        io::stdin().read_line(&mut c_input).unwrap();
        let side_c = c_input.trim().parse().unwrap();

        println!(" c kenarına ait yüksekliği giriniz");
        io::stdin().read_line(&mut height_input).unwrap();
        let height_c = height_input.trim().parse().unwrap();

        println!("lütfen geçerli bir birim giriniz (mm,cm,m):");

        io::stdin().read_line(&mut unit_input).unwrap();

        let unit = match unit_input.trim() {

            "mm" => MetricUnit::Millimeter,
            "cm" => MetricUnit::Centimeter,
            "m"  => MetricUnit::Meter,
            _=> panic!("lütfen geçerli birim giriniz"),
        };

        Triangle::yapi(side_a,side_b,side_c,height_c,unit)


    }
    pub fn area(&self)->f64{
        (self.side_c*self.height_c)/2.00
    }

    pub fn perimeter(&self)->f64{

        self.side_c+self.side_a+self.side_b
    }

    pub fn display_area(&self){

        println!("alan değeri : {:.2} {}",self.area(),self.unit.as_str());

    }
    pub fn display_perimeter(&self){

        println!("üçgenin çevresi : {:.2} {}",self.perimeter(),self.unit.as_str());

    }

}