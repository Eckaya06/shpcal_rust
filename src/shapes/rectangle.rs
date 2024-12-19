use std::io;
use crate::units::MetricUnit;



pub struct Rectangle {
    width:f64,
    height:f64,
    unit: MetricUnit,
}

impl Rectangle {

    pub fn yapi(yukseklik:f64,genislik:f64,birim:MetricUnit)->Self{

        Rectangle{
            width: genislik,
            height: yukseklik,
            unit:birim,
        }
    }

    pub fn user_input()->Self{

        let mut width_input = String::new();
        let mut height_input = String::new();
        let mut unit_input= String::new();
        println!("dikdörtgenin genişliğini giriniz:");
        io::stdin().read_line(&mut width_input).unwrap();
        let width = width_input.trim().parse().unwrap();
        println!("dikdörtgenin yüksekliğini giriniz:");
        io::stdin().read_line(&mut height_input).unwrap();
        let height = height_input.trim().parse().unwrap();
        println!("lütfen geçerli bir birim giriniz");
        io::stdin().read_line(&mut unit_input).unwrap();
        let unit = match unit_input.trim() {
            "mm"=>MetricUnit::Millimeter,
            "cm"=>MetricUnit::Centimeter,
            "m"=>MetricUnit::Meter,
            _=>panic!("geçerli birim giriniz"),

        };

        Rectangle::yapi(height,width,unit)

    }

    pub fn area(&self)->f64{
        self.height*self.width
    }

    pub fn perimeter(&self)->f64{

        (self.height+self.width)*2.0

    }

    pub fn display_area(&self){
        println!("alan değeri: {:.2} {}² ",self.area(),self.unit.as_str());
    }

    pub fn display_perimeter(self){
        println!("çevre değeri: {:.2} {} ",self.perimeter(),self.unit.as_str());

    }
}

