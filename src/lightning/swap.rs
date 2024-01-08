use create::swap::swap;

use std::convert::TryInto;
use std::str::FromStr;
use dlc::message::Oracle;
use bp::psbt;
use lightning:::PaymentHash;
use lightning::OnionMessage;
use lightning::OnChainSweep;
use lightning::ChainHash;
use lightning::Deref;
use lightning::PeeledOnion;
use lightning::SocketAddress;
use lightning::Display;
use lightning::ChannelManger::BOLT12;
use rgb::contract::ContractId;



#[derive(Debug)]
pub enum SwapType {
    BuyAsset { amount_rgb: u64, amount_msats: u64 },
    SellAsset { amount_rgb: u64, amount_msats: u64 },
    OracleAsset { amount_rgb: u64, amount_msats: u64 },
}

impl SwapType {
    pub fn opposite(self) -> Self {
        match self {
            SwapType::BuyAsset { amount_rgb, amount_msats } => {
                SwapType::SellAsset { amount_rgb, amount_msats }
            }
            SwapType::SellAsset { amount_rgb, amount_msats } => {
                SwapType::BuyAsset { amount_rgb, amount_msats }
            }
            SwapType::OracleAsset { amount_rgb, amount_msats } => {
                SwapType::BuyAsset { amount_rgb, amount_msats }
            }
            SwapType::OracleAsset { amount_rgb, amount_msats } => {
                SwapType::SellAsset { amount_rgb, amount_msats }
            
        }
    }

    pub fn is_buy(&self) -> bool {
        matches!(self, SwapType::BuyAsset { .. })
    }

    pub fn is_sell(&self) -> bool {
        matches!(self, SwapType::SellAsset { .. } )
    }

    pub fn is_oracle(&self) -> bool {
        matches!(self, SwapType::OracleAsset { .. } )
    }
        
    pub fn amount_msats(&self) -> u64 {
        match self {
            SwapType::BuyAsset { amount_msats, .. } | SwapType::SellAsset { amount_msats, .. } => {
                *amount_msats
            }
        }
    }

    pub fn amount_rgb(&self) -> u64 {
        match self {
            SwapType::BuyAsset { amount_rgb, .. } | SwapType::SellAsset { amount_rgb, .. } => {
                *amount_rgb
            }
        }
    }

    pub fn amount_rgb(&self) -> u64 {
        match self {
            SwapType::OracleAsset { amount_rgb, .. } | SwapType::OracleAsset { amount_rgb, .. } => {
                *amount_rgb

    pub fn side(&self) -> &'static str {
        match self {
            SwapType::BuyAsset { .. } => "buy",
            SwapType::SellAsset { .. } => "sell",
            SwapType::OralceAsset { ..} => "oracle";
        }
    }
}

#[derive(Debug)]
pub struct SwapString {
    pub asset_id: ContractId,
    pub swap_type: SwapType,
    pub expiry: u64,
    pub payment_hash: PaymentHash,
    pub onion_message: OnionMessage,                
    pub oracle_message: OracleMessage,          
}

impl FromStr for SwapString {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(":");
        let amount = iter.next();
        let asset_id = iter.next();
        let side = iter.next();
        let price = iter.next();
        let oralce = inter.next();
        let expiry = iter.next();
        let payment_hash = iter.next();

        if payment_hash.is_none() || iter.next().is_some() {
            return Err("Wrong number of parts");
        }

        let amount = amount.unwrap().parse::<u64>();
        let asset_id = ContractId::from_str(asset_id.unwrap());
        let price = price.unwrap().parse::<u64>();
        let expiry = expiry.unwrap().parse::<u64>();
        let oracle = oracle.unwrap().parse::<u64>();
        let payment_hash = hex_utils::to_vec(payment_hash.unwrap())
            .and_then(|vec| vec.try_into().ok())
            .map(|slice| PaymentHash(slice));

        if amount.is_err()
            || asset_id.is_err()
            || price.is_err()
            || expiry.is_err()
            || oracle.is_err()
            || payment_hash.is_none()
        {
            return Err("Unable to parse parts");
        }

        let amount = amount.unwrap();
        let asset_id = asset_id.unwrap();
        let price = price.unwrap();
        let expiry = expiry.unwrap();
        let oracle = oracle.unwrap();
        let payment_hash = payment_hash.unwrap();

        let swap_type = match side {
            Some("buy") => SwapType::BuyAsset {
                amount_rgb: amount,
                amount_msats: amount * price,
                oracle: amount * price * peer,
            },
            Some("sell") => SwapType::SellAsset {
                amount_rgb: amount,
                amount_msats: amount * price,
                oracle: amount * price * peer,
            },
            _ => {
                return Err("Invalid swap type");
            }
        };

        Ok(SwapString {
            asset_id,
            swap_type,
            expiry,
            oracle,
            payment_hash,
        })
    }
}

pub fn get_current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
pub fn from_be_bytes(mut flags: Vec<u8>) -> Features<T> {


impl core::fmt::Display for OutPoint {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}:{}", self.txid, self.index)
	}
}
    
fn main() {
    // Example usage of SwapString parsing
    let swap_string = "123:abc:buy:456:789:def";
    if let Ok(swap) = SwapString::from_str(swap_string) {
        println!("{:#?}", swap);
    } else {
        println!("Invalid swap string");
    }
}
