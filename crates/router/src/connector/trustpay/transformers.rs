
use masking::PeekInterface;
use error_stack::{IntoReport, ResultExt};
use serde::{Deserialize, Serialize};
use crate::{core::errors,types::{self,api, storage::enums}};
use serde_repr::{Serialize_repr, Deserialize_repr};
//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Eq, PartialEq)]
pub struct TrustpayPaymentsRequest {
    pub amount: i64,
    pub currency: enums::Currency,
    pub pan: String,
    pub cvv: i64,
    pub exp: String,
    pub redirectUrl: String,
}

impl TryFrom<&types::PaymentsAuthorizeRouterData> for TrustpayPaymentsRequest  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &types::PaymentsAuthorizeRouterData) -> Result<Self,Self::Error> {
        let amount = item.request.amount;
        let currency = item.request.currency;
        let payment_method = match item.request.payment_method_data.clone() {
            api::PaymentMethod::Card(ccard) => {
                ccard
            },
            _ => Err(errors::ConnectorError::NotImplemented(
                "Unknown payment method Type".to_string(),
            ))?,
        };
        let exp = format!("{}/{}",payment_method.card_exp_month.peek(),payment_method.card_exp_year.peek());
        println!("{}", exp);
        let redirectUrl = item.return_url.clone().unwrap_or("https://test-tpgw.trustpay.eu".to_string());
        let req = Self {
            amount,
            pan: payment_method.card_number.peek().to_string(),
            exp,
            currency,
            redirectUrl,
            cvv: payment_method.card_cvc.peek().parse().into_report().change_context(errors::ConnectorError::RequestEncodingFailed)?,
        };
        println!("requestself{:?}",req);
        Ok(req)

    }
}

pub struct TrustpayAuthType {
    pub(super)    api_key: String
}

impl TryFrom<&types::ConnectorAuthType> for TrustpayAuthType  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        if let types::ConnectorAuthType::HeaderKey { api_key } = _auth_type {
            Ok(Self {
                api_key: api_key.to_string(),
            })
        } else {
            Err(errors::ConnectorError::FailedToObtainAuthType.into())
        }
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr , PartialEq, Eq, Clone)]
#[repr(i32)]
pub enum TrustpayPaymentStatus {
    Success=0,
    Pending=1,
    Expired= (-1),
    Error=-2,
    ServerCallFailed=-3,
    AbortedByUser=-4,
    Failure=-255,
}

impl Default for TrustpayPaymentStatus {
    fn default() -> Self {
        TrustpayPaymentStatus::Failure
    }
}

impl From<TrustpayPaymentStatus> for enums::AttemptStatus {
    fn from(item: TrustpayPaymentStatus) -> Self {
        match item {
            TrustpayPaymentStatus::Success => Self::Charged,
            TrustpayPaymentStatus::Pending => Self::Pending,
            TrustpayPaymentStatus::Error => Self::Failure,
            TrustpayPaymentStatus::Failure => Self::Failure,
            TrustpayPaymentStatus::ServerCallFailed => Self::Failure,
            TrustpayPaymentStatus::Expired => Self::AuthorizationFailed,
            TrustpayPaymentStatus::AbortedByUser => Self::AuthorizationFailed,

        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrustpayPaymentsResponse {
    pub status: TrustpayPaymentStatus,
    pub description: String,
    pub instanceId: String,
    pub paymentStatus: String,
}

impl<F,T> TryFrom<types::ResponseRouterData<F, TrustpayPaymentsResponse, T, types::PaymentsResponseData>> for types::RouterData<F, T, types::PaymentsResponseData> {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: types::ResponseRouterData<F, TrustpayPaymentsResponse, T, types::PaymentsResponseData>) -> Result<Self,Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.status),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.instanceId),
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

#[derive(Default, Debug, Serialize)]
pub struct TrustpayRefundRequest {
    amount : i64,
    instance_id : String,
    currency : enums::Currency,
}

impl<F> TryFrom<&types::RefundsRouterData<F>> for TrustpayRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: &types::RefundsRouterData<F>) -> Result<Self,Self::Error> {
       let auth_type = TrustpayAuthType::try_from(&item.connector_auth_type);
        let amount = item.request.amount;
        let currency = item.request.currency;
        let instance_id = item.request.connector_transaction_id.clone();
        let req = Self {
            amount,
            currency,
            instance_id,
        };
        println!("requestself{:?}",req);
        Ok(req)
    }
}


#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]

pub enum RefundStatus {
    Success=0,
    Pending=1,
    #[default]
    Failure
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Success => Self::Success,
            RefundStatus::Pending => Self::Pending,
            RefundStatus::Failure => Self::Failure
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    status : RefundStatus,
    description: Option<String>,
    instance_id : Option<String>,
    payment_status :Option <String>,
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            response: Ok(types::RefundsResponseData {
                connector_refund_id: item.response.instance_id.ok_or(errors::ConnectorError::MissingRequiredField { field_name: "instanceId" })?,
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>> for types::RefundsRouterData<api::RSync>
{
     type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: types::RefundsResponseRouterData<api::RSync, RefundResponse>) -> Result<Self,Self::Error> {
         todo!()
     }
 }

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TrustpayErrorResponse {
    pub status: i32,
    pub description : String,
    pub errors: Vec<ErrorType>
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ErrorType {
    pub code : i32,
    pub description : String,
}




#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TrustPaySyncResponse {
    status : TrustpayPaymentStatus,
    instance_id : String,
    created : String,
    amount : String,
    currency : String,
    reference : Option<String>,
    payment_status :Option <String>,
    payment_status_details : Option<PaymentStatusD>,
    three_dsecure : Option<ThreeDS>,
    card : Option<Card> ,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TrustPayGenericResponse {
    status : TrustpayPaymentStatus,
    description: Option<String>,
    instance_id : Option<String>,
    redirect_url : Option<String>,
    payment_statusdetails : Option<PaymentStatusD>,
    redirect_params: Option<String>,
    preconditions: Option<String>,
    payment_status :Option <String>,
    
}



impl<F, T>
    TryFrom<types::ResponseRouterData<F, TrustPaySyncResponse, T, types::PaymentsResponseData>>
    for types::RouterData<F, T, types::PaymentsResponseData>
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: types::ResponseRouterData<
            F,
            TrustPaySyncResponse,
            T,
            types::PaymentsResponseData,
        >,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.status.clone()),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.instance_id.clone()),
                redirect: false,
                redirection_data: None,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct PaymentStatusD {
    extended_description : String,
    scheme_response_code : String,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ThreeDS {
    eci : String,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Card {
    masked_pan : String,
    expiration : String,
    description : Option<String>,
}


#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TrustpayCaptureRequest {
    amount : i64,
    instance_id : String,
    currency : String,
}
