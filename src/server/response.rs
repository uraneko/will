use std::collections::HashMap;

pub(crate) struct ResponseLine<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
}

pub(crate) struct Response<'a> {
    response_line: ResponseLine<'a>,
    headers: HashMap<&'a str, &'a str>,
    body: &'a str,
}

pub(crate) fn process_request<'a>(
    url_domain: &'a [&'a str; 5],
    url_params: &'a HashMap<&'a str, &'a str>,
    method: &'a str,
    headers: &'a HashMap<&'a str, &'a str>,
    body: &'a HashMap<&'a str, &'a str>,
) -> Result<Response<'a>, ResponseErr> {
    match method {
        "GET" => process_get_request(url_domain, url_params, headers),
        "HEAD" => process_head_request(url_domain, url_params, headers),
        "POST" => process_post_request(url_domain, url_params, headers, body),
        // FIXME: return an ok response method not allowed instead of returning an error
        _ => Err(ResponseErr::MethodUnsupported),
    }
}

pub(crate) fn process_head_request<'a>(
    url_domain: &'a [&'a str; 5],
    url_params: &'a HashMap<&'a str, &'a str>,
    headers: &'a HashMap<&'a str, &'a str>,
    // body: &HashMap<&str, &str>,
) -> Result<Response<'a>, ResponseErr> {
    todo!()
}

pub(crate) fn process_get_request<'a>(
    url_domain: &'a [&'a str; 5],
    url_params: &'a HashMap<&'a str, &'a str>,
    headers: &'a HashMap<&'a str, &'a str>,
    // body: &HashMap<&str, &str>,
) -> Result<Response<'a>, ResponseErr> {
    todo!()
}

pub(crate) fn process_post_request<'a>(
    url_domain: &'a [&'a str; 5],
    url_params: &'a HashMap<&'a str, &'a str>,
    headers: &'a HashMap<&'a str, &'a str>,
    body: &'a HashMap<&'a str, &'a str>,
) -> Result<Response<'a>, ResponseErr> {
    todo!()
}

#[derive(Debug)]
pub(crate) enum StatusErr {
    FileNotFound,
}

// do the functions that generate response stuff
fn status_code<'a>(status: &'a Result<Option<&str>, StatusErr>) -> &'a str {
    if let Err(e) = status {
        return match e {
            StatusErr::FileNotFound => "",
        };
    }

    let status = status.as_ref().unwrap();

    match status {
        Some(data) => "",
        None => "",
    }
}

fn status_text(status_code: &str) -> Result<&str, ResponseErr> {
    match status_code {
        "100" => Ok("Continue"),
        "101" => Ok("Switching Protocols"),
        "200" => Ok("OK"),
        "201" => Ok("Created"),
        "202" => Ok("Accepted"),
        "303" => Ok("See Other"),
        "304" => Ok("Not Modified"),
        "307" => Ok("Temporary Redirect"),
        "308" => Ok("Permanent Redirect"),
        "400" => Ok("Bad Request"),
        "401" => Ok("Unauthorized"),
        "403" => Ok("Forbidden"),
        "404" => Ok("Not Found"),
        "405" => Ok("Method Not Allowed"),
        "406" => Ok("Not Acceptable"),
        "408" => Ok("Request Timeout"),
        "410" => Ok("Gone"),
        "500" => Ok("Internal Server Error"),
        "501" => Ok("Not Implemented"),
        "502" => Ok("Bad Gateway"),
        "503" => Ok("Service Unavailable"),
        "504" => Ok("Gateway Timeout"),
        _ => Err(ResponseErr::UnsupportedStatusCode),
    }
}

pub(crate) enum ResponseErr {
    UnrecognizedStatusCode,
    UnsupportedStatusCode,
    MethodUnsupported,
}

impl<'a> Response<'a> {
    fn new() -> Self {
        Self {
            response_line: ResponseLine::<'a> {
                status_code: "",
                version: "",
                status_text: "",
            },
            body: "",
            headers: HashMap::new(),
        }
    }

    fn status_code(self, status_code: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code,
                ..self.response_line
            },
            ..self
        }
    }

    fn status_text(self, status_text: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_text,
                ..self.response_line
            },
            ..self
        }
    }

    fn version(self, version: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                version,
                ..self.response_line
            },
            ..self
        }
    }

    fn body(self, body: &'a str) -> Self {
        Self {
            response_line: self.response_line,
            body,
            ..self
        }
    }

    fn headers(self, headers: HashMap<&'a str, &'a str>) -> Self {
        Self {
            response_line: self.response_line,
            headers,
            ..self
        }
    }

    fn parse_response(self) -> &'a str {
        ""
    }

    fn is_all_filled(&self) -> bool {
        if self.response_line.version.is_empty()
            || self.response_line.status_text.is_empty()
            || self.response_line.status_code.is_empty()
            || self.headers.is_empty()
            || self.body.is_empty()
        {
            return true;
        }

        false
    }
}

trait ClientErrorResponses<'a> {
    fn method_not_allowed(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self;

    fn unauthorized(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn bad_request(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn forbidden(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn not_found(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn not_acceptable(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn request_timeout(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str)
        -> Self;

    fn gone(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;
}

// TODO: cache the parsed error messages for later uses
impl<'a> ClientErrorResponses<'a> for Response<'a> {
    fn bad_request(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "400",
                status_text: "Unauthorized",
                version,
            },
            headers,
            body,
        }
    }

    fn unauthorized(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "401",
                status_text: "Unauthorized",
                version,
            },
            headers,
            body,
        }
    }

    fn forbidden(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "403",
                status_text: "Forbidden",
                version,
            },
            headers,
            body,
        }
    }

    fn not_found(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "404",
                status_text: "Not Found",
                version,
            },
            headers,
            body,
        }
    }

    fn method_not_allowed(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "405",
                status_text: "Method Not Allowed",
                version,
            },
            headers,
            body,
        }
    }

    fn not_acceptable(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "406",
                status_text: "Not Acceptable",
                version,
            },
            headers,
            body,
        }
    }

    fn request_timeout(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "408",
                status_text: "Request Timeout",
                version,
            },
            headers,
            body,
        }
    }

    fn gone(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "410",
                status_text: "Gone",
                version,
            },
            headers,
            body,
        }
    }
}

trait ServerErrorResponses<'a> {
    fn internal_server_error(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self;

    fn not_implemented(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str)
        -> Self;

    fn bad_gateway(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn service_unavailable(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self;

    fn gateway_timeout(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str)
        -> Self;
}

impl<'a> ServerErrorResponses<'a> for Response<'a> {
    fn internal_server_error(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "500",
                status_text: "Gone",
                version,
            },
            headers,
            body,
        }
    }

    fn not_implemented(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "501",
                status_text: "Gone",
                version,
            },
            headers,
            body,
        }
    }

    fn bad_gateway(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "502",
                status_text: "Gone",
                version,
            },
            headers,
            body,
        }
    }

    fn service_unavailable(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "503",
                status_text: "Gone",
                version,
            },
            headers,
            body,
        }
    }

    fn gateway_timeout(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "504",
                status_text: "Gone",
                version,
            },
            headers,
            body,
        }
    }
}

trait RedirectionMessages<'a> {
    fn see_other(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn not_modified(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn temporary_redirect(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self;

    fn permanent_redirect(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self;
}

impl<'a> RedirectionMessages<'a> for Response<'a> {
    fn see_other(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "303",
                status_text: "See Other",
                version,
            },
            headers,
            body,
        }
    }
    fn not_modified(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "304",
                status_text: "Not Modified",
                version,
            },
            headers,
            body,
        }
    }
    fn temporary_redirect(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "307",
                status_text: "Temporary redirect",
                version,
            },
            headers,
            body,
        }
    }
    fn permanent_redirect(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "308",
                status_text: "Permanent Redirect",
                version,
            },
            headers,
            body,
        }
    }
}

trait SuccessfulResponses<'a> {
    fn ok(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn created(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;

    fn accepted(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self;
}

impl<'a> SuccessfulResponses<'a> for Response<'a> {
    fn ok(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "200",
                status_text: "OK",
                version,
            },
            headers,
            body,
        }
    }

    fn created(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "201",
                status_text: "Created",
                version,
            },
            headers,
            body,
        }
    }
    fn accepted(version: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "203",
                status_text: "Accepted",
                version,
            },
            headers,
            body,
        }
    }
}

trait InformationResponses<'a> {
    fn continue_(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>, /* , body: &'a str */
    ) -> Self;

    fn switching_protocols(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        // body: &'a str,
    ) -> Self;
}

impl<'a> InformationResponses<'a> for Response<'a> {
    fn continue_(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>, /* , body: &'a str */
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "100",
                status_text: "Continue",
                version,
            },
            headers,
            body: "",
        }
    }

    fn switching_protocols(
        version: &'a str,
        headers: HashMap<&'a str, &'a str>,
        // body: &'a str,
    ) -> Self {
        Self {
            response_line: ResponseLine {
                status_code: "101",
                status_text: "Switching Protocols",
                version,
            },
            headers,
            body: "",
        }
    }
}
