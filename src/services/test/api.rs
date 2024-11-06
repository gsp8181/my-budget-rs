use chrono::{DateTime, Local, TimeZone};
use rust_decimal_macros::dec;

use crate::{
    models::item::{Category, Db_Name, JsonObject},
    services::apiservice,
};

pub fn test_data() -> Vec<JsonObject> {
    let d1: JsonObject = JsonObject {
        id: Some(1),
        oldId: None,
        category: Category::bank,
        name: String::from("b1"),
        day: None,
        amount: dec!(2000.22),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d2 = JsonObject {
        id: Some(2),
        oldId: None,
        category: Category::bank,
        name: String::from("b2"),
        day: None,
        amount: dec!(100),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d3: JsonObject = JsonObject {
        id: Some(3),
        oldId: None,
        category: Category::bank,
        name: String::from("b3"),
        day: None,
        amount: dec!(25.51),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d4 = JsonObject {
        id: Some(4),
        oldId: None,
        category: Category::cash,
        name: String::from("c1"),
        day: None,
        amount: dec!(10.0),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d5 = JsonObject {
        id: Some(5),
        oldId: None,
        category: Category::cash,
        name: String::from("x2"),
        day: None,
        amount: dec!(6.20),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d6 = JsonObject {
        id: Some(6),
        oldId: None,
        category: Category::recurring,
        name: String::from("r1"),
        day: Some(22),
        amount: dec!(125.0),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d7 = JsonObject {
        id: Some(7),
        oldId: None,
        category: Category::creditcard,
        name: String::from("c1"),
        day: None,
        amount: dec!(202.63),
        cardid: Some(14),
        dbName: Db_Name::credit,
    };

    let d8 = JsonObject {
        id: Some(8),
        oldId: None,
        category: Category::creditcard,
        name: String::from("c2"),
        day: None,
        amount: dec!(139.0),
        cardid: Some(14),
        dbName: Db_Name::credit,
    };

    let d9 = JsonObject {
        id: Some(9),
        oldId: None,
        category: Category::creditcard,
        name: String::from("c3"),
        day: None,
        amount: dec!(104.0),
        cardid: Some(14),
        dbName: Db_Name::credit,
    };

    let d10 = JsonObject {
        id: Some(10),
        oldId: None,
        category: Category::creditcard,
        name: String::from("c4"),
        day: None,
        amount: dec!(25.30),
        cardid: Some(14),
        dbName: Db_Name::credit,
    };

    let d11 = JsonObject {
        id: Some(11),
        oldId: None,
        category: Category::creditcard,
        name: String::from("c5"),
        day: None,
        amount: dec!(200.27),
        cardid: Some(14),
        dbName: Db_Name::credit,
    };

    let d12 = JsonObject {
        id: Some(12),
        oldId: None,
        category: Category::debt,
        name: String::from("d1"),
        day: None,
        amount: dec!(92.0),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d13 = JsonObject {
        id: Some(13),
        oldId: None,
        category: Category::debt,
        name: String::from("d2"),
        day: None,
        amount: dec!(22.10),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d14 = JsonObject {
        id: Some(14),
        oldId: None,
        category: Category::cardbalance,
        name: String::from("cb1"),
        day: None,
        amount: dec!(1261.40),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d15 = JsonObject {
        id: Some(15),
        oldId: None,
        category: Category::cardbalance,
        name: String::from("cb2"),
        day: None,
        amount: dec!(176.02),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d16 = JsonObject {
        id: Some(16),
        oldId: None,
        category: Category::recurring,
        name: String::from("r1"),
        day: Some(20),
        amount: dec!(1.00),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d17 = JsonObject {
        id: Some(17),
        oldId: None,
        category: Category::recurring,
        name: String::from("r2"),
        day: Some(1),
        amount: dec!(6.00),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d18 = JsonObject {
        id: Some(18),
        oldId: None,
        category: Category::recurring,
        name: String::from("r3"),
        day: Some(30),
        amount: dec!(19.99),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d19 = JsonObject {
        id: Some(19),
        oldId: None,
        category: Category::recurring,
        name: String::from("r4"),
        day: Some(1),
        amount: dec!(40.00),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d20 = JsonObject {
        id: Some(20),
        oldId: None,
        category: Category::recurring,
        name: String::from("r5"),
        day: Some(4),
        amount: dec!(9.99),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d21 = JsonObject {
        id: Some(21),
        oldId: None,
        category: Category::recurring,
        name: String::from("r6"),
        day: Some(24),
        amount: dec!(3.50),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d22 = JsonObject {
        id: Some(22),
        oldId: None,
        category: Category::recurring,
        name: String::from("r7"),
        day: Some(26),
        amount: dec!(7.00),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d23 = JsonObject {
        id: Some(23),
        oldId: None,
        category: Category::recurring,
        name: String::from("r8"),
        day: Some(17),
        amount: dec!(20.00),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d24 = JsonObject {
        id: Some(24),
        oldId: None,
        category: Category::recurring,
        name: String::from("r9"),
        day: Some(7),
        amount: dec!(3.99),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d25 = JsonObject {
        id: Some(25),
        oldId: None,
        category: Category::recurring,
        name: String::from("r10"),
        day: Some(28),
        amount: dec!(5.99),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d26 = JsonObject {
        id: Some(26),
        oldId: None,
        category: Category::creditcard,
        name: String::from("rc1"),
        day: None,
        amount: dec!(20),
        cardid: Some(7),
        dbName: Db_Name::credit,
    };

    let d27 = JsonObject {
        id: Some(27),
        oldId: None,
        category: Category::debt,
        name: String::from("do1"),
        day: None,
        amount: dec!(30),
        cardid: None,
        dbName: Db_Name::debit,
    };

    let d28 = JsonObject {
        id: Some(28),
        oldId: None,
        category: Category::misc,
        name: String::from("mc1"),
        day: None,
        amount: dec!(300),
        cardid: None,
        dbName: Db_Name::credit,
    };

    let d29: JsonObject = JsonObject {
        id: Some(29),
        oldId: None,
        category: Category::misc,
        name: String::from("md1"),
        day: None,
        amount: dec!(250),
        cardid: None,
        dbName: Db_Name::debit,
    };

    vec![
        d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15, d16, d17, d18, d19, d20,
        d21, d22, d23, d24, d25, d26, d27, d28, d29,
    ]
}

#[test]
fn check_calculate_date_and_pay() {
    // debit 1 40
    // debit 1 6
    // debit 4 9.99
    // debit 7 3.99
    // debit 17 20.00
    // debit 20 1
    // credit 22 125
    // debit 24 3.50
    // debit 26 7.00
    // debit 28 5.99
    // debit 30 19.99

    // aggregate +7.54

    assert_eq!(
        dec!(1616.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 1 22:13:15.000 +0100"),
            dec!(0),
            25,
            dec!(0),
            true
        )
    );

    assert_eq!(
        dec!(1529.81),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 1 22:13:15.000 +0100"),
            dec!(0),
            3,
            dec!(0),
            true
        )
    );

    assert_eq!(
        dec!(1613.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 5 22:13:15.000 +0100"),
            dec!(0),
            30,
            dec!(0),
            true
        )
    );

    assert_eq!(
        dec!(1547.34),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 5 22:13:15.000 +0100"),
            dec!(0),
            3,
            dec!(0),
            true
        )
    );
    //assert!(dec!(920.31) == api::calculate(&test_data(), &dt, dec!(40), 25, dec!(25)));
}

#[test]
fn check_calculate_dr_ws() {
    assert_eq!(
        dec!(456.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 1 22:13:15.000 +0100"),
            dec!(40),
            25,
            dec!(40),
            true
        )
    );

    assert_eq!(
        dec!(1036.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 1 22:13:15.000 +0100"),
            dec!(20),
            25,
            dec!(40),
            true
        )
    );

    assert_eq!(
        dec!(1616.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 1 22:13:15.000 +0100"),
            dec!(0),
            25,
            dec!(40),
            true
        )
    );

    assert_eq!(
        dec!(471.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 2 22:13:15.000 +0100"),
            dec!(40),
            25,
            dec!(25),
            true
        )
    );

    assert_eq!(
        dec!(456.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 2 22:13:15.000 +0100"),
            dec!(40),
            25,
            dec!(40),
            true
        )
    );

    assert_eq!(
        dec!(446.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 2 22:13:15.000 +0100"),
            dec!(40),
            25,
            dec!(50),
            true
        )
    );

    assert_eq!(
        dec!(486.33),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 3 22:13:15.000 +0100"),
            dec!(40),
            25,
            dec!(25),
            true
        )
    );

    assert_eq!(
        dec!(666.32),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 6 22:13:15.000 +0100"),
            dec!(40),
            25,
            dec!(25),
            true
        )
    );

    assert_eq!(
        dec!(666.32),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 6 22:13:15.000 +0100"),
            dec!(40),
            25,
            dec!(50),
            true
        )
    );
}

#[test]
fn check_calculate_rollover_payday() {
    let vect: Vec<JsonObject> = vec![];
    assert_eq!(
        dec!(-1200),
        apiservice::calculate(
            &vect,
            &get_local_date("2024 Sep 2 22:13:15.000 +0100"),
            dec!(40),
            1,
            dec!(40),
            false
        )
    );

    assert_eq!(
        dec!(-2400),
        apiservice::calculate(
            &vect,
            &get_local_date("2024 Sep 2 22:13:15.000 +0100"),
            dec!(40),
            1,
            dec!(40),
            true
        )
    );

    //TODO: should this calculate the test data aswell? currently it does NOT
    assert_eq!(
        dec!(1547.34),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 5 22:13:15.000 +0100"),
            dec!(0),
            3,
            dec!(0),
            true
        )
    );

    assert_eq!(
        dec!(1547.34),
        apiservice::calculate(
            &test_data(),
            &get_local_date("2024 Sep 5 22:13:15.000 +0100"),
            dec!(0),
            3,
            dec!(0),
            false
        )
    );

    //TODO:test with data
}

#[test]
fn check_calculate_payday_oor() {
    let vect: Vec<JsonObject> = vec![];

    assert_eq!(
        dec!(-1160),
        apiservice::calculate(
            &vect,
            &get_local_date("2024 Sep 1 22:13:15.000 +0100"),
            dec!(40),
            31,
            dec!(40),
            true
        )
    );
}

#[test]
fn check_static_totals() {
    // static credits 3247.23
    // static debits 1717.42
    // static total 1529.81

    let mut amount = dec!(0);
    for data in test_data() {
        match data {
            JsonObject {
                dbName: Db_Name::debit,
                day: None,
                ..
            } => amount -= data.amount,
            JsonObject {
                dbName: Db_Name::credit,
                day: None,
                ..
            } => amount += data.amount,
            _ => {}
        }
    }
    assert_eq!(dec!(1529.81), amount);
}

fn get_local_date(dt_str: &str) -> DateTime<Local> {
    
    Local
        .from_local_datetime(
            &DateTime::parse_from_str(dt_str, "%Y %b %d %H:%M:%S%.3f %z")
                .unwrap()
                .naive_local(),
        )
        .unwrap()
}
