use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use reqwest::header;
use super::storage;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder().cookie_store(true).build().unwrap();
//    static ref URL_TRANS: Vec<(char, &'static str)> = vec![('/', "%2F"), (' ', "%20"), ('?', "%3F")];
    static ref QUERY_URL: &'static str = "https://linggle.com/query/";
    static ref BASE_URL: &'static str = "https://linggle.com/";
}

type Timestamp = i64;

#[derive(Deserialize, Serialize, Debug)]
pub struct LinggleRequest {
    pub query: String,
    pub time: Timestamp,
}

#[derive(Deserialize, Debug)]
pub struct NGram {
    pub text: String,
    pub count: usize,
}

#[derive(Deserialize, Debug)]
pub struct LinggleResult {
    pub query: String,
    pub time: Timestamp,
    pub total: usize,
    pub ngrams: Vec<NGram>,
}

#[derive(Deserialize, Debug)]
pub struct CSRF {
    pub csrf_token: String,
    #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    pub expires: DateTime<Utc>,
}


pub fn get_csrf() -> Result<CSRF, reqwest::Error> {
    let response = HTTP_CLIENT.get(BASE_URL.clone()).send()?;
    let cookies: Vec<_> = response.cookies().collect();
    Ok(CSRF {
        csrf_token: cookies[0].value().to_string(),
        expires: DateTime::from(cookies[0].expires().unwrap()),
    })
}

pub fn query(q: &str) -> Result<LinggleResult, reqwest::Error> {
    let params = LinggleRequest { query: q.to_owned(), time: Utc::now().timestamp_millis() };
    let mut res = HTTP_CLIENT.post(QUERY_URL.clone())
        .headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(header::CONTENT_TYPE,
                           header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap());
            headers.insert("X-CSRFToken",
                           {
                               let st = storage::get_csrf_token().unwrap().csrf_token;
                               header::HeaderValue::from_str(st.as_str()).unwrap()
                           });
            headers.insert("X-Requested-With", header::HeaderValue::from_str("XMLHttpRequest").unwrap());
            headers.insert(header::REFERER, header::HeaderValue::from_str(BASE_URL.clone()).unwrap());
            headers
        })
        .json(&params).send()?;
    Ok(res.json().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn over_five_words_fail_01() {
        // Note: the maximum number of words in a query is 5
        let res: LinggleResult = query("cut ?a bread but got nothing").unwrap();
        assert_eq!(res.total, 0);
        assert_eq!(res.ngrams.len(), 0);
        assert_eq!(res.query, "cut ?a bread but got nothing");
    }

    #[test]
    fn over_five_words_fail_02() {
        // Note: the maximum number of words in a query is 5
        let res: LinggleResult = query("There is nothing here _ _").unwrap();
        assert_eq!(res.total, 0);
        assert_eq!(res.ngrams.len(), 0);
        assert_eq!(res.query, "There is nothing here _ _");
    }

    #[test]
    fn over_five_words_fail_03() {
        // Note: the maximum number of words in a query is 5
        let res: LinggleResult = query("you are definitely a fancy/naughty guy").unwrap();
        assert_eq!(res.total, 0);
        assert_eq!(res.ngrams.len(), 0);
        assert_eq!(res.query, "you are definitely a fancy/naughty guy");
    }

    #[test]
    fn writing_ahead_success_01() {
        let res: LinggleResult = query("what _ day").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.query, "what _ day");
        assert_ne!(res.ngrams.len(), 0);
    }

    #[test]
    fn writing_ahead_success_02() {
        let res: LinggleResult = query("write * code").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.query, "write * code");
        assert_ne!(res.ngrams.len(), 0);
    }

    #[test]
    fn writing_ahead_success_03() {
        let res: LinggleResult = query("present a method _ _").unwrap();
        assert_ne!(res.total, 0);
        assert_ne!(res.ngrams.len(), 0);
        assert_eq!(res.query, "present a method _ _");
    }

    #[test]
    fn checking_word_needed_success_01() {
        let res: LinggleResult = query("go ?to ask your mom").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "go ask your mom");
        assert_ne!(res.ngrams[0].count, 0);
    }

    #[test]
    fn checking_word_needed_success_02() {
        let res: LinggleResult = query("cut ?a bread").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "cut bread");
        assert_ne!(res.ngrams[0].count, 0);
    }

    #[test]
    fn checking_word_needed_success_03() {
        let res: LinggleResult = query("discuss ?about the issue").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "discuss the issue");
        assert_eq!(res.ngrams[1].text, "discuss about the issue");
    }

    #[test]
    fn deciding_on_alternative_phrases_success_01() {
        let res: LinggleResult = query("not in the/a position to").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "not in a position to");
        assert_eq!(res.ngrams[1].text, "not in the position to");
    }

    #[test]
    fn deciding_on_alternative_phrases_success_02() {
        let res: LinggleResult = query("play an important role/part").unwrap();
        assert_ne!(res.total, 0);
        assert_eq!(res.ngrams[0].text, "play an important role");
        assert_eq!(res.ngrams[1].text, "play an important part");
    }
}
