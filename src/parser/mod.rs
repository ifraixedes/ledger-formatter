use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/ledger.pest"]
pub struct LedgerParser;

// pub fn parse_file<P: AsRef<path::Path>>(path: P) {
pub fn parse(data: String) {
    let res = LedgerParser::parse(Rule::journal, &data);
    match res {
        Ok(pair) => println!("pair {}", pair),
        Err(error) => println!("error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_account_name() {
        // TODO: use assertion to check that it doesn't return emtpy
        // because without using SOI and EOI there is no error if it doesn't
        // parse
        LedgerParser::parse(Rule::account_name_check, "assets").expect("error parsing");
        LedgerParser::parse(Rule::account_name_check, "assets:cash").expect("error parsing");
        LedgerParser::parse(Rule::account_name_check, "assets:wallet cash").expect("error parsing");
        LedgerParser::parse(Rule::account_name_check, "(assets:wallet cash)")
            .expect("error parsing");
        // Error cases
        // LedgerParser::parse(Rule::account_check, "assets:wallet cash ").expect("error parsing");
        //LedgerParser::parse(Rule::account_check, " assets").expect("error parsing");
    }

    #[test]
    fn rule_comment_line() {
        LedgerParser::parse(Rule::comment_line_check, "# hey you").expect("error parsing");
        LedgerParser::parse(Rule::comment_line_check, ";heyyou ").expect("error parsing");
        LedgerParser::parse(Rule::comment_line_check, "%wow wow").expect("error parsing");
        LedgerParser::parse(Rule::comment_line_check, "| hey you ").expect("error parsing");
        LedgerParser::parse(Rule::comment_line_check, "*    `hey you`    ").expect("error parsing");
    }

    #[test]
    fn rule_date() {
        LedgerParser::parse(Rule::date_check, "2").expect("error parsing");
        // TODO: continue checking month, the next test fails
        // LedgerParser::parse(Rule::date_check, "10").expect("error parsing");
    }

    // TODO: follow with these tests when implementing parse
    fn read_ledger_data_file() -> String {
        std::fs::read_to_string("src/parser/ledger-example.dat").expect("error reading")
    }

    #[test]
    fn check_pest_syntax() {
        parse(read_ledger_data_file())
    }
}
