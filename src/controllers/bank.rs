use crate::generate_controller;

generate_controller!(Category::bank, Db_Name::credit, "name,amount");
