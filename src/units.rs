#[derive(Debug)]
pub enum MetricUnit{
    Millimeter,
    Centimeter,
    Meter,
}


impl MetricUnit {
    pub fn as_str(&self) -> &'static str{

        match self {

            MetricUnit::Millimeter => "mm",
            MetricUnit::Centimeter => "cm",
            MetricUnit::Meter => "m"

        }


    }
}




