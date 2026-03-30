use crate::generate_controller;

generate_controller!(Category::cardbalance, Db_Name::debit, "name,amount");
