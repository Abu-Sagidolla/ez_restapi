use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
use dotenv::dotenv;
use std::env;
use chrono::prelude::*;
use chrono;
use tokio::time::Duration;
use crate::models::TSL_RESPONSE;
use crate::models::DNSER;
use crate::models::Service;
use crate::models::Cve;
use crate::models::Report;
use crate::models::Data;
use crate::models::IPReport;
use chrono::{DateTime, NaiveDateTime, Utc,Local, TimeZone};
use std::net::Ipv4Addr;


mod crud;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> 
{
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("MAILCOACH_API_TOKEN must be set.");
    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
    let timed = Utc::now();
    //migration 
    //sqlx::migrate!("./migrations").run(&pool).await?;
    let scan_results =   Data {
            udp_results: vec![IPReport {ip:"127.0.0.1".parse::<Ipv4Addr>(),port:[8881]}], // Add your actual Report instances
            tcp_results: vec![IPReport {ip:"127.0.0.1".parse::<Ipv4Addr>(),port:[88,123,111]}],
            domain_results: vec!["qotaq.kz".to_string()],
            osint: vec![], // Assuming `T` is `SomeType` for this example
            cve: vec![vec![]],
            services: vec![],
            dns_records: vec![DNSER { method: "A".to_string(), res: "192.0.2.1".to_string() }],
            tls_res: vec![TSL_RESPONSE {
                certificates: vec![
                                    "-----BEGIN CERTIFICATE-----\r\nMIIFeTCCBGGgAwIBAgIMaMoWXEyq88GT2I8vMA0GCSqGSIb3DQEBCwUAMFMxCzAJ\r\nBgNVBAYTAkJFMRkwFwYDVQQKExBHbG9iYWxTaWduIG52LXNhMSkwJwYDVQQDEyBH\r\nbG9iYWxTaWduIEdDQyBSMyBEViBUTFMgQ0EgMjAyMDAeFw0yMzAyMjcxMTM0Mjla\r\nFw0yNDAzMzAxMTM0MjhaMBYxFDASBgNVBAMMCyouZ2F6ZXRhLnJ1MFkwEwYHKoZI\r\nzj0CAQYIKoZIzj0DAQcDQgAEv6oz7pIMp1e/6qrKjV83IpyhvPEx+bSNZ43gk1ZI\r\nqhZZPecxw5MpTRNmVVvSwwTFPEbiBX18LRSdtJw+zK44OqOCA1MwggNPMA4GA1Ud\r\nDwEB/wQEAwIHgDCBkwYIKwYBBQUHAQEEgYYwgYMwRgYIKwYBBQUHMAKGOmh0dHA6\r\nLy9zZWN1cmUuZ2xvYmFsc2lnbi5jb20vY2FjZXJ0L2dzZ2NjcjNkdnRsc2NhMjAy\r\nMC5jcnQwOQYIKwYBBQUHMAGGLWh0dHA6Ly9vY3NwLmdsb2JhbHNpZ24uY29tL2dz\r\nZ2NjcjNkdnRsc2NhMjAyMDBWBgNVHSAETzBNMEEGCSsGAQQBoDIBCjA0MDIGCCsG\r\nAQUFBwIBFiZodHRwczovL3d3dy5nbG9iYWxzaWduLmNvbS9yZXBvc2l0b3J5LzAI\r\nBgZngQwBAgEwCQYDVR0TBAIwADBBBgNVHR8EOjA4MDagNKAyhjBodHRwOi8vY3Js\r\nLmdsb2JhbHNpZ24uY29tL2dzZ2NjcjNkdnRsc2NhMjAyMC5jcmwwIQYDVR0RBBow\r\nGIILKi5nYXpldGEucnWCCWdhemV0YS5ydTAdBgNVHSUEFjAUBggrBgEFBQcDAQYI\r\nKwYBBQUHAwIwHwYDVR0jBBgwFoAUDZjAc3+rvb3ZR0tJrQpKDKw+x3wwHQYDVR0O\r\nBBYEFDCYt+s3N66mpX8mdh3Pw5prxT4LMIIBfQYKKwYBBAHWeQIEAgSCAW0EggFp\r\nAWcAdgBz2Z6JG0yWeKAgfUed5rLGHNBRXnEZKoxrgBB6wXdytQAAAYaSpzVjAAAE\r\nAwBHMEUCIAcf29748sBxM2WUMlRXTKT2lxA6FzZGyDQL/wRrVFRAAiEAtWD02OJJ\r\n455t84NHVOafBQOS/w/ADhlbm4PhUHXALbMAdQDuzdBk1dsazsVct520zROiModG\r\nfLzs3sNRSFlGcR+1mwAAAYaSpzXdAAAEAwBGMEQCIFqvDfV/EzCfdBl21tA8knkK\r\nQvC1PpXSSfz4pGa8qxGfAiBIwsCP/sK8BRrUxI1D2KUheCsrZ1fhyTzv0yN4b1qi\r\nqAB2ANq2v2s/tbYin5vCu1xr6HCRcWy7UYSFNL2kPTBI1/urAAABhpKnNXIAAAQD\r\nAEcwRQIgX9uuQVAdZSVqzj/cuP6IHb0UOi4iiEfNpj8YXozjx6UCIQD/bBYct40B\r\nB0mn/PJVvYrOLg4oldm9d7jbFQM1OpdFUjANBgkqhkiG9w0BAQsFAAOCAQEAFk0x\r\nTc08we3LRglDmSmXXBBvddQRw+yw8oiXE9B3fBrstvws2Zq5DN0H/f2IQNS1MVfW\r\nG5LohpIWtSv1C0YleIyJRSrZXlCXo1eu9QTzr9i7epBBNkk3MBbh1aOK7/cWaAcg\r\nq2AE/BW9z+44d7J/sK7Wf+bcVN+oyFhCZXYTiBtQESvctvqmR7v7MyWD+tywLu4K\r\nUSzIICbO1Dk1hVIU0swiFKCjFh4tFOin1yTGk+IUd4lFjLOFWR9OEMJ9dlk6wBix\r\nIIRMPdRAHFwdf8XdhPIxvAcySFSEV6qtzz/CIxiQXBhJH0sRoPL6aUdXMlrLjvoi\r\nlFGPbMxuG1cgchTJzw==\r\n-----END CERTIFICATE-----\r\n".to_string(),
                                    "-----BEGIN CERTIFICATE-----\r\nMIIEsDCCA5igAwIBAgIQd70OB0LV2enQSdd00CpvmjANBgkqhkiG9w0BAQsFADBM\r\nMSAwHgYDVQQLExdHbG9iYWxTaWduIFJvb3QgQ0EgLSBSMzETMBEGA1UEChMKR2xv\r\nYmFsU2lnbjETMBEGA1UEAxMKR2xvYmFsU2lnbjAeFw0yMDA3MjgwMDAwMDBaFw0y\r\nOTAzMTgwMDAwMDBaMFMxCzAJBgNVBAYTAkJFMRkwFwYDVQQKExBHbG9iYWxTaWdu\r\nIG52LXNhMSkwJwYDVQQDEyBHbG9iYWxTaWduIEdDQyBSMyBEViBUTFMgQ0EgMjAy\r\nMDCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAKxnlJV/de+OpwyvCXAJ\r\nIcxPCqkFPh1lttW2oljS3oUqPKq8qX6m7K0OVKaKG3GXi4CJ4fHVUgZYE6HRdjqj\r\nhhnuHY6EBCBegcUFgPG0scB12Wi8BHm9zKjWxo3Y2bwhO8Fvr8R42pW0eINc6OTb\r\nQXC0VWFCMVzpcqgz6X49KMZowAMFV6XqtItcG0cMS//9dOJs4oBlpuqX9INxMTGp\r\n6EASAF9cnlAGy/RXkVS9nOLCCa7pCYV+WgDKLTF+OK2Vxw3RUJ/p8009lQeUARv2\r\nUCcNNPCifYX1xIspvarkdjzLwzOdLahDdQbJON58zN4V+lMj0msg+c0KnywPIRp3\r\nBMkCAwEAAaOCAYUwggGBMA4GA1UdDwEB/wQEAwIBhjAdBgNVHSUEFjAUBggrBgEF\r\nBQcDAQYIKwYBBQUHAwIwEgYDVR0TAQH/BAgwBgEB/wIBADAdBgNVHQ4EFgQUDZjA\r\nc3+rvb3ZR0tJrQpKDKw+x3wwHwYDVR0jBBgwFoAUj/BLf6guRSSuTVD6Y5qL3uLd\r\nG7wwewYIKwYBBQUHAQEEbzBtMC4GCCsGAQUFBzABhiJodHRwOi8vb2NzcDIuZ2xv\r\nYmFsc2lnbi5jb20vcm9vdHIzMDsGCCsGAQUFBzAChi9odHRwOi8vc2VjdXJlLmds\r\nb2JhbHNpZ24uY29tL2NhY2VydC9yb290LXIzLmNydDA2BgNVHR8ELzAtMCugKaAn\r\nhiVodHRwOi8vY3JsLmdsb2JhbHNpZ24uY29tL3Jvb3QtcjMuY3JsMEcGA1UdIARA\r\nMD4wPAYEVR0gADA0MDIGCCsGAQUFBwIBFiZodHRwczovL3d3dy5nbG9iYWxzaWdu\r\nLmNvbS9yZXBvc2l0b3J5LzANBgkqhkiG9w0BAQsFAAOCAQEAy8j/c550ea86oCkf\r\nr2W+ptTCYe6iVzvo7H0V1vUEADJOWelTv07Obf+YkEatdN1Jg09ctgSNv2h+LMTk\r\nKRZdAXmsE3N5ve+z1Oa9kuiu7284LjeS09zHJQB4DJJJkvtIbjL/ylMK1fbMHhAW\r\ni0O194TWvH3XWZGXZ6ByxTUIv1+kAIql/Mt29PmKraTT5jrzcVzQ5A9jw16yysuR\r\nXRrLODlkS1hyBjsfyTNZrmL1h117IFgntBA5SQNVl9ckedq5r4RSAU85jV8XK5UL\r\nREjRZt2I6M9Po9QL7guFLu4sPFJpwR1sPJvubS2THeo7SxYoNDtdyBHs7euaGcMa\r\nD/fayQ==\r\n-----END CERTIFICATE-----\r\n".to_string(),
                                    "-----BEGIN CERTIFICATE-----\r\nMIIDXzCCAkegAwIBAgILBAAAAAABIVhTCKIwDQYJKoZIhvcNAQELBQAwTDEgMB4G\r\nA1UECxMXR2xvYmFsU2lnbiBSb290IENBIC0gUjMxEzARBgNVBAoTCkdsb2JhbFNp\r\nZ24xEzARBgNVBAMTCkdsb2JhbFNpZ24wHhcNMDkwMzE4MTAwMDAwWhcNMjkwMzE4\r\nMTAwMDAwWjBMMSAwHgYDVQQLExdHbG9iYWxTaWduIFJvb3QgQ0EgLSBSMzETMBEG\r\nA1UEChMKR2xvYmFsU2lnbjETMBEGA1UEAxMKR2xvYmFsU2lnbjCCASIwDQYJKoZI\r\nhvcNAQEBBQADggEPADCCAQoCggEBAMwldpB5BngiFvXAg7aEyiie/QV2EcWtiHL8\r\nRgJDx7KKnQRfJMsuS+FggkbhUqsMgUdwbN1k0ev1LKMPgj0MK66X17YUhhB5uzsT\r\ngHeMCOFJ0mpiLx9e+pZo34knlTifBtc+ycsmWQ1z3rDI6SYOgxXG71uL0gRgykmm\r\nKPZpO/bLyCiR5Z2KYVc3rHQU3HTgOu5yLy6c+9C7v/U9AOEGM+iCK65TpjoWc4zd\r\nQQ4gOsC0p6Hpsk+QLjJg6VfLuQSSaGjlOCZgdbKfd/+RFO+uIEn8rUAVSNECMWEZ\r\nXriX7613t2Saer9fwRPvm2L7DWzgVGkWqQPabumDk3F2xmmFghcCAwEAAaNCMEAw\r\nDgYDVR0PAQH/BAQDAgEGMA8GA1UdEwEB/wQFMAMBAf8wHQYDVR0OBBYEFI/wS3+o\r\nLkUkrk1Q+mOai97i3Ru8MA0GCSqGSIb3DQEBCwUAA4IBAQBLQNvAUKr+yAzv95ZU\r\nRUm7lgAJQayzE4aGKAczymvmdLm6AC2upArT9fHxD4q/c2dKg8dEe3jgr25sbwMp\r\njjM5RcOO5LlXbKr8EpbsU8Yt5CRsuZRj+9xTaGdWPoO4zzUhw8lo/s7awlOqzJCK\r\n6fBdRoyV3XpYKBovHd7NADdBj+1EbddTKJd+82cEHhXXipa0095MJ6RMG3NzdvQX\r\nmcIfeg7jLQitChws/zyrVQ4PkX4268NXSb7hLi18YIvDQVETI53O9zJrlAGomecs\r\nMx86OyXShkDOOyyGeMlhLxS67ttVb9+E7gUJTb0o2HLO02JQZR7rkpeDMdmztcpH\r\nWD9f\r\n-----END CERTIFICATE-----\r\n".to_string()
                                  ],
              version: "TLSv1_2".to_string(),
              alpn_protocol: "None".to_string(),
              handshaking: false,
              cipher_suite: "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256".to_string(),
              vulnerabilities: vec![
                "Potential vulnerabilities: Some attacks like FREAK and Logjam remain possible.".to_string(),
                " SLOTH (Security Losses from Obsolete and Truncated Transcript Hashes):\nAllows attackers to downgrade connections to use insecure RSA-MD5 signatures.\nCould enable man-in-the-middle attacks".to_string(),
                "FREAK (Factoring RSA Export Keys):Exploits weak, export-grade cipher suites to compromise server keys.".to_string(),
                "Logjam:Exploits vulnerabilities in Diffie-Hellman key exchange to downgrade connections and decrypt traffic.".to_string(),
                "Raccoon Attack:Targets the Diffie-Hellman key exchange process to retrieve the premaster secret and decrypt traffic.".to_string(),
                "Sweet32:Exploits a flaw in 64-bit block ciphers (like 3DES and Blowfish) to recover plaintext data after enough encrypted traffic is captured.".to_string(),
                "POODLE (Padding Oracle On Downgraded Legacy Encryption):While primarily a TLS 1.0 vulnerability, it could still affect TLS 1.2 under certain configurations.".to_string()
                  ]
            }],
            xss_sex: vec![],
            hostInjection: vec![],
            SQL_results: vec![],
        };


    let new_book = models::Report {
        scanned:timed,
        scan_data:  &scan_results
    };
    //crud::create(&new_book,&pool).await?;
    println!("{:?}",crud::read(&pool).await?);
    Ok(())
}
