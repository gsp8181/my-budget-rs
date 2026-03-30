use crate::generate_controller;

generate_controller!(Category::misc, Db_Name::debit, "name,amount");
