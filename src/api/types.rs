use serde::{Deserialize, Serialize};

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

//{"11": {"name": "Stregdollar", "price": 100}, "14": {"name": "Øl(Grøn, Classic, <b>Gulddamer</b>) (excl. pant)", "price": 750}, "16": {"name": "Cocio <b>(Mellem)</b>", "price": 1818}, "31": {"name": "Pant B (plastflasker under 1 liter)", "price": 150}, "32": {"name": "Voksenvand (1 kop)", "price": 150}, "42": {"name": "Limfjordsporter (excl. pant)", "price": 1000}, "44": {"name": "Søm", "price": 300}, "51": {"name": "<h1>Futtetyr</h1>", "price": 33300}, "54": {"name": "Mambo No. 8", "price": 800}, "1836": {"name": "<h1>FLAN billet</h1>", "price": 4269}, "1837": {"name": "Life potion (excl. pant)", "price": 1316}, "1839": {"name": "Kildevæld uden pant (excl. brus)\"", "price": 706}, "1848": {"name": "Pant A (glasflasker og metaldåser under 1 liter)", "price": 100}, "1858": {"name": "Grimbergen (excl. pant)", "price": 1900}, "1877": {"name": "Somersby (incl. pant)", "price": 1540}, "1879": {"name": "Apache attack helicopter", "price": 1280}, "1882": {"name": "Kakao (0,46 kop)", "price": 69}, "1891": {"name": "Sportycola (excl. pant)", "price": 600}, "1893": {"name": "Kinley (excl. pant)", "price": 680}, "1901": {"name": "Bober Øl (excl. pant x2)", "price": 650}, "1903": {"name": "<h3>Månedens Øl</h3><br>September: Bonk Beer", "price": 1460}, "1904": {"name": "Tebrev (Partibestemt smag)", "price": 90}, "1905": {"name": "Ramløse/ vand med brus (excl. pant)", "price": 808}, "1912": {"name": "Juice (Solita, Rynkeby)", "price": 1131}, "1915": {"name": "Faxe Kondi Pro", "price": 1600}}
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

// {"status":200,"msg":"OK","values":{"order":{"room":10,"member":321,"created_on":"2024-05-12T18:26:09.508Z","items":[123,123,123]},"promille":0.2,"is_ballmer_peaking":false,"bp_minutes":null,"bp_seconds":null,"caffeine":2,"cups":4,"product_contains_caffeine":true,"is_coffee_master":false,"cost":1800,"give_multibuy_hint":true,"sale_hints":"<span class=\"username\">kresten</span> 123:3","member_has_low_balance":false,"member_balance":182}}
#[derive(Serialize, Deserialize, Debug)]
pub struct SaleResponse {
    pub status: i32,
    pub msg: String,
    pub values: Option<SaleResponseValues>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleResponseValues {
    pub order: SaleResponseOrder,
    pub promille: f32,
    pub is_ballmer_peaking: bool,
    pub bp_minutes: Option<i32>,
    pub bp_seconds: Option<i32>,
    pub caffeine: f32,
    pub cups: i32,
    pub product_contains_caffeine: bool,
    pub is_coffee_master: bool,
    pub cost: i32,
    pub give_multibuy_hint: bool,
    pub sale_hints: Option<String>,
    pub member_has_low_balance: bool,
    pub member_balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleResponseOrder {
    pub room: i32,
    pub member: i32,
    pub created_on: String,
    pub items: Vec<i32>,
}
