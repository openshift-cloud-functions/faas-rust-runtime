use actix_web::HttpResponse;
use cloudevent::http::*;
use cloudevent::Event;

const DEFAULT_ENCODING: Encoding = Encoding::BINARY;

pub fn write_cloud_event(
    mut ce: Vec<Event>,
    e: Option<Encoding>,
) -> Result<HttpResponse, actix_web::Error> {
    if ce.len() == 1 {
        let encoding = e.unwrap_or(DEFAULT_ENCODING);

        return match encoding {
            Encoding::STRUCTURED => write_structured(ce.remove(0)),
            _ => write_binary(ce.remove(0)),
        };
    } else if ce.len() == 0 {
        return Ok(HttpResponse::Accepted().finish());
    } else {
        unimplemented!()
    }
}

fn write_binary(event: Event) -> Result<HttpResponse, actix_web::Error> {
    // Write headers
    let mut builder = HttpResponse::Ok();
    builder.header(CE_ID_HEADER, event.id);
    builder.header(CE_SPECVERSION_HEADER, event.spec_version.to_string());
    builder.header(CE_SOURCE_HEADER, event.source);
    builder.header(CE_TYPE_HEADER, event.event_type);
    if let Some(sub) = event.subject {
        builder.header(CE_SUBJECT_HEADER, sub);
    }
    if let Some(time) = event.time {
        builder.header(CE_TIME_HEADER, time.to_rfc3339());
    }
    let result = if let Some(p) = event.payload {
        builder.content_type(p.content_type).body(p.data)
    } else {
        builder.finish()
    };

    Ok(result)
}

fn write_structured(event: Event) -> Result<HttpResponse, actix_web::Error> {
    serde_json::to_vec(&event)
        .map(|j| {
            HttpResponse::Ok()
                .content_type("application/cloudevents+json")
                .body(j)
        })
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))
}
