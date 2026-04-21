use serde::{Deserialize, Serialize};

use crate::api::endpoints;
use std::collections::HashMap;

//{"balance": 0, "username": "x", "active": true, "name": "thomas eh", "signup_due_paid": true}
#[derive(Serialize, Deserialize, Debug)]
pub struct MemberInfo {
    pub balance: i32,
    pub username: String,
    pub active: bool,
    pub name: String,
    pub signup_due_paid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberBalance {
    pub balance: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberId {
    pub member_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sale {
    pub timestamp: String,
    pub product: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub name: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleRequest {
    pub member_id: i32,
    pub buystring: String,
    pub room: i32,
}

pub async fn get_member_id(
    api_url: &str,
    username: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}",
        api_url,
        endpoints::GET_MEMBER_ID_ENDPOINT.replace("{username}", username)
    );
    let resp = reqwest::get(url).await?.json::<MemberId>().await?;
    Ok(resp.member_id)
}

pub async fn get_member_balance(
    api_url: &str,
    member_id: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}{}",
        api_url,
        endpoints::GET_MEMBER_BALANCE_ENDPOINT
            .replace("{member_id}", member_id.to_string().as_str())
    );
    let resp = reqwest::get(url).await?.json::<MemberBalance>().await?;
    Ok(resp.balance)
}

pub async fn get_member_info(
    api_url: &str,
    member_id: &i32,
) -> Result<MemberInfo, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}{}",
        api_url,
        endpoints::GET_MEMBER_INFO_ENDPOINT.replace("{member_id}", member_id.to_string().as_str())
    );
    let resp = reqwest::get(url).await?.json::<MemberInfo>().await?;
    Ok(resp)
}

pub async fn get_member_history(
    api_url: &str,
    member_id: &i32,
) -> Result<Vec<Sale>, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}{}",
        api_url,
        endpoints::GET_MEMBER_HISTORY_ENDPOINT
            .replace("{member_id}", member_id.to_string().as_str())
    );
    let resp = reqwest::get(url).await?.json::<Vec<Sale>>().await?;
    Ok(resp)
}

pub async fn get_named_products(
    api_url: &str,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let url: String = format!("{}{}", api_url, endpoints::GET_NAMED_PRODUCTS_ENDPOINT);
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, i32>>()
        .await?;
    Ok(resp)
}

pub async fn get_active_products(
    api_url: &str,
    room_id: i32,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}{}",
        api_url,
        endpoints::GET_ACTIVE_PRODUCTS_ENDPOINT.replace("{room_id}", room_id.to_string().as_str())
    );
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, i32>>()
        .await?;
    Ok(resp)
}

pub async fn post_sale(
    api_url: &str,
    sale_request: SaleRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let url: String = format!("{}{}", api_url, endpoints::SALES_ENDPOINT);
    let client = reqwest::Client::new();
    let resp = client.post(url).json(&sale_request).send().await?;
    // TODO: Handle response, check for errors, etc.
    Ok(())
}
