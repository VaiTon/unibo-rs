pub mod models;

use chrono::NaiveDate;
use models::{Aula, Impegno, Lezione};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use std::error::Error;

const HOST: &str = "https://apache.prod.up.cineca.it";

const CAL1: &str = "5f632ffc78b5fe001d1ea638";
const CAL_RISORGIMENTO: &str = "5e9996a228a649001237296d";
const CLIENT: &str = "5ad08435b6ca5357dbac609e";

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.append("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36"));

        Client {
            client: reqwest::blocking::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }
    pub fn get_aule(&self) -> Result<Vec<Aula>, Box<dyn Error>> {
        let params = AulaRequest {
            link_calendario_id: CAL_RISORGIMENTO.to_owned(),
            cliente_id: CLIENT.to_owned(),
            aule_ids: vec![],
            edifici_ids: vec![],
            limit: 100,
            order: "edificio.codice".to_owned(),
        };

        let resp = self
            .client
            .post(&get_aule_endpoint())
            .json(&params)
            .send()?;
        let aule: Vec<Aula> = resp.json()?;
        Ok(aule)
    }

    pub fn get_lezioni(
        &self,
        corso: &str,
        anno: u8,
        start: &NaiveDate,
        end: &NaiveDate,
    ) -> Result<Vec<Lezione>, Box<dyn Error>> {
        let url = format!(
            "https://corsi.unibo.it/laurea/{}/orario-lezioni/@@orario_reale_json?anno={}&start={}&end={}",
            corso, anno, start, end
        );

        let resp = self.client.get(&url).send()?;
        let lezioni: Vec<Lezione> = resp.json()?;

        Ok(lezioni)
    }

    pub fn get_impegni_calendario(
        &self,
        aule_ids: Vec<&str>,
    ) -> Result<Vec<Impegno>, Box<dyn Error>> {
        let aule_ids: Vec<String> = aule_ids.iter().map(|it| it.to_string()).collect();

        let params = CalendarioRequest {
            mostra_impegni_annullati: true,
            mostra_indisponibilita_totali: false,
            linkCalendarioId: CAL_RISORGIMENTO.to_owned(),
            auleIds: aule_ids.to_vec(),
            clienteId: CLIENT.to_owned(),
            dataInizio: "2023-05-26T00:00:00.000Z".to_owned(),
            data_fine: "2023-05-26T23:59:59.999Z".to_owned(),
            pianificazioneTemplate: false,
            limitaRisultati: false,
        };

        let resp = self
            .client
            .post(&get_calendario_endpoint())
            .json(&params)
            .send()?;

        let impegni: Vec<Impegno> = resp.json()?;

        let impegni: Vec<Impegno> = impegni
            .iter()
            .filter(|i| i.aule.iter().any(|aula| aule_ids.contains(&aula.id)))
            .cloned()
            .collect();

        Ok(impegni)
    }
}

#[derive(Serialize, Debug)]
struct AulaRequest {
    #[serde(rename = "linkCalendarioId")]
    link_calendario_id: String,
    #[serde(rename = "clienteId")]
    cliente_id: String,
    #[serde(rename = "auleIds")]
    aule_ids: Vec<String>,
    #[serde(rename = "edificiIds")]
    edifici_ids: Vec<String>,
    limit: i32,
    order: String,
}
#[derive(Serialize, Debug)]
struct CalendarioRequest {
    #[serde(rename = "mostraImpegniAnnullati")]
    mostra_impegni_annullati: bool,
    #[serde(rename = "mostraIndisponibilitaTotali")]
    mostra_indisponibilita_totali: bool,
    linkCalendarioId: String,
    clienteId: String,
    pianificazioneTemplate: bool,
    auleIds: Vec<String>,
    limitaRisultati: bool,
    dataInizio: String,
    #[serde(rename = "dataFine")]
    data_fine: String,
}

fn get_aule_endpoint() -> String {
    format!("{}/api/Aule/getAulePerCalendarioPubblico", HOST)
}

fn get_calendario_endpoint() -> String {
    format!("{}/api/Impegni/getImpegniCalendarioPubblico", HOST)
}
