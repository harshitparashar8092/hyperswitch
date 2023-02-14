use serde::{Deserialize, Serialize};
use crate::{core::errors,types::{self,api, storage::enums},pii::{self,Secret}};
use api_models::payments::Card;

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Eq, PartialEq)]
pub struct SagepayPaymentsRequest {}
pub struct PaymentOption {
    pub card: Card,
}
pub struct SagepayMeta {
    pub session_token: String,
}
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct SagepaySessionRequest{
    pub vendor_name: String
}
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SagepaySessionResponse{
    pub session_token: String
}


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CardData {
    card_data: Secret<String, pii::CardNumber>,
    expiration_month: Secret<String>,
    expiration_year: Secret<String>,
    security_code: Secret<String>,
}
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SagepayCardTokenizationResponse{
    pub card_identifier: String,
    pub expiry : String,
    pub card_type : String,
}
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SagepayCardTokenizationRequest{
    pub card_details: Card,
}

impl TryFrom<&types::PaymentsPreAuthorizeRouterData> for SagepaySessionRequest{
    type Error = error_stack:: Report<errors::ConnectorError>;
    fn try_from(_item: &types::PaymentsPreAuthorizeRouterData)-> Result<Self,Self::Error>{
        let vendor_name = "sandbox";
        Ok(Self { vendor_name: (vendor_name.to_string())})
    }
}


impl<F, T>
    TryFrom<types::ResponseRouterData<F, SagepaySessionResponse, T, types::PaymentsResponseData>>
    for types::RouterData<F, T, types::PaymentsResponseData>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        item: types::ResponseRouterData<F, SagepaySessionResponse, T, types::PaymentsResponseData>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            session_token: Some(item.response.session_token),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::NoResponseId,
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

impl<F, T>
    TryFrom<types::ResponseRouterData<F, SagepayCardTokenizationResponse, T, types::PaymentsResponseData>>
    for types::RouterData<F, T, types::PaymentsResponseData>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        item: types::ResponseRouterData<F, SagepayCardTokenizationResponse, T, types::PaymentsResponseData>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            session_token: Some(item.response.card_identifier),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::NoResponseId,
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

impl TryFrom<&types::PaymentsCardTokenizeRouterData> for SagepayCardTokenizationRequest{
    type Error = error_stack:: Report<errors::ConnectorError>;
    fn try_from(item: &types::PaymentsCardTokenizeRouterData)-> Result<Self,Self::Error>{
        match item.request.card_details {
            (ref card) => Ok(Self {
                    card_details : Card{
                        card_number: card.card_number.clone(),
                        card_exp_month: card.card_exp_month.clone(),
                        card_exp_year: card.card_exp_year.clone(),
                        card_cvc : card.card_cvc.clone(),
                        card_holder_name : card.card_holder_name.clone(),

                    },
                })
    }
}
}

impl TryFrom<&types::PaymentsAuthorizeRouterData> for SagepayPaymentsRequest  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_item: &types::PaymentsAuthorizeRouterData) -> Result<Self,Self::Error> {
        todo!()
    }
}


//TODO: Fill the struct with respective fields
// Auth Struct
pub struct SagepayAuthType {
    pub(super) api_key: String
}

impl TryFrom<&types::ConnectorAuthType> for SagepayAuthType  {
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

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SagepayCard{ 
       pub merchantSessionKey: String,
       pub cardIdentifier : String,
       pub reusable: bool,
       pub save : bool,
}
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SagepayPaypal{
    pub merchantSessionKey: String,
    pub callbackurl : String
}

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SagepayApplePay {
    payload: String,
    clientIpAddress: String
  }

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SagepayPaymentMethod {
	
    card: SagepayCard ,
    paypal: SagepayPaypal ,
    applepay: SagepayApplePay
  
}

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryMethod{
    Ecommerce, MailOrder ,TelephoneOrder, ContinuousAuthority
}

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SagepayBillingAddress  {
    pub address1: String,
    pub address2 : Option<String>,
    pub address3 : Option<String>,
    pub city : String,
    pub postalCode : Option<String>,
    pub country : String,
    pub state : Option<String>
  }

#[derive(Eq, PartialEq, Debug)]
pub enum Sagepay3dSecure{
    UseMSPSetting, Force, Disable, ForceIgnoringRules
}
#[derive(Serialize, Debug, Eq, PartialEq)]
pub enum SagepayWindowSize{
    Small,Medium,Large,ExtraLarge,FullScreen
}
#[derive(Serialize, Debug, Eq, PartialEq)]
pub enum SagepayTransType{
    GoodsAndServicePurchase,CheckAcceptance,AccountFunding,QuasiCashTransaction,PrepaidActivationAndLoad 
}
#[derive(Serialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SagepaythreeDSRequestorAuthenticationInfo{
    threeDSReqAuthData : String,
    threeDSReqAuthMethod: String,
    threeDSReqAuthTimestamp: String
  }
#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct ThreeDsrequestorPriorAuthenticationInfo{
    pub threeDSReqPriorAuthData: String,
    pub threeDSReqPriorAuthMethod: String,
    pub threeDSReqPriorAuthTimestamp: String,
    pub threeDSReqPriorRef: String
  } 
#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct SagepayAcctInfo{
    chAccAgeInd: String,
    chAccChange: String,
    chAccChangeInd: String,
    chAccDate: String,
    chAccPwChange: String,
    chAccPwChangeInd: String,
    nbPurchaseAccount: String,
    provisionAttemptsDay:String,
    txnActivityDay: String,
    txnActivityYear: String,
    paymentAccAge: String,
    paymentAccInd: String,
    shipAddressUsage: String,
    shipAddressUsageInd:String,
    shipNameIndicator: String,
    suspiciousAccActivity: String
  }
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct SagepayMerchantRiskIndicator {
    deliveryEmailAddress: String,
    deliveryTimeframe: String,
    giftCardAmount: i64,
    giftCardCount: i32,
    preOrderDate: String,
    preOrderPurchaseInd: String,
    reorderItemsInd: String,
    shipIndicator: String
  }

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct SagepayStrongCustomerAuthentication{
    pub notificationURL: String,
    pub browserIP: String ,
    pub browserAcceptHeader: String ,
    pub browserJavascriptEnabled: bool,
    pub browserJavaEnabled: Option<bool>,
    pub browserLanguage: String,
    pub browserColorDepth: i32,
    pub browserScreenHeight: String,
    pub browserScreenWidth : String,
    pub browserTZ : String ,
    pub browserUserAgent: String,
    pub challengeWindowSize: SagepayWindowSize,
    pub acctID : String,
    pub transType: SagepayTransType,
    pub threeDSRequestorAuthenticationInfo: SagepaythreeDSRequestorAuthenticationInfo,
    pub threeDSRequestorPriorAuthenticationInfo: ThreeDsrequestorPriorAuthenticationInfo,
    pub acctInfo: SagepayAcctInfo,
    pub merchantRiskIndicator: SagepayMerchantRiskIndicator,
    pub threeDSExemptionIndicator: String,
    pub website: String
  }


#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct SagepayShippingDetails{
    pub recipientFirstName: String,
    pub recipientLastName: String,
    pub shippingAddress1: String,
    pub shippingAddress2:   Option<String>,
    pub shippingAddress3: Option<String>,
    pub shippingCity: String,
    pub shippingPostalCode: Option<String>,
    pub shippingCountry: String,
    pub shippingState: String
  }

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "mitType")]
  pub enum SagepaymitType{
    Instalment(Installemnt), Recurring(Recurring), Unscheduled, Incremental, DelayedCharge, NoShow ,Reauthorisation, Resubmission
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Recurring {
    recurringExpiry: String,
    recurringFrequency: String,   
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Installemnt {
    purchaseInstalData: String,
    #[serde(flatten)]
    recurring: Recurring
}


#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct SagepayCredentialType {
    cofUsage: String,
    initiatedType: String,
    #[serde(flatten)]
    mitType: SagepaymitType,
  }  

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct SagepayfiRecipient {
    accountNumber: String,
    surname: String,
    postcode: String,
    dateOfBirth: String
  }
#[derive(Debug,Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
  pub enum SagePayTransactionType{
     #[default] Payment , Deferred, Authenticate, Repeat , Refund , Authorise,
  }
  

#[derive(Debug, Eq, PartialEq)]
pub struct PaymentIntentRequest {
   pub transactionType : SagePayTransactionType,
   pub paymentMethod : SagepayPaymentMethod ,
   pub vendorTxCode : String,
   pub amount : i64,
   pub currency : String,
   pub description : String,
   pub settlementReferenceText : String,
   pub customerFirstName : String,
   pub customerLastName : String,
   pub billingAddress : SagepayBillingAddress ,
   pub entryMethod: Option<EntryMethod> ,
   pub giftAid : Option<bool>,
   pub apply3DSecure : Option<Sagepay3dSecure>,
   pub applyAvsCvcCheck: Option<Sagepay3dSecure>,
   pub customerEmail: Option<Secret<String>>,
   pub customerPhone : Option<Secret<String>>,
   pub shippingDetails: Option<SagepayShippingDetails>,
   pub referrerId: Option<String>,
   pub strongCustomerAuthentication : Option<SagepayStrongCustomerAuthentication> ,
   pub customerMobilePhone: String,
   pub customerWorkPhone :String,
   pub credentialType : SagepayCredentialType,
   pub fiRecipient : SagepayfiRecipient
  }




// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SagepayPaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SagepayAmount{
        totalAmount: i64,
        saleAmount: i64,
        surchargeAmount: i64
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
pub struct PaymentIntentResponse {
    transactionId : String,
    acsTransId : Option<String>,
    dsTransId: Option<String>,
    transactionType: SagePayTransactionType,
    status: String,
    statusCode: String,
    statusDetail: String,
    additionalDeclineDetail: Option<String>,
    retrievalReference: String,
    settlementReferenceText: String,
    bankResponseCode: String,
    bankAuthorisationCode: String,
    avsCvcCheck: String,
    paymentMethod: String,
    amount : SagepayAmount,
    currency : String,
    #[serde(rename = "3DSecure")]
    dSecure : Option<String>,
    fiRecipient: SagepayfiRecipient,
}


    


impl From<SagepayPaymentStatus> for enums::AttemptStatus {
    fn from(item: SagepayPaymentStatus) -> Self {
        match item {
            SagepayPaymentStatus::Succeeded => Self::Charged,
            SagepayPaymentStatus::Failed => Self::Failure,
            SagepayPaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SagepayPaymentsResponse {
    status: SagepayPaymentStatus,
    id: String,
}

impl<F,T> TryFrom<types::ResponseRouterData<F, SagepayPaymentsResponse, T, types::PaymentsResponseData>> for types::RouterData<F, T, types::PaymentsResponseData> {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: types::ResponseRouterData<F, SagepayPaymentsResponse, T, types::PaymentsResponseData>) -> Result<Self,Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.status),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.id),
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct SagepayRefundRequest {}

impl<F> TryFrom<&types::RefundsRouterData<F>> for SagepayRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::RefundsRouterData<F>) -> Result<Self,Self::Error> {
       todo!()
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>> for types::RefundsRouterData<api::RSync>
{
     type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: types::RefundsResponseRouterData<api::RSync, RefundResponse>) -> Result<Self,Self::Error> {
         todo!()
     }
 }

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct SagepayErrorResponse {}
