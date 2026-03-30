use crate::generate_controller;

generate_controller!(Category::recurring, Db_Name::credit, "name,amount,day");
