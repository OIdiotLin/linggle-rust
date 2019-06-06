use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
//    static ref URL_TRANS: Vec<(char, &'static str)> = vec![('/', "%2F"), (' ', "%20"), ('?', "%3F")];
    static ref QUERY_URL: &'static str = "https://linggle.com/query/";
}

//macro_rules! linggle {
//    ($x:expr) => {format!("https://linggle.com/query/{}", $x.as_str())};
//}

#[derive(Deserialize, Serialize, Debug)]
pub struct LinggleRequest {
    query: String,
    time: u128,
}

#[derive(Deserialize, Debug)]
pub struct NGram {
    text: String,
    count: usize,
}

#[derive(Deserialize, Debug)]
pub struct LinggleResult {
    query: String,
    time: u128,
    total: usize,
    ngrams: Vec<NGram>,
}

fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

pub fn query(q: &str) -> Result<LinggleResult, reqwest::Error> {
    let params = LinggleRequest { query: q.to_owned(), time: timestamp() };
    let mut res = HTTP_CLIENT.post(QUERY_URL.clone()).form(&params).send()?;
    let res = res.json()?;
    Ok(res)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn over_five_words_fail() {
        // Note: the maximum number of words in a query is 5
        let res: LinggleResult = query("cut ?a bread but got nothing").unwrap();
        assert_eq!(res.total, 0);
        assert_eq!(res.ngrams.len(), 0);
        assert_eq!(res.query, "cut ?a bread but got nothing");

        // Note: the maximum number of words in a query is 5
        let res: LinggleResult = query("There is nothing here _ _").unwrap();
        assert_eq!(res.total, 0);
        assert_eq!(res.ngrams.len(), 0);
        assert_eq!(res.query, "There is nothing here _ _");

        // Note: the maximum number of words in a query is 5
        let res: LinggleResult = query("you are definitely a fancy/naughty guy").unwrap();
        assert_eq!(res.total, 0);
        assert_eq!(res.ngrams.len(), 0);
        assert_eq!(res.query, "you are definitely a fancy/naughty guy");
    }

    #[test]
    fn writing_ahead() {
        let res: LinggleResult = query("what _ day").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.query, "what _ day");
        assert_ne!(res.ngrams.len(), 0);

        let res: LinggleResult = query("write * code").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.query, "write * code");
        assert_ne!(res.ngrams.len(), 0);

        let res: LinggleResult = query("present a method _ _").unwrap();
        assert_ne!(res.total, 0);
        assert_ne!(res.ngrams.len(), 0);
        assert_eq!(res.query, "present a method _ _");
    }

    #[test]
    fn checking_word_needed() {
        let res: LinggleResult = query("go ?to ask your mom").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "go ask your mom");
        assert_ne!(res.ngrams[0].count, 0);

        let res: LinggleResult = query("cut ?a bread").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "cut bread");
        assert_ne!(res.ngrams[0].count, 0);

        let res: LinggleResult = query("discuss ?about the issue").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "discuss the issue");
        assert_eq!(res.ngrams[1].text, "discuss about the issue");
    }

    #[test]
    fn deciding_on_alternative_phrases() {
        let res: LinggleResult = query("not in the/a position to").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "not in a position to");
        assert_eq!(res.ngrams[1].text, "not in the position to");

        let res: LinggleResult = query("play an important role/part").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "play an important role");
        assert_eq!(res.ngrams[1].text, "play an important part");
    }
}
