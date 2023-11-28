use reqwest::Error;
use scraper::{ElementRef, Html};
use structopt::StructOpt;
use tabled::{Table, Tabled};

#[derive(Debug, StructOpt)]
#[structopt(name = "league-url-input", about = "CLI based football league scraper")]
struct Args {
    #[structopt(short, long)]
    league_table_url: String,
}

#[derive(Tabled)]
struct LeagueData {
    position: i8,
    team: String,
    played: i8,
    home_wins: i8,
    home_draws: i8,
    home_losses: i8,
    away_wins: i8,
    away_draws: i8,
    away_losses: i8,
    wins: i8,
    draws: i8,
    losses: i8,
    goals_for: i8,
    goals_against: i8,
    goal_difference: i8,
    points: i8,
}

fn main() {
    let args = Args::from_args();

    let target_url: String = String::from(args.league_table_url);

    let scrape_result: String = match do_throttled_request(&target_url) {
        Ok(v) => v,
        Err(_) => return,
    };

    let parsed_html = parse_html(&scrape_result);

    let raw_league_table_data = extract_league_table_data(parsed_html);

    let league_data: Vec<LeagueData> = raw_league_table_data
        .iter()
        .skip(1)
        .map(|raw_league_data| {
            let league_data: LeagueData = LeagueData {
                position: raw_league_data.get(0).unwrap().parse::<i8>().expect("Failed to parse position"),
                team: raw_league_data.get(1).unwrap().to_string(),
                played: raw_league_data.get(2).unwrap().parse::<i8>().expect("Failed to parse position"),
                home_wins: raw_league_data.get(3).unwrap().parse::<i8>().expect("Failed to parse position"),
                home_draws: raw_league_data.get(4).unwrap().parse::<i8>().expect("Failed to parse position"),
                home_losses: raw_league_data.get(5).unwrap().parse::<i8>().expect("Failed to parse position"),
                away_wins: raw_league_data.get(6).unwrap().parse::<i8>().expect("Failed to parse position"),
                away_draws: raw_league_data.get(7).unwrap().parse::<i8>().expect("Failed to parse position"),
                away_losses: raw_league_data.get(8).unwrap().parse::<i8>().expect("Failed to parse position"),
                wins: raw_league_data.get(9).unwrap().parse::<i8>().expect("Failed to parse position"),
                draws: raw_league_data.get(10).unwrap().parse::<i8>().expect("Failed to parse position"),
                losses: raw_league_data.get(11).unwrap().parse::<i8>().expect("Failed to parse position"),
                goals_for: raw_league_data.get(12).unwrap().parse::<i8>().expect("Failed to parse position"),
                goals_against: raw_league_data.get(13).unwrap().parse::<i8>().expect("Failed to parse position"),
                goal_difference: raw_league_data.get(14).unwrap().parse::<i8>().expect("Failed to parse position"),
                points: raw_league_data.get(15).unwrap().parse::<i8>().expect("Failed to parse position"),
            };
            league_data
        })
        .collect();

    let table = Table::new(league_data).to_string();
    println!("{:#^10}", table);
}

fn do_throttled_request(url: &str) -> Result<String, Error> {
    let response = reqwest::blocking::get(url)?;
    response.text()
}

fn parse_html(html_payload: &str) -> Html {
    Html::parse_document(html_payload)
}

fn extract_league_table_data(parsed_html: Html) -> Vec<Vec<String>> {
    let page_table_selector = scraper::Selector::parse("div.o-table-responsive").unwrap();
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let league_table = parsed_html.select(&page_table_selector).next().expect("Table not found");

    let raw_league_table_data: Vec<Vec<String>> = league_table
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
    raw_league_table_data
}
