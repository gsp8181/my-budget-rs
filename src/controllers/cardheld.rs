use crate::generate_controller;

generate_controller!(Category::creditcard, Db_Name::credit, "name,amount,cardid");
