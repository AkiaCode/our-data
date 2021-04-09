/// 에러코드
pub enum Code {
    /// 정상
    NormalCode = 00,
    /// 어플리케이션 에러
    ApplicationError = 01,
    /// 데이터베이스 에러
    DbError = 02,
    /// 데이터없음 에러
    NodataError = 03,
    /// HTTP 에러
    HttpError = 04,
    /// 서비스 연결실패 에러
    ServicetimeoutError = 05,
    /// 잘못된 요청 파라메터 에러
    InvalidRequestParameterError = 10,
    /// 필수요청 파라메터가 없음
    NoMandatoryRequestParametersError = 11,
    /// 해당 오픈API서비스가 없거나 폐기됨
    NoOpenapiServiceError = 12,
    /// 서비스 접근거부
    ServiceAccessDeniedError = 20,
    /// 일시적으로 사용할 수 없는 서비스 키
    TemporarilyDisableTheServicekeyError = 21,
    /// 서비스 요청제한횟수 초과에러
    LimitedNumberOfServiceRequestsExceedsError = 22,
    /// 등록되지 않은 서비스키
    ServiceKeyIsNotRegisteredError = 30,
    /// 기한만료된 서비스키
    DeadlineHasExpiredError = 31,
    /// 등록되지 않은 IP
    UnregisteredIpError = 32,
    /// 서명되지 않은 호출
    UnsignedCallError = 33,
    /// 기타에러
    UnknownError = 99,
}
