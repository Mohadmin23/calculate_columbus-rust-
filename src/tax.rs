pub fn federal_tax(income: f64) -> f64 {
    if income <= 11925.0 {
        income * 0.10
    } else if income <= 48475.0 {
        11925.0 * 0.10 + (income - 11925.0) * 0.12
    } else if income <= 103350.0 {
        11925.0 * 0.10 + (48475.0 - 11925.0) * 0.12 + (income - 48475.0) * 0.22
    } else if income <= 197300.0 {
        11925.0 * 0.10
            + (48475.0 - 11925.0) * 0.12
            + (103350.0 - 48475.0) * 0.22
            + (income - 103350.0) * 0.24
    } else if income <= 250525.0 {
        11925.0 * 0.10
            + (48475.0 - 11925.0) * 0.12
            + (103350.0 - 48475.0) * 0.22
            + (197300.0 - 103350.0) * 0.24
            + (income - 197300.0) * 0.32
    } else if income <= 626350.0 {
        11925.0 * 0.10
            + (48475.0 - 11925.0) * 0.12
            + (103350.0 - 48475.0) * 0.22
            + (197300.0 - 103350.0) * 0.24
            + (250525.0 - 197300.0) * 0.32
            + (income - 250525.0) * 0.35
    } else {
        11925.0 * 0.10
            + (48475.0 - 11925.0) * 0.12
            + (103350.0 - 48475.0) * 0.22
            + (197300.0 - 103350.0) * 0.24
            + (250525.0 - 197300.0) * 0.32
            + (626350.0 - 250525.0) * 0.35
            + (income - 626350.0) * 0.37
    }
}

pub fn ohio_state_tax(income: f64) -> f64 {
    if income <= 26050.0 {
        0.0
    } else if income <= 100000.0 {
        income * 0.0275
    } else {
        income * 0.03125
    }
}

pub fn columbus_city_tax(income: f64) -> f64 {
    income * 0.025
}

pub fn total_tax(income: f64) -> (f64, f64, f64, f64, f64) {
    let federal = federal_tax(income);
    let state = ohio_state_tax(income);
    let city = columbus_city_tax(income);
    let total = federal + state + city;
    let net_income = income - total;
    (federal, state, city, total, net_income)
}
