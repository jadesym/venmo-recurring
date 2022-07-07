use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Error;
use ureq::Error as UreqError;

const VENMO_BASE_URL: &str = "https://api.venmo.com/v1";
const VENMO_PAYMENTS_PATH: &str = "/payments";
const PRIVATE_AUDIENCE_PARAM_VALUE: &str = "private";

#[derive(PartialEq, Deserialize)]
struct VenmoPaymentTarget {
    // TODO | figure out how to deserialize into some form of enum string type
    #[serde(rename = "type")]
    kind: String,
    phone: Option<String>,
    email: Option<String>,
    user: Option<String>,
}

#[derive(PartialEq, Deserialize)]
struct VenmoPaymentActor {
    id: String,
}

#[derive(PartialEq, Deserialize)]
struct VenmoPayment {
    id: String,
    status: String,
    action: String,
}

#[derive(PartialEq, Deserialize)]
struct VenmoChargeCreateResponseData {
    balance: f32,
    payment: VenmoPayment,
}

#[derive(PartialEq, Deserialize)]
#[serde(untagged)]
enum VenmoChargeCreateResponse {
    DataResponse {
        data: VenmoChargeCreateResponseData,
    },
    ErrorResponse {
        error: VenmoChargeCreateResponseError,
    },
}

impl fmt::Display for VenmoChargeCreateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VenmoChargeCreateError::JsonParse(ref e) => {
                write!(f, "Error parsing JSON: {:?}", e)
            }
            VenmoChargeCreateError::CallResponse(ref e) => {
                write!(f, "Error from response: {:?}", e)
            }
            VenmoChargeCreateError::WithinResponse(ref e) => write!(
                f,
                "Error within response with code [{:?}] and message: {:?}",
                e.code, e.message
            ),
        }
    }
}

impl From<Error> for VenmoChargeCreateError {
    fn from(e: Error) -> Self {
        VenmoChargeCreateError::JsonParse(e)
    }
}

impl From<UreqError> for VenmoChargeCreateError {
    fn from(e: UreqError) -> Self {
        VenmoChargeCreateError::CallResponse(e)
    }
}

#[derive(Serialize)]
struct VenmoChargeCreateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,

    amount: f64,

    access_token: String,

    note: String,

    audience: String,
}

fn create_venmo_charge_create_params(
    access_token: String,
    charge_amount_in_cents: u32,
    venmo_unique_id: VenmoPaymentTargetUniqueId,
    note_text: String,
) -> VenmoChargeCreateParameters {
    let amount: f64 = f64::from(charge_amount_in_cents) / 100f64;
    let audience: String = String::from(PRIVATE_AUDIENCE_PARAM_VALUE);

    match venmo_unique_id {
        VenmoPaymentTargetUniqueId::PhoneNumber(phone_number) => VenmoChargeCreateParameters {
            phone: Some(phone_number),
            email: None,
            user_id: None,
            amount,
            access_token,
            note: note_text,
            audience,
        },
        VenmoPaymentTargetUniqueId::Email(email) => VenmoChargeCreateParameters {
            phone: None,
            email: Some(email),
            user_id: None,
            amount,
            access_token,
            note: note_text,
            audience,
        },
        VenmoPaymentTargetUniqueId::UserId(user_id) => VenmoChargeCreateParameters {
            phone: None,
            email: None,
            user_id: Some(user_id),
            amount,
            access_token,
            note: note_text,
            audience,
        },
    }
}

fn execute_payment_charge_request(
    charge_amount_in_cents: u32,
    venmo_unique_id: VenmoPaymentTargetUniqueId,
    note_text: String,
) -> Result<VenmoChargeCreateResponseData, VenmoChargeCreateError> {
    let params: VenmoChargeCreateParameters = create_venmo_charge_create_params(
        String::from("<TO_BE_REPLACED_ACCESS_TOKEN>"),
        charge_amount_in_cents,
        venmo_unique_id,
        note_text,
    );

    let mut url: String = VENMO_BASE_URL.to_owned();
    url.push_str(VENMO_PAYMENTS_PATH);
    let url: &str = url.as_str();

    let response = ureq::post(url).send_json(serde_json::json!(params))?;

    let parsed_json_response: VenmoChargeCreateResponse = response.into_json()?;

    match parsed_json_response {
        VenmoChargeCreateResponse::DataResponse { data } => Ok(data),
        VenmoChargeCreateResponse::ErrorResponse { error } => {
            Err(VenmoChargeCreateError::WithinResponse(error))
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct VenmoChargeCreateResponseError {
    message: String,
    code: String,
}

#[derive(Debug)]
pub enum VenmoChargeCreateError {
    CallResponse(ureq::Error),
    JsonParse(std::io::Error),
    WithinResponse(VenmoChargeCreateResponseError),
}

#[derive(Debug)]
pub struct VenmoCharge {
    pub charge_amount_in_cents: u32,
    pub target_unique_id: VenmoPaymentTargetUniqueId,
    pub note_text: String,
}

#[derive(Debug, PartialEq)]
pub enum VenmoPaymentTargetUniqueId {
    #[allow(dead_code)]
    PhoneNumber(String),
    #[allow(dead_code)]
    Email(String),
    UserId(String),
}

pub fn create_venmo_payment_charge(venmo_charge: VenmoCharge) -> Option<VenmoChargeCreateError> {
    let VenmoCharge {
        charge_amount_in_cents,
        target_unique_id,
        note_text,
    } = venmo_charge;
    execute_payment_charge_request(charge_amount_in_cents, target_unique_id, note_text).err()
}
