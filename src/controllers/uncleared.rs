use crate::generate_controller;

generate_controller!(Category::creditcard, Db_Name::debit, "name,amount,cardid");
