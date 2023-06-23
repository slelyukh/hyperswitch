#![allow(dead_code, unused_variables)]

use api_models::errors::types::Extra;
use http::StatusCode;

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    InvalidRequestError,
    ObjectNotFound,
    RouterError,
    ProcessingError,
    BadGateway,
    ServerNotAvailable,
    DuplicateRequest,
    ValidationError,
    ConnectorError,
}

#[allow(dead_code)]
#[derive(Debug, Clone, router_derive::ApiError)]
#[error(error_type_enum = ErrorType)]
pub enum ApiErrorResponse {
    #[error(error_type = ErrorType::ServerNotAvailable, code = "IR_00", message = "{message:?}")]
    NotImplemented { message: NotImplementedMessage },
    #[error(
        error_type = ErrorType::InvalidRequestError, code = "IR_01",
        message = "API key not provided or invalid API key used"
    )]
    Unauthorized,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_02", message = "Unrecognized request URL")]
    InvalidRequestUrl,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_03", message = "The HTTP method is not applicable for this API")]
    InvalidHttpMethod,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_04", message = "Missing required param: {field_name}")]
    MissingRequiredField { field_name: &'static str },
    #[error(
        error_type = ErrorType::InvalidRequestError, code = "IR_05",
        message = "{field_name} contains invalid data. Expected format is {expected_format}"
    )]
    InvalidDataFormat {
        field_name: String,
        expected_format: String,
    },
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_06", message = "{message}")]
    InvalidRequestData { message: String },
    /// Typically used when a field has invalid value, or deserialization of the value contained in a field fails.
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_07", message = "Invalid value provided: {field_name}")]
    InvalidDataValue { field_name: &'static str },
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_08", message = "Client secret was not provided")]
    ClientSecretNotGiven,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_08", message = "Client secret has expired")]
    ClientSecretExpired,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_09", message = "The client_secret provided does not match the client_secret associated with the Payment")]
    ClientSecretInvalid,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_10", message = "Customer has active mandate/subsciption")]
    MandateActive,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_11", message = "Customer has already been redacted")]
    CustomerRedacted,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_12", message = "Reached maximum refund attempts")]
    MaximumRefundCount,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_13", message = "Refund amount exceeds the payment amount")]
    RefundAmountExceedsPaymentAmount,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_14", message = "This Payment could not be {current_flow} because it has a {field_name} of {current_value}. The expected state is {states}")]
    PaymentUnexpectedState {
        current_flow: String,
        field_name: String,
        current_value: String,
        states: String,
    },
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_15", message = "Invalid Ephemeral Key for the customer")]
    InvalidEphemeralKey,
    /// Typically used when information involving multiple fields or previously provided information doesn't satisfy a condition.
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_16", message = "{message}")]
    PreconditionFailed { message: String },
    #[error(
        error_type = ErrorType::InvalidRequestError, code = "IR_17",
        message = "Access forbidden, invalid JWT token was used"
    )]
    InvalidJwtToken,
    #[error(
        error_type = ErrorType::InvalidRequestError, code = "IR_18",
        message = "{message}",
    )]
    GenericUnauthorized { message: String },
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_19", message = "{message}")]
    NotSupported { message: String },
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_20", message = "{flow} flow not supported by the {connector} connector")]
    FlowNotSupported { flow: String, connector: String },
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_21", message = "Missing required params")]
    MissingRequiredFields { field_names: Vec<&'static str> },
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_22", message = "Access forbidden. Not authorized to access this resource")]
    AccessForbidden,
    #[error(error_type = ErrorType::InvalidRequestError, code = "IR_23", message = "{message}")]
    FileProviderNotSupported { message: String },
    #[error(error_type = ErrorType::ConnectorError, code = "CE_00", message = "{code}: {message}", ignore = "status_code")]
    ExternalConnectorError {
        code: String,
        message: String,
        connector: String,
        status_code: u16,
        reason: Option<String>,
    },
    #[error(error_type = ErrorType::ProcessingError, code = "CE_01", message = "Payment failed during authorization with connector. Retry payment")]
    PaymentAuthorizationFailed { data: Option<serde_json::Value> },
    #[error(error_type = ErrorType::ProcessingError, code = "CE_02", message = "Payment failed during authentication with connector. Retry payment")]
    PaymentAuthenticationFailed { data: Option<serde_json::Value> },
    #[error(error_type = ErrorType::ProcessingError, code = "CE_03", message = "Capture attempt failed while processing with connector")]
    PaymentCaptureFailed { data: Option<serde_json::Value> },
    #[error(error_type = ErrorType::ProcessingError, code = "CE_04", message = "The card data is invalid")]
    InvalidCardData { data: Option<serde_json::Value> },
    #[error(error_type = ErrorType::ProcessingError, code = "CE_05", message = "The card has expired")]
    CardExpired { data: Option<serde_json::Value> },
    #[error(error_type = ErrorType::ProcessingError, code = "CE_06", message = "Refund failed while processing with connector. Retry refund")]
    RefundFailed { data: Option<serde_json::Value> },
    #[error(error_type = ErrorType::ProcessingError, code = "CE_07", message = "Verification failed while processing with connector. Retry operation")]
    VerificationFailed { data: Option<serde_json::Value> },
    #[error(error_type = ErrorType::ProcessingError, code = "CE_08", message = "Dispute operation failed while processing with connector. Retry operation")]
    DisputeFailed { data: Option<serde_json::Value> },

    #[error(error_type = ErrorType::ServerNotAvailable, code = "HE_00", message = "Something went wrong")]
    InternalServerError,
    #[error(error_type = ErrorType::DuplicateRequest, code = "HE_01", message = "Duplicate refund request. Refund already attempted with the refund ID")]
    DuplicateRefundRequest,
    #[error(error_type = ErrorType::DuplicateRequest, code = "HE_01", message = "Duplicate mandate request. Mandate already attempted with the Mandate ID")]
    DuplicateMandate,
    #[error(error_type = ErrorType::DuplicateRequest, code = "HE_01", message = "The merchant account with the specified details already exists in our records")]
    DuplicateMerchantAccount,
    #[error(error_type = ErrorType::DuplicateRequest, code = "HE_01", message = "The merchant connector account with the specified connector_label '{connector_label}' already exists in our records")]
    DuplicateMerchantConnectorAccount { connector_label: String },
    #[error(error_type = ErrorType::DuplicateRequest, code = "HE_01", message = "The payment method with the specified details already exists in our records")]
    DuplicatePaymentMethod,
    #[error(error_type = ErrorType::DuplicateRequest, code = "HE_01", message = "The payment with the specified payment_id '{payment_id}' already exists in our records")]
    DuplicatePayment { payment_id: String },
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Refund does not exist in our records")]
    RefundNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Customer does not exist in our records")]
    CustomerNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "RE_02", message = "Config key does not exist in our records.")]
    ConfigNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Payment does not exist in our records")]
    PaymentNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Payment method does not exist in our records")]
    PaymentMethodNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Merchant account does not exist in our records")]
    MerchantAccountNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Merchant connector account with id '{id}' does not exist in our records")]
    MerchantConnectorAccountNotFound { id: String },
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Resource ID does not exist in our records")]
    ResourceIdNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Mandate does not exist in our records")]
    MandateNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "Failed to update mandate")]
    MandateUpdateFailed,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_02", message = "API Key does not exist in our records")]
    ApiKeyNotFound,
    #[error(error_type = ErrorType::ValidationError, code = "HE_03", message = "Return URL is not configured and not passed in payments request")]
    ReturnUrlUnavailable,
    #[error(error_type = ErrorType::ValidationError, code = "HE_03", message = "This refund is not possible through Hyperswitch. Please raise the refund through {connector} dashboard")]
    RefundNotPossible { connector: String },
    #[error(error_type = ErrorType::ValidationError, code = "HE_03", message = "Mandate Validation Failed" )]
    MandateValidationFailed { reason: String },
    #[error(error_type= ErrorType::ValidationError, code = "HE_03", message = "The payment has not succeeded yet. Please pass a successful payment to initiate refund")]
    PaymentNotSucceeded,
    #[error(error_type = ErrorType::ValidationError, code = "HE_03", message = "The specified merchant connector account is disabled")]
    MerchantConnectorAccountDisabled,
    #[error(error_type= ErrorType::ObjectNotFound, code = "HE_04", message = "Successful payment not found for the given payment id")]
    SuccessfulPaymentNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_04", message = "The connector provided in the request is incorrect or not available")]
    IncorrectConnectorNameGiven,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_04", message = "Address does not exist in our records")]
    AddressNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_04", message = "Dispute does not exist in our records")]
    DisputeNotFound { dispute_id: String },
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_04", message = "File does not exist in our records")]
    FileNotFound,
    #[error(error_type = ErrorType::ObjectNotFound, code = "HE_04", message = "File not available")]
    FileNotAvailable,
    #[error(error_type = ErrorType::InvalidRequestError, code = "HE_04", message = "Dispute status validation failed")]
    DisputeStatusValidationFailed { reason: String },
    #[error(error_type = ErrorType::InvalidRequestError, code = "HE_04", message = "Card with the provided iin does not exist")]
    InvalidCardIin,
    #[error(error_type = ErrorType::InvalidRequestError, code = "HE_04", message = "The provided card IIN length is invalid, please provide an iin with 6 or 8 digits")]
    InvalidCardIinLength,
    #[error(error_type = ErrorType::ValidationError, code = "HE_03", message = "File validation failed")]
    FileValidationFailed { reason: String },
    #[error(error_type = ErrorType::InvalidRequestError, code = "HE_04", message = "File not found / valid in the request")]
    MissingFile,
    #[error(error_type = ErrorType::InvalidRequestError, code = "HE_04", message = "Dispute id not found in the request")]
    MissingDisputeId,
    #[error(error_type = ErrorType::InvalidRequestError, code = "HE_04", message = "File purpose not found in the request or is invalid")]
    MissingFilePurpose,
    #[error(error_type = ErrorType::InvalidRequestError, code = "HE_04", message = "File content type not found / valid")]
    MissingFileContentType,
    #[error(error_type = ErrorType::InvalidRequestError, code = "WE_01", message = "Failed to authenticate the webhook")]
    WebhookAuthenticationFailed,
    #[error(error_type = ErrorType::ObjectNotFound, code = "WE_04", message = "Webhook resource not found")]
    WebhookResourceNotFound,
    #[error(error_type = ErrorType::InvalidRequestError, code = "WE_02", message = "Bad request received in webhook")]
    WebhookBadRequest,
    #[error(error_type = ErrorType::RouterError, code = "WE_03", message = "There was some issue processing the webhook")]
    WebhookProcessingFailure,
    #[error(error_type = ErrorType::InvalidRequestError, code = "HE_04", message = "required payment method is not configured or configured incorrectly for all configured connectors")]
    IncorrectPaymentMethodConfiguration,
    #[error(error_type = ErrorType::InvalidRequestError, code = "WE_05", message = "Unable to process the webhook body")]
    WebhookUnprocessableEntity,
}

#[derive(Clone)]
pub enum NotImplementedMessage {
    Reason(String),
    Default,
}

impl std::fmt::Debug for NotImplementedMessage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reason(message) => write!(fmt, "{message} is not implemented"),
            Self::Default => {
                write!(
                    fmt,
                    "This API is under development and will be made available soon."
                )
            }
        }
    }
}

impl ::core::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{{"error":{}}}"#,
            serde_json::to_string(self).unwrap_or_else(|_| "API error response".to_string())
        )
    }
}

impl actix_web::ResponseError for ApiErrorResponse {
    fn status_code(&self) -> StatusCode {
        common_utils::errors::ErrorSwitch::<api_models::errors::types::ApiErrorResponse>::switch(
            self,
        )
        .status_code()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        common_utils::errors::ErrorSwitch::<api_models::errors::types::ApiErrorResponse>::switch(
            self,
        )
        .error_response()
    }
}

impl crate::services::EmbedError for error_stack::Report<ApiErrorResponse> {}

impl common_utils::errors::ErrorSwitch<api_models::errors::types::ApiErrorResponse>
    for ApiErrorResponse
{
    fn switch(&self) -> api_models::errors::types::ApiErrorResponse {
        use api_models::errors::types::{ApiError, ApiErrorResponse as AER};

        let error_message = self.error_message();
        let error_codes = self.error_code();
        let error_type = self.error_type();

        match self {
            Self::NotImplemented { message } => {
                AER::NotImplemented(ApiError::new("IR", 0, format!("{message:?}"), None))
            }
            Self::Unauthorized => AER::Unauthorized(ApiError::new(
                "IR",
                1,
                "API key not provided or invalid API key used", None
            )),
            Self::InvalidRequestUrl => {
                AER::NotFound(ApiError::new("IR", 2, "Unrecognized request URL", None))
            }
            Self::InvalidHttpMethod => AER::MethodNotAllowed(ApiError::new(
                "IR",
                3,
                "The HTTP method is not applicable for this API", None
            )),
            Self::MissingRequiredField { field_name } => AER::BadRequest(
                ApiError::new("IR", 4, format!("Missing required param: {field_name}"), None),
            ),
            Self::InvalidDataFormat {
                field_name,
                expected_format,
            } => AER::Unprocessable(ApiError::new(
                "IR",
                5,
                format!(
                    "{field_name} contains invalid data. Expected format is {expected_format}"
                ), None
            )),
            Self::InvalidRequestData { message } => {
                AER::Unprocessable(ApiError::new("IR", 6, message.to_string(), None))
            }
            Self::InvalidDataValue { field_name } => AER::BadRequest(ApiError::new(
                "IR",
                7,
                format!("Invalid value provided: {field_name}"), None
            )),
            Self::ClientSecretNotGiven => AER::BadRequest(ApiError::new(
                "IR",
                8,
                "client_secret was not provided", None
            )),
            Self::ClientSecretInvalid => {
                AER::BadRequest(ApiError::new("IR", 9, "The client_secret provided does not match the client_secret associated with the Payment", None))
            }
            Self::MandateActive => {
                AER::BadRequest(ApiError::new("IR", 10, "Customer has active mandate/subsciption", None))
            }
            Self::CustomerRedacted => {
                AER::BadRequest(ApiError::new("IR", 11, "Customer has already been redacted", None))
            }
            Self::MaximumRefundCount => AER::BadRequest(ApiError::new("IR", 12, "Reached maximum refund attempts", None)),
            Self::RefundAmountExceedsPaymentAmount => {
                AER::BadRequest(ApiError::new("IR", 13, "Refund amount exceeds the payment amount", None))
            }
            Self::PaymentUnexpectedState {
                current_flow,
                field_name,
                current_value,
                states,
            } => AER::BadRequest(ApiError::new("IR", 14, format!("This Payment could not be {current_flow} because it has a {field_name} of {current_value}. The expected state is {states}"), None)),
            Self::InvalidEphemeralKey => AER::Unauthorized(ApiError::new("IR", 15, "Invalid Ephemeral Key for the customer", None)),
            Self::PreconditionFailed { message } => {
                AER::BadRequest(ApiError::new("IR", 16, message.to_string(), None))
            }
            Self::InvalidJwtToken => AER::Unauthorized(ApiError::new("IR", 17, "Access forbidden, invalid JWT token was used", None)),
            Self::GenericUnauthorized { message } => {
                AER::Unauthorized(ApiError::new("IR", 18, message.to_string(), None))
            },
            Self::ClientSecretExpired => AER::BadRequest(ApiError::new(
                "IR",
                19,
                "The provided client_secret has expired", None
            )),
            Self::MissingRequiredFields { field_names } => AER::BadRequest(
                ApiError::new("IR", 21, "Missing required params".to_string(), Some(Extra {data: Some(serde_json::json!(field_names)), ..Default::default() })),
            ),
            Self::AccessForbidden => AER::ForbiddenCommonResource(ApiError::new("IR", 22, "Access forbidden. Not authorized to access this resource", None)),
            Self::FileProviderNotSupported { message } => {
                AER::BadRequest(ApiError::new("IR", 23, message.to_string(), None))
            },
            Self::ExternalConnectorError {
                code,
                message,
                connector,
                reason,
                status_code,
            } => AER::ConnectorError(ApiError::new("CE", 0, format!("{code}: {message}"), Some(Extra {connector: Some(connector.clone()), reason: reason.clone(), ..Default::default()})), StatusCode::from_u16(*status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)),
            Self::PaymentAuthorizationFailed { data } => {
                AER::BadRequest(ApiError::new("CE", 1, "Payment failed during authorization with connector. Retry payment", Some(Extra { data: data.clone(), ..Default::default()})))
            }
            Self::PaymentAuthenticationFailed { data } => {
                AER::BadRequest(ApiError::new("CE", 2, "Payment failed during authentication with connector. Retry payment", Some(Extra { data: data.clone(), ..Default::default()})))
            }
            Self::PaymentCaptureFailed { data } => {
                AER::BadRequest(ApiError::new("CE", 3, "Capture attempt failed while processing with connector", Some(Extra { data: data.clone(), ..Default::default()})))
            }
            Self::DisputeFailed { data } => {
                AER::BadRequest(ApiError::new("CE", 1, "Dispute operation failed while processing with connector. Retry operation", Some(Extra { data: data.clone(), ..Default::default()})))
            }
            Self::InvalidCardData { data } => AER::BadRequest(ApiError::new("CE", 4, "The card data is invalid", Some(Extra { data: data.clone(), ..Default::default()}))),
            Self::CardExpired { data } => AER::BadRequest(ApiError::new("CE", 5, "The card has expired", Some(Extra { data: data.clone(), ..Default::default()}))),
            Self::RefundFailed { data } => AER::BadRequest(ApiError::new("CE", 6, "Refund failed while processing with connector. Retry refund", Some(Extra { data: data.clone(), ..Default::default()}))),
            Self::VerificationFailed { data } => {
                AER::BadRequest(ApiError::new("CE", 7, "Verification failed while processing with connector. Retry operation", Some(Extra { data: data.clone(), ..Default::default()})))
            },
            Self::MandateUpdateFailed | Self::InternalServerError => {
                AER::InternalServerError(ApiError::new("HE", 0, "Something went wrong", None))
            }
            Self::DuplicateRefundRequest => AER::BadRequest(ApiError::new("HE", 1, "Duplicate refund request. Refund already attempted with the refund ID", None)),
            Self::DuplicateMandate => AER::BadRequest(ApiError::new("HE", 1, "Duplicate mandate request. Mandate already attempted with the Mandate ID", None)),
            Self::DuplicateMerchantAccount => AER::BadRequest(ApiError::new("HE", 1, "The merchant account with the specified details already exists in our records", None)),
            Self::DuplicateMerchantConnectorAccount { connector_label } => {
                AER::BadRequest(ApiError::new("HE", 1, format!("The merchant connector account with the specified connector_label '{connector_label}' already exists in our records"), None))
            }
            Self::DuplicatePaymentMethod => AER::BadRequest(ApiError::new("HE", 1, "The payment method with the specified details already exists in our records", None)),
            Self::DuplicatePayment { payment_id } => {
                AER::BadRequest(ApiError::new("HE", 1, format!("The payment with the specified payment_id '{payment_id}' already exists in our records"), None))
            }
            Self::RefundNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "Refund does not exist in our records.", None))
            }
            Self::CustomerNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "Customer does not exist in our records", None))
            }
            Self::ConfigNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "Config key does not exist in our records.", None))
            }
            Self::PaymentNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "Payment does not exist in our records", None))
            }
            Self::PaymentMethodNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "Payment method does not exist in our records", None))
            }
            Self::MerchantAccountNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "Merchant account does not exist in our records", None))
            }
            Self::MerchantConnectorAccountNotFound { id } => {
                AER::NotFound(ApiError::new("HE", 2, format!("Merchant connector account with id '{id}' does not exist in our records"), None))
            }
            Self::MerchantConnectorAccountDisabled => {
                AER::BadRequest(ApiError::new("HE", 3, "The selected merchant connector account is disabled", None))
            }
            Self::ResourceIdNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "Resource ID does not exist in our records", None))
            }
            Self::MandateNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "Mandate does not exist in our records", None))
            }
            Self::ReturnUrlUnavailable => AER::NotFound(ApiError::new("HE", 3, "Return URL is not configured and not passed in payments request", None)),
            Self::RefundNotPossible { connector } => {
                AER::BadRequest(ApiError::new("HE", 3, format!("This refund is not possible through Hyperswitch. Please raise the refund through {connector} dashboard"), None))
            }
            Self::MandateValidationFailed { reason } => {
                AER::BadRequest(ApiError::new("HE", 3, "Mandate Validation Failed", Some(Extra { reason: Some(reason.clone()), ..Default::default() })))
            }
            Self::PaymentNotSucceeded => AER::BadRequest(ApiError::new("HE", 3, "The payment has not succeeded yet. Please pass a successful payment to initiate refund", None)),
            Self::SuccessfulPaymentNotFound => {
                AER::NotFound(ApiError::new("HE", 4, "Successful payment not found for the given payment id", None))
            }
            Self::IncorrectConnectorNameGiven => {
                AER::NotFound(ApiError::new("HE", 4, "The connector provided in the request is incorrect or not available", None))
            }
            Self::AddressNotFound => {
                AER::NotFound(ApiError::new("HE", 4, "Address does not exist in our records", None))
            },
            Self::ApiKeyNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "API Key does not exist in our records", None))
            }
            Self::NotSupported { message } => {
                AER::BadRequest(ApiError::new("HE", 3, "Payment method type not supported", Some(Extra {reason: Some(message.to_owned()), ..Default::default()})))
            },
            Self::InvalidCardIin => AER::BadRequest(ApiError::new("HE", 3, "The provided card IIN does not exist", None)),
            Self::InvalidCardIinLength  => AER::BadRequest(ApiError::new("HE", 3, "The provided card IIN length is invalid, please provide an IIN with 6 digits", None)),
            Self::FlowNotSupported { flow, connector } => {
                AER::BadRequest(ApiError::new("IR", 20, format!("{flow} flow not supported"), Some(Extra {connector: Some(connector.to_owned()), ..Default::default()}))) //FIXME: error message
            }
            Self::DisputeNotFound { .. } => {
                AER::NotFound(ApiError::new("HE", 2, "Dispute does not exist in our records", None))
            }
            Self::FileNotFound => {
                AER::NotFound(ApiError::new("HE", 2, "File does not exist in our records", None))
            }
            Self::FileNotAvailable => {
                AER::NotFound(ApiError::new("HE", 2, "File not available", None))
            }
            Self::DisputeStatusValidationFailed { .. } => {
                AER::BadRequest(ApiError::new("HE", 2, "Dispute status validation failed", None))
            }
            Self::FileValidationFailed { reason } => {
                AER::BadRequest(ApiError::new("HE", 2, format!("File validation failed {reason}"), None))
            }
            Self::MissingFile => {
                AER::BadRequest(ApiError::new("HE", 2, "File not found in the request", None))
            }
            Self::MissingFilePurpose => {
                AER::BadRequest(ApiError::new("HE", 2, "File purpose not found in the request or is invalid", None))
            }
            Self::MissingFileContentType => {
                AER::BadRequest(ApiError::new("HE", 2, "File content type not found", None))
            }
            Self::MissingDisputeId => {
                AER::BadRequest(ApiError::new("HE", 2, "Dispute id not found in the request", None))
            }
            Self::WebhookAuthenticationFailed => {
                AER::Unauthorized(ApiError::new("WE", 1, "Webhook authentication failed", None))
            }
            Self::WebhookResourceNotFound => {
                AER::NotFound(ApiError::new("WE", 4, "Webhook resource was not found", None))
            }
            Self::WebhookBadRequest => {
                AER::BadRequest(ApiError::new("WE", 2, "Bad request body received", None))
            }
            Self::WebhookProcessingFailure => {
                AER::InternalServerError(ApiError::new("WE", 3, "There was an issue processing the webhook", None))
            },
            Self::IncorrectPaymentMethodConfiguration => {
                AER::BadRequest(ApiError::new("HE", 4, "No eligible connector was found for the current payment method configuration", None))
            }
            Self::WebhookUnprocessableEntity => {
                AER::Unprocessable(ApiError::new("WE", 5, "There was an issue processing the webhook body", None))
            }
        }
    }
}
