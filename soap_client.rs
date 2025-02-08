use reqwest::Client;
use quick_xml::Writer;
use quick_xml::events::{Event, BytesStart};
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso";
    
    // Construct SOAP request XML
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Decl("<?xml version=\"1.0\" encoding=\"utf-8\"?>".into()))?;
    
    let mut envelope = BytesStart::new("soap:Envelope");
    envelope.push_attribute(("xmlns:soap", "http://schemas.xmlsoap.org/soap/envelope/"));
    envelope.push_attribute(("xmlns:web", "http://www.oorsprong.org/websamples.countryinfo"));
    writer.write_event(Event::Start(envelope.clone()))?;
    
    writer.write_event(Event::Start(BytesStart::new("soap:Body")))?;
    writer.write_event(Event::Start(BytesStart::new("web:CapitalCity")))?;
    
    let mut country_code = BytesStart::new("web:sCountryISOCode");
    writer.write_event(Event::Start(country_code.clone()))?;
    writer.write_event(Event::Text("KE".into()))?;
    writer.write_event(Event::End(country_code.to_end()))?;
    
    writer.write_event(Event::End(BytesStart::new("web:CapitalCity").to_end()))?;
    writer.write_event(Event::End(BytesStart::new("soap:Body").to_end()))?;
    writer.write_event(Event::End(envelope.to_end()))?;
    
    let soap_request = String::from_utf8(writer.into_inner().into_inner())?;
    
    // Send SOAP request
    let response = client
        .post(url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "http://www.oorsprong.org/websamples.countryinfo/CountryInfoService.wso")
        .body(soap_request)
        .send()
        .await?;
    
    let response_text = response.text().await?;
    println!("Response: {}", response_text);
    
    Ok(())
}
