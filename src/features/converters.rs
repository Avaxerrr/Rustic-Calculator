#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConverterCategory {
    Volume,
    Length,
    WeightMass,
    Temperature,
    Energy,
    Area,
    Speed,
    Time,
    Power,
    Data,
    Pressure,
    Angle,
}

impl ConverterCategory {
    pub fn title(self) -> &'static str {
        match self {
            ConverterCategory::Volume => "Volume",
            ConverterCategory::Length => "Length",
            ConverterCategory::WeightMass => "Weight and mass",
            ConverterCategory::Temperature => "Temperature",
            ConverterCategory::Energy => "Energy",
            ConverterCategory::Area => "Area",
            ConverterCategory::Speed => "Speed",
            ConverterCategory::Time => "Time",
            ConverterCategory::Power => "Power",
            ConverterCategory::Data => "Data",
            ConverterCategory::Pressure => "Pressure",
            ConverterCategory::Angle => "Angle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit {
    pub name: &'static str,
    pub symbol: &'static str,
    pub to_base_factor: f64,
}

impl Unit {
    pub const fn new(name: &'static str, symbol: &'static str, to_base_factor: f64) -> Self {
        Self {
            name,
            symbol,
            to_base_factor,
        }
    }
}

pub const CATEGORIES: &[ConverterCategory] = &[
    ConverterCategory::Volume,
    ConverterCategory::Length,
    ConverterCategory::WeightMass,
    ConverterCategory::Temperature,
    ConverterCategory::Energy,
    ConverterCategory::Area,
    ConverterCategory::Speed,
    ConverterCategory::Time,
    ConverterCategory::Power,
    ConverterCategory::Data,
    ConverterCategory::Pressure,
    ConverterCategory::Angle,
];

pub const VOLUME_UNITS: &[Unit] = &[
    Unit::new("Liter", "L", 1.0),
    Unit::new("Milliliter", "mL", 0.001),
    Unit::new("Cubic meter", "m3", 1_000.0),
    Unit::new("Cubic centimeter", "cm3", 0.001),
    Unit::new("US gallon", "gal", 3.785_411_784),
    Unit::new("US quart", "qt", 0.946_352_946),
    Unit::new("US pint", "pt", 0.473_176_473),
    Unit::new("US cup", "cup", 0.236_588_236_5),
    Unit::new("Fluid ounce", "fl oz", 0.029_573_529_562_5),
];

pub const LENGTH_UNITS: &[Unit] = &[
    Unit::new("Meter", "m", 1.0),
    Unit::new("Kilometer", "km", 1_000.0),
    Unit::new("Centimeter", "cm", 0.01),
    Unit::new("Millimeter", "mm", 0.001),
    Unit::new("Micrometer", "um", 0.000_001),
    Unit::new("Nanometer", "nm", 0.000_000_001),
    Unit::new("Mile", "mi", 1_609.344),
    Unit::new("Yard", "yd", 0.9144),
    Unit::new("Foot", "ft", 0.3048),
    Unit::new("Inch", "in", 0.0254),
    Unit::new("Nautical mile", "nmi", 1_852.0),
];

pub const WEIGHT_MASS_UNITS: &[Unit] = &[
    Unit::new("Kilogram", "kg", 1.0),
    Unit::new("Gram", "g", 0.001),
    Unit::new("Milligram", "mg", 0.000_001),
    Unit::new("Metric ton", "t", 1_000.0),
    Unit::new("Pound", "lb", 0.453_592_37),
    Unit::new("Ounce", "oz", 0.028_349_523_125),
    Unit::new("Stone", "st", 6.350_293_18),
];

pub const TEMPERATURE_UNITS: &[&str] = &["Celsius", "Fahrenheit", "Kelvin"];

pub const ENERGY_UNITS: &[Unit] = &[
    Unit::new("Joule", "J", 1.0),
    Unit::new("Kilojoule", "kJ", 1_000.0),
    Unit::new("Calorie", "cal", 4.184),
    Unit::new("Kilocalorie", "kcal", 4_184.0),
    Unit::new("Watt hour", "Wh", 3_600.0),
    Unit::new("Kilowatt hour", "kWh", 3_600_000.0),
    Unit::new("Electronvolt", "eV", 1.602_176_634e-19),
    Unit::new("British thermal unit", "BTU", 1_055.055_852_62),
];

pub const AREA_UNITS: &[Unit] = &[
    Unit::new("Square meter", "m2", 1.0),
    Unit::new("Square kilometer", "km2", 1_000_000.0),
    Unit::new("Square centimeter", "cm2", 0.0001),
    Unit::new("Square millimeter", "mm2", 0.000_001),
    Unit::new("Square mile", "mi2", 2_589_988.110_336),
    Unit::new("Square yard", "yd2", 0.836_127_36),
    Unit::new("Square foot", "ft2", 0.092_903_04),
    Unit::new("Square inch", "in2", 0.000_645_16),
    Unit::new("Hectare", "ha", 10_000.0),
    Unit::new("Acre", "acre", 4_046.856_422_4),
];

pub const SPEED_UNITS: &[Unit] = &[
    Unit::new("Meter per second", "m/s", 1.0),
    Unit::new("Kilometer per hour", "km/h", 0.277_777_777_777_777_8),
    Unit::new("Mile per hour", "mph", 0.447_04),
    Unit::new("Foot per second", "ft/s", 0.3048),
    Unit::new("Knot", "kn", 0.514_444_444_444_444_5),
];

pub const TIME_UNITS: &[Unit] = &[
    Unit::new("Second", "s", 1.0),
    Unit::new("Millisecond", "ms", 0.001),
    Unit::new("Microsecond", "us", 0.000_001),
    Unit::new("Minute", "min", 60.0),
    Unit::new("Hour", "h", 3_600.0),
    Unit::new("Day", "d", 86_400.0),
    Unit::new("Week", "wk", 604_800.0),
    Unit::new("Year", "yr", 31_536_000.0),
];

pub const POWER_UNITS: &[Unit] = &[
    Unit::new("Watt", "W", 1.0),
    Unit::new("Kilowatt", "kW", 1_000.0),
    Unit::new("Megawatt", "MW", 1_000_000.0),
    Unit::new("Horsepower", "hp", 745.699_871_582_270_2),
    Unit::new("BTU per hour", "BTU/h", 0.293_071_070_172_222_2),
];

pub const DATA_UNITS: &[Unit] = &[
    Unit::new("Bit", "bit", 1.0),
    Unit::new("Byte", "B", 8.0),
    Unit::new("Kilobit", "kbit", 1_000.0),
    Unit::new("Kilobyte", "KB", 8_000.0),
    Unit::new("Megabit", "Mbit", 1_000_000.0),
    Unit::new("Megabyte", "MB", 8_000_000.0),
    Unit::new("Gigabit", "Gbit", 1_000_000_000.0),
    Unit::new("Gigabyte", "GB", 8_000_000_000.0),
    Unit::new("Kibibyte", "KiB", 8_192.0),
    Unit::new("Mebibyte", "MiB", 8_388_608.0),
    Unit::new("Gibibyte", "GiB", 8_589_934_592.0),
];

pub const PRESSURE_UNITS: &[Unit] = &[
    Unit::new("Pascal", "Pa", 1.0),
    Unit::new("Kilopascal", "kPa", 1_000.0),
    Unit::new("Bar", "bar", 100_000.0),
    Unit::new("Atmosphere", "atm", 101_325.0),
    Unit::new("Pound per square inch", "psi", 6_894.757_293_168_361),
    Unit::new("Torr", "Torr", 133.322_368_421_052_63),
];

pub const ANGLE_UNITS: &[Unit] = &[
    Unit::new("Radian", "rad", 1.0),
    Unit::new("Degree", "deg", std::f64::consts::PI / 180.0),
    Unit::new("Gradian", "grad", std::f64::consts::PI / 200.0),
    Unit::new("Arcminute", "arcmin", std::f64::consts::PI / 10_800.0),
    Unit::new("Arcsecond", "arcsec", std::f64::consts::PI / 648_000.0),
];

pub fn units_for(category: ConverterCategory) -> Option<&'static [Unit]> {
    match category {
        ConverterCategory::Temperature => None,
        ConverterCategory::Volume => Some(VOLUME_UNITS),
        ConverterCategory::Length => Some(LENGTH_UNITS),
        ConverterCategory::WeightMass => Some(WEIGHT_MASS_UNITS),
        ConverterCategory::Energy => Some(ENERGY_UNITS),
        ConverterCategory::Area => Some(AREA_UNITS),
        ConverterCategory::Speed => Some(SPEED_UNITS),
        ConverterCategory::Time => Some(TIME_UNITS),
        ConverterCategory::Power => Some(POWER_UNITS),
        ConverterCategory::Data => Some(DATA_UNITS),
        ConverterCategory::Pressure => Some(PRESSURE_UNITS),
        ConverterCategory::Angle => Some(ANGLE_UNITS),
    }
}

pub fn convert(category: ConverterCategory, value: f64, from: &str, to: &str) -> Option<f64> {
    if !value.is_finite() {
        return None;
    }

    if category == ConverterCategory::Temperature {
        return convert_temperature(value, from, to);
    }

    let units = units_for(category)?;
    convert_linear(value, units, from, to)
}

pub fn convert_linear(value: f64, units: &[Unit], from: &str, to: &str) -> Option<f64> {
    let from = find_unit(units, from)?;
    let to = find_unit(units, to)?;

    Some(value * from.to_base_factor / to.to_base_factor)
}

pub fn convert_temperature(value: f64, from: &str, to: &str) -> Option<f64> {
    if !value.is_finite() {
        return None;
    }

    let celsius = match normalize_unit_name(from).as_str() {
        "celsius" => value,
        "fahrenheit" => (value - 32.0) * 5.0 / 9.0,
        "kelvin" => value - 273.15,
        _ => return None,
    };

    match normalize_unit_name(to).as_str() {
        "celsius" => Some(celsius),
        "fahrenheit" => Some(celsius * 9.0 / 5.0 + 32.0),
        "kelvin" => Some(celsius + 273.15),
        _ => None,
    }
}

fn find_unit(units: &[Unit], query: &str) -> Option<Unit> {
    let query = normalize_unit_name(query);

    units.iter().copied().find(|unit| {
        normalize_unit_name(unit.name) == query || normalize_unit_name(unit.symbol) == query
    })
}

fn normalize_unit_name(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close_to(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.000_001,
            "expected {actual} to be close to {expected}"
        );
    }

    #[test]
    fn exposes_converter_categories() {
        assert_eq!(CATEGORIES.len(), 12);
        assert_eq!(CATEGORIES[0].title(), "Volume");
        assert_eq!(CATEGORIES[11].title(), "Angle");
        assert_eq!(TEMPERATURE_UNITS, &["Celsius", "Fahrenheit", "Kelvin"]);
    }

    #[test]
    fn converts_linear_units_by_category() {
        close_to(
            convert(ConverterCategory::Length, 1.0, "mile", "foot").unwrap(),
            5_280.0,
        );
        close_to(
            convert(ConverterCategory::WeightMass, 1.0, "kg", "lb").unwrap(),
            2.204_622_621_848_775_7,
        );
        close_to(
            convert(ConverterCategory::Data, 1.0, "MiB", "B").unwrap(),
            1_048_576.0,
        );
    }

    #[test]
    fn converts_temperature_units() {
        close_to(
            convert(
                ConverterCategory::Temperature,
                100.0,
                "Celsius",
                "Fahrenheit",
            )
            .unwrap(),
            212.0,
        );
        close_to(
            convert_temperature(32.0, "Fahrenheit", "Kelvin").unwrap(),
            273.15,
        );
    }

    #[test]
    fn rejects_unknown_or_invalid_conversion_inputs() {
        assert_eq!(
            convert(ConverterCategory::Length, f64::NAN, "m", "cm"),
            None
        );
        assert_eq!(convert(ConverterCategory::Length, 1.0, "m", "bogus"), None);
    }
}
