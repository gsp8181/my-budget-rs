use crate::generate_controller;

generate_controller!(Category::debt, Db_Name::credit, "name,amount");
