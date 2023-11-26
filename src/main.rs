use reqwest::Error;
use scraper::ElementRef;

fn do_throttled_request(url: &str) -> Result<String, Error> {
    let response = reqwest::blocking::get(url)?;
    response.text()
}

fn parse_html(html_payload: &str) -> scraper::Html {
    scraper::Html::parse_document(html_payload)
}

fn main() {
    let target_url: String =
        String::from("https://www.thenationalleague.org.uk/match-info/tables?table_id=d-9900645");

    let scrape_result: String = match do_throttled_request(&target_url) {
        Ok(v) => v,
        Err(_) => return,
    };

    let parsed_html = parse_html(&scrape_result);

    let page_table_selector = scraper::Selector::parse("div.o-table-responsive").unwrap();
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let league_table = parsed_html.select(&page_table_selector).next().unwrap();

    let league_table_data: Vec<Vec<String>> = league_table
        .select(&tr_selector)
        .map(|td_element: ElementRef| {
            td_element
                .select(&td_selector)
                .map(|inner_value: ElementRef| {
                    inner_value
                        .text()
                        .map(|formatted_string: &str| {
                            formatted_string.trim().replace("\n", "").replace(".", "")
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    dbg!(league_table_data.get(0).unwrap());
}
