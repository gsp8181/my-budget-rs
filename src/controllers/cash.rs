use crate::generate_controller;

generate_controller!(Category::cash, Db_Name::credit, "name,amount");
