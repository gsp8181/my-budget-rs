use crate::generate_controller;

generate_controller!(Category::recurring, Db_Name::debit, "name,amount,day");
