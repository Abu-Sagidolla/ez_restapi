use sqlx::FromRow;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use std::time::Duration;
use std::net::Ipv4Addr;
use chrono;
use serde_json::{Value as JsonValue, to_value};
use sqlx::{Encode, TypeInfo};
use sqlx::Type;
use serde_json::json;
use chrono::{DateTime, Local};
//main output and input data 


#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct Report {
    pub scan_data: Data,
    pub scanned: Option<NaiveDateTime>,
    pub scan_counter: i8 

}

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct IPReport {
    ip: Ipv4Addr,
    ports: Vec<u16>,
}
#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct Data{
    udp_results: Vec<IPReport>,
    tcp_results: Vec<IPReport>,
    domain_results: Vec<String>,
    osint: Vec<String>,
    cve: Vec<Vec<Cve>>,
    services: Vec<Service>,
    dns_records: Vec<DNSER>,
    tls_res: Vec<TSL_RESPONSE>,
    xss_sex: Vec<(bool,Responds)>,
    hostInjection: Vec<Responds_with_cookies<Vec<Headers>>>,
    SQL_results: Vec<Responds>
}




//optional to main output 
#[derive(FromRow)]
struct Port_Service {
    service: Service,
    port: u16,
}

#[derive(Serialize,Deserialize,Debug, Clone, Hash, Eq, PartialEq,FromRow)]
pub struct Responds {
    pub endpoint: String,
    pub status: String,
    pub body: String,
    pub body_length: usize,
    pub time: Duration,
}

#[derive(Serialize,Deserialize,Debug, Clone, Hash, Eq, PartialEq,FromRow)]
pub struct Responds_with_cookies<T> {
    pub endpoint: String,
    pub status: String,
    pub body: String,
    pub body_length: usize,
    pub time: Duration,
    pub payload: T,
}


#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct Service {
    pub cpe: String,
    pub service: String,
    pub port: u16,
}


#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct Cve {
    #[serde(default)]
    id: String,
    lastModified: String,
    // ... other fields ...
    pub descriptions: Vec<Description>,
    // ... other fields ...
    references: Vec<Reference>,
    pub metrics: Metrics,
}

#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct Description {
    pub lang: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct Reference {
    pub url: String,
    pub source: String,
    // ... other fields ...
}

#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct Vulnerabilities {
    pub vulnerabilities: Vec<Qotaq>,
    // ... other fields ...
}
#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct Qotaq {
    pub cve: Cve,
}

#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct CvssDataV3 {
    pub baseScore: f32,
    pub baseSeverity: String,
    pub vectorString: String,
    //accessVector: String,
}
#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct CvssDataV2 {
    pub baseScore: f32,
    pub vectorString: String,
    //accessVector: String,
    //baseSeverity: String,
}

#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct CvssMetricV31 {
    pub cvssData: CvssDataV3,
    pub exploitabilityScore: f32,
    pub impactScore: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone,FromRow)]
pub struct CvssMetricV2 {
    pub cvssData: CvssDataV2,
    pub exploitabilityScore: f32,
    pub impactScore: f32,
    pub baseSeverity: String, //r#type: String, // "type" is a reserved keyword, so use a different name
}
#[derive(Debug, Serialize, Deserialize, Clone,FromRow)]
pub struct Metrics {
    pub cvssMetricV31: Option<Vec<CvssMetricV31>>, // CVSS version 3 metrics
    pub cvssMetricV2: Option<Vec<CvssMetricV2>>,   // CVSS version 2 metrics (optional)
                                                   // published: String, // Add the published field here
}


#[derive(Serialize,Deserialize,Debug, Clone,FromRow)]
pub struct Headers {
    name: String,
    value: String,
}

#[derive(Deserialize,Serialize,Debug,FromRow)]
pub struct DNSER {
    pub method: String,
    pub res: String,
}


#[derive(Debug, Serialize, Deserialize,FromRow)] // Add Serialize here
pub struct TSL_RESPONSE {
    certificates: Vec<String>,
    version: String,
    alpn_protocol: String,
    handshaking: bool,
    cipher_suite: String,
    vulnerabilities: Vec<String>,
}