use crate::generate_controller;

generate_controller!(Category::debt, Db_Name::debit, "name,amount");
