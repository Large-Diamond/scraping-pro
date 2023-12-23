#![allow(dead_code)]
use std::collections::HashSet;

#[derive(Debug)]
struct PokemonProduct {
	url: Option<String>,
	image: Option<String>,
	name: Option<String>,
	price: Option<String>,
}

fn single_page_scraper() {
	let response = reqwest::blocking::get("https://scrapeme.live/shop/");
	let html_content = response.unwrap().text().unwrap();
	let document = scraper::Html::parse_document(&html_content);
	let html_product_selector = scraper::Selector::parse("li.product").unwrap();
	let html_products = document.select(&html_product_selector);
	let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

	let a_selector = scraper::Selector::parse("a").unwrap();
	let img_selector = scraper::Selector::parse("img").unwrap();
	let h2_selector = scraper::Selector::parse("h2").unwrap();
	let price_selector = scraper::Selector::parse(".price").unwrap();
	for html_product in html_products {
		let url = html_product
			.select(&a_selector)
			.next()
			.and_then(|a| a.value().attr("href"))
			.map(str::to_owned);
		let image = html_product
			.select(&img_selector)
			.next()
			.and_then(|img| img.value().attr("src"))
			.map(str::to_owned);
		let name = html_product.select(&h2_selector).next().map(|h2| h2.text().collect::<String>());
		let price = html_product
			.select(&price_selector)
			.next()
			.map(|price| price.text().collect::<String>());

		let pokemon_product = PokemonProduct { url, image, name, price };
		pokemon_products.push(pokemon_product);
	}

	for pokemon_product in pokemon_products {
		println!("{:?}", pokemon_product);
	}
}

fn multi_page_scraper() {
	let mut pokemon_products: Vec<PokemonProduct> = Vec::new();
	let first_page = "https://scrapeme.live/shop/page/1/";
	let mut pages_to_scrape: Vec<String> = vec![first_page.to_owned()];
	let mut pages_discovered: HashSet<String> = HashSet::new();

	// current iteration
	let mut i = 1;
	// max number of iterations allowed
	let max_iterations = 5;

	while !pages_to_scrape.is_empty() && i <= max_iterations {
		// get the first element from the queue
		let page_to_scrape = pages_to_scrape.remove(0);

		// retrieve and parse the HTML document
		let response = reqwest::blocking::get(page_to_scrape);
		let html_content = response.unwrap().text().unwrap();
		let document = scraper::Html::parse_document(&html_content);

		let html_product_selector = scraper::Selector::parse("li.product").unwrap();
		let html_products = document.select(&html_product_selector);

		let a_selector = scraper::Selector::parse("a").unwrap();
		let img_selector = scraper::Selector::parse("img").unwrap();
		let h2_selector = scraper::Selector::parse("h2").unwrap();
		let price_selector = scraper::Selector::parse(".price").unwrap();
		for html_product in html_products {
			let url = html_product
				.select(&a_selector)
				.next()
				.and_then(|a| a.value().attr("href"))
				.map(str::to_owned);
			let image = html_product
				.select(&img_selector)
				.next()
				.and_then(|img| img.value().attr("src"))
				.map(str::to_owned);
			let name =
				html_product.select(&h2_selector).next().map(|h2| h2.text().collect::<String>());
			let price = html_product
				.select(&price_selector)
				.next()
				.map(|price| price.text().collect::<String>());

			let pokemon_product = PokemonProduct { url, image, name, price };
			pokemon_products.push(pokemon_product);
		}

		// get all pagination link elements
		let html_pagination_link_selector = scraper::Selector::parse("a.page-numbers").unwrap();
		let html_pagination_links = document.select(&html_pagination_link_selector);

		// iterate over them to find new pages to scrape
		for html_pagination_link in html_pagination_links {
			// get the pagination link URL
			let pagination_url = html_pagination_link.value().attr("href").unwrap().to_owned();

			// if the page discovered is new
			if !pages_discovered.contains(&pagination_url) {
				pages_discovered.insert(pagination_url.clone());

				// if the page discovered should be scraped
				if !pages_to_scrape.contains(&pagination_url) {
					pages_to_scrape.push(pagination_url.clone());
				}
			}
		}

		// increment the iteration counter
		i += 1;
	}

	for pokemon_product in pokemon_products {
		println!("{:?}", pokemon_product);
	}
}

fn headless_scraper() {
	let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

	let browser = headless_chrome::Browser::default().unwrap();
	let tab = browser.new_tab().unwrap();
	tab.navigate_to("https://scrapeme.live/shop/").unwrap();

	let html_products = tab.wait_for_elements("li.product").unwrap();

	for html_product in html_products {
		// scraping logic...
		let url = html_product
			.wait_for_element("a")
			.unwrap()
			.get_attributes()
			.unwrap()
			.unwrap()
			.get(1)
			.unwrap()
			.to_owned();
		// let image = html_product
		//     .wait_for_element("img")
		//     .unwrap()
		//     .get_attributes()
		//     .unwrap()
		//     .unwrap()
		//     .get(5)
		//     .unwrap()
		//     .to_owned();
		let name = html_product.wait_for_element("h2").unwrap().get_inner_text().unwrap();
		let price = html_product.wait_for_element(".price").unwrap().get_inner_text().unwrap();
		let pokemon_product =
			PokemonProduct { url: Some(url), image: None, name: Some(name), price: Some(price) };

		pokemon_products.push(pokemon_product);
	}

	for pokemon_product in pokemon_products {
		println!("{:?}", pokemon_product);
	}
}

fn main() {
	single_page_scraper();
	multi_page_scraper();
	headless_scraper();
}
