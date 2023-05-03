use std::collections::HashMap;
use std::fmt::{Debug};

use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Vehicle {
    url: String,
    title: String,
    price: u16,
    miles: u32,
    registration_year: String,
    data_layer: DataLayer,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    products: Vec<Vehicle>,
    page: Page,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Page {
    current_page: u16,
    last_page: u16,
    total_count: u16,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct DataLayer {
    product: Product,
    dealer: Dealer,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Product {
    name: String,
    series: String,
    nameplate: String,
    bodystyle: String,
    engine: String,
    fuel_type: String,
    color: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct Dealer {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let initial = get_vehicles(None).await?;

    let mut all_vehicles: Vec<Vehicle> = vec![dummy_vehicle(); initial.page.total_count as usize];

    let mut n = 1;
    let size = initial.products.len();

    populate_vehicles(initial.products, &mut all_vehicles, initial.page.current_page, size);

    while n < initial.page.last_page {
        n = n + 1;
        let next = get_vehicles(Option::Some(n)).await?;
        populate_vehicles(next.products, &mut all_vehicles, n, size);
    }

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path("out.csv")
        .unwrap();

    writer.write_record(["url", "title", "price", "miles", "registration_year", "name", "series", "nameplate", "bodystyle", "engine", "fuel_type", "colour", "dealer_name"]).unwrap();

    for vehicle in all_vehicles {
        writer.serialize(vehicle).unwrap();
    }

    Ok(())

}

fn dummy_vehicle() -> Vehicle {
    let vec_veh = Vehicle {
        url: String::new(),
        title: String::new(),
        price: 0,
        miles: 0,
        registration_year: String::new(),
        data_layer: DataLayer {
            dealer: Dealer {
                name: String::new()
            },
            product: Product {
                name: String::new(),
                series: String::new(),
                nameplate: String::new(),
                bodystyle: String::new(),
                engine: String::new(),
                fuel_type: String::new(),
                color: String::new(),
            },
        },
    };
    vec_veh
}

fn populate_vehicles(api_vehicles: Vec<Vehicle>, all_vehicles: &mut Vec<Vehicle>, page: u16, page_size: usize) {
    let mut n = (page as usize - 1) * page_size;

    for vehicle in api_vehicles {
        all_vehicles[n] = vehicle;
        n = n + 1;
    }
}


async fn get_vehicles(page: Option<u16>) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let params = [
        ("step", String::from("carFilter")),
        ("model_category[]", String::from("127525")),
        ("fuel_type[]", String::from("30664")),
        ("au_model_year[0][0]", String::from("2021")),
        ("badge_engine_cc[]", String::from("2554")),
        ("p", page.unwrap_or(1).to_string())
    ];


    let wrap = client.post("https://used.jaguar.co.uk/")
        .header(ACCEPT, "application/json")
        .header(USER_AGENT, "curl/7.87.0")
        .header("X-Requested-With", "XMLHttpRequest")
        .form(&params)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;


    let resp: ApiResponse = serde_json::from_str(&wrap.get("products").get_or_insert(&String::from("[]"))).unwrap();

    Ok(resp)

}
